use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ignore link while building docs.
    if env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    // Make sure jvm has been linked.
    let java_home = env::var("JAVA_HOME")?;
    println!("cargo:rustc-link-search=native={java_home}/lib/server");
    println!("cargo:rustc-link-lib=jvm");

    // Static link compiled `libhdfs.a`
    println!("cargo:rustc-link-lib=static=hdfs");

    let mut builder = cc::Build::new();
    builder.warnings(false);
    builder.static_flag(true);
    builder.static_crt(true);

    {
        builder.include(format!("{java_home}/include"));
        builder.include(format!("{java_home}/include/linux"));
    }

    // Choose the latest version.
    let mut version = "hdfs_2_2";
    if cfg!(feature = "hdfs_2_3") {
        version = "hdfs_2_3"
    }
    if cfg!(feature = "hdfs_2_4") {
        version = "hdfs_2_4"
    }
    if cfg!(feature = "hdfs_2_5") {
        version = "hdfs_2_5"
    }
    if cfg!(feature = "hdfs_2_6") {
        version = "hdfs_2_6"
    }
    if cfg!(feature = "hdfs_2_7") {
        version = "hdfs_2_7"
    }
    if cfg!(feature = "hdfs_2_8") {
        version = "hdfs_2_8"
    }
    if cfg!(feature = "hdfs_2_9") {
        version = "hdfs_2_9"
    }
    if cfg!(feature = "hdfs_2_10") {
        version = "hdfs_2_10"
    }
    if cfg!(feature = "hdfs_3_0") {
        version = "hdfs_3_0"
    }
    if cfg!(feature = "hdfs_3_1") {
        version = "hdfs_3_1"
    }
    if cfg!(feature = "hdfs_3_2") {
        version = "hdfs_3_2"
    }
    if cfg!(feature = "hdfs_3_3") {
        version = "hdfs_3_3"
    }

    builder.include("libhdfs");
    builder.include(format!("libhdfs/{version}"));
    builder.file(format!("libhdfs/{version}/exception.c"));
    builder.file(format!("libhdfs/{version}/jni_helper.c"));
    builder.file(format!("libhdfs/{version}/hdfs.c"));

    // Since 2.6, we need to include mutexes.
    if cfg!(feature = "hdfs_2_6") {
        builder.include(format!("libhdfs/{version}/os"));

        if cfg!(os = "windows") {
            builder.include(format!("libhdfs/{version}/os/windows"));
            builder.file(format!("libhdfs/{version}/os/windows/mutexes.c"));
            builder.file(format!("libhdfs/{version}/os/windows/thread.c"));
            builder.file(format!(
                "libhdfs/{version}/os/windows/thread_local_storage.c"
            ));
        } else {
            builder.include(format!("libhdfs/{version}/os/posix"));
            builder.file(format!("libhdfs/{version}/os/posix/mutexes.c"));
            builder.file(format!("libhdfs/{version}/os/posix/thread.c"));
            builder.file(format!("libhdfs/{version}/os/posix/thread_local_storage.c"));
        }
    }

    // From 2.6 to 3.3, we need to include htable (removed in 3.3)
    if cfg!(feature = "hdfs_2_6") && !cfg!(feature = "hdfs_3_3") {
        builder.include(format!("libhdfs/{version}/common"));
        builder.file(format!("libhdfs/{version}/common/htable.c"));
    }

    // Since 2.8, `hdfs.h` has been moved to `include/hdfs/hdfs.h`
    if cfg!(feature = "hdfs_2_8") {
        builder.include(format!("libhdfs/{version}/include"));
    }

    // Since 3.3, we need to compile `jclasses.c`
    if cfg!(feature = "hdfs_3_3") {
        builder.file(format!("libhdfs/{version}/jclasses.c"));
    }

    builder.compile("hdfs");
    Ok(())
}