use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    // Ignore link while building docs.
    if env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    find_jvm()?;

    let found = if cfg!(feature = "vendored") {
        false
    } else {
        find_libhdfs()?
    };
    if !found {
        build_libhdfs()?;
    }

    Ok(())
}

fn find_jvm() -> Result<()> {
    let jvm_path = java_locator::locate_jvm_dyn_library()?;

    println!("cargo:rustc-link-lib=jvm");
    println!("cargo:rustc-link-search=native={jvm_path}");

    // Add JVM to rpath
    println!("cargo:rustc-link-arg=-Wl,-rpath,{jvm_path}");

    // Export the used JVM_PATH as metadata, in case a crate needs it in order to link
    println!("cargo:metadata=JVM_PATH={jvm_path}");

    // Add jvm.lib into search path for windows.
    #[cfg(windows)]
    if let Ok(jvm_lib_path) = java_locator::locate_file("jvm.lib") {
        println!("cargo:rustc-link-search=native={jvm_lib_path}");
    }

    Ok(())
}

/// Find libhdfs
///
/// Return `true` if libhdfs is found, else `false`.
///
/// - Check `HDFS_LIB_DIR` first, then `HADOOP_HOME`.
/// - If `HDFS_STATIC` is set, link statically, otherwise, dynamic.
fn find_libhdfs() -> Result<bool> {
    // rerun if hdfs related env changed
    println!("cargo:rerun-if-env-changed=HDFS_LIB_DIR");
    println!("cargo:rerun-if-env-changed=HDFS_STATIC");
    println!("cargo:rerun-if-env-changed=HADOOP_HOME");

    let lib_dir = if let Ok(lib_dir) = env::var("HDFS_LIB_DIR") {
        lib_dir
    } else if let Ok(hadoop_home) = env::var("HADOOP_HOME") {
        format!("{hadoop_home}/lib/native")
    } else {
        return Ok(false);
    };

    println!("cargo:rustc-link-search=native={lib_dir}");
    let mode = match env::var_os("HDFS_STATIC") {
        Some(_) => "static",
        None => "dylib",
    };
    println!("cargo:rustc-link-lib={mode}=hdfs");

    Ok(true)
}

fn build_libhdfs() -> Result<()> {
    let java_home = java_locator::locate_java_home()?;

    // Static link compiled `libhdfs.a`
    println!("cargo:rustc-link-lib=static=hdfs");

    let mut builder = cc::Build::new();
    builder.warnings(false);

    // This flag does not work on windows, just throws warnings
    #[cfg(not(windows))]
    builder.static_flag(true);

    builder.static_crt(true);

    // Ignore all warnings from cc as we don't care about code written by Apache Hadoop.
    builder.flag_if_supported("-w");
    builder.flag_if_supported("-std=c++17");

    // Inspired by [hadoop-hdfs-native-client/src/CMakeLists.txt](https://github.com/apache/hadoop/blob/trunk/hadoop-hdfs-project/hadoop-hdfs-native-client/src/CMakeLists.txt)
    if cfg!(windows) {
        // Set the optimizer level.
        builder.flag("-O2");
        // Set warning level 4.
        builder.flag("/W4");
        // Skip "unreferenced formal parameter".
        builder.flag("/wd4100");
        // Skip "conditional expression is constant".
        builder.flag("/wd4127");
        // Skip deprecated POSIX function warnings.
        builder.flag("-D_CRT_NONSTDC_NO_DEPRECATE");
        // Skip CRT non-secure function warnings.  If we can convert usage of
        // strerror, getenv and ctime to their secure CRT equivalents, then we can
        // re-enable the CRT non-secure function warnings.
        builder.flag("-D_CRT_SECURE_NO_WARNINGS");
        // Omit unneeded headers.
        builder.flag("-DWIN32_LEAN_AND_MEAN");
    } else {
        builder.flag("-fvisibility=hidden");
        // using old default behavior on GCC >= 10.0
        builder.flag("-fcommon");
    }

    // Handle java headers.
    builder.include(format!("{java_home}/include"));
    if cfg!(target_os = "linux") {
        builder.include(format!("{java_home}/include/linux"));
    }
    if cfg!(target_os = "macos") {
        builder.include(format!("{java_home}/include/darwin"));
    }
    if cfg!(target_os = "windows") {
        builder.include(format!("{java_home}/include/win32"));
    }

    // Choose the latest hdfs version.
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
    // Since 2.6, hdfs supports windows.
    //
    // We build with src from `hdfs_2_6` but expose earlier ABI like `hdfs_2_2`.
    // This simple trick makes hdfs-sys works on windows without breaking our ABI promise.
    if cfg!(target_os = "windows") {
        version = "hdfs_2_6"
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
    if cfg!(feature = "hdfs_2_6") || cfg!(target_os = "windows") {
        builder.include(format!("libhdfs/{version}/os"));

        if cfg!(target_os = "windows") {
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

        // Since 3.3, windows will need to link `dirent`
        if cfg!(target_os = "windows") {
            builder.include("libdirent/include");
        }
    }

    #[cfg(not(feature = "vendored"))]
    {
        println!("cargo:warning=Building libhdfs from source as a fallback, \
        if you are encountering issues with missing headers on JDK8, consider enabling the `vendored` feature.");
    }

    builder.compile("hdfs");
    Ok(())
}
