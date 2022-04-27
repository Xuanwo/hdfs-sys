extern crate bindgen;

use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    // Ignore linking libs if we are building docs.
    if env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    // Link to libs: dynamics, static, runtime

    // Make sure jvm has been linked.
    let java_home = env::var("JAVA_HOME")?;
    println!("cargo:rustc-link-search=native={java_home}/lib/server");
    println!("cargo:rustc-link-lib=jvm");

    let hadoop_home = env::var("HADOOP_HOME")?;
    println!("cargo:rustc-link-search=native={hadoop_home}/lib/native");
    println!("cargo:rustc-link-lib=hdfs");
    Ok(())
}
