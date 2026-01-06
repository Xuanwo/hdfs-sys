//! Bindings to `HDFS` Native C API.
//!
//! Work with these bindings directly is boring and error proven, please use [hdrs](https://github.com/Xuanwo/hdrs) instead if ever possible.
//!
//! ## Supported Versions
//!
//! To target a version of `libhdfs`, enable a Cargo features such as one of the following:
//!
//! - `hdfs_2_2` (**default**): requires hdfs 2.2 or later releases.
//! - `hdfs_2_3`: requires hdfs 2.3 or later releases.
//! - `hdfs_2_4`: requires hdfs 2.4 or later releases.
//! - `hdfs_2_5`: requires hdfs 2.5 or later releases.
//! - `hdfs_2_6`: requires hdfs 2.6 or later releases.
//! - `hdfs_2_7`: requires hdfs 2.7 or later releases.
//! - `hdfs_2_8`: requires hdfs 2.8 or later releases.
//! - `hdfs_2_9`: requires hdfs 2.9 or later releases.
//! - `hdfs_2_10`: requires hdfs 2.10 or later releases.
//! - `hdfs_3_0`: requires hdfs 3.0 or later releases.
//! - `hdfs_3_1`: requires hdfs 3.1 or later releases.
//! - `hdfs_3_2`: requires hdfs 3.2 or later releases.
//! - `hdfs_3_3`: requires hdfs 3.3 or later releases.
//!
//! If you do not enable one of these features, the API provided by `hdfs_2_2` will be available by default.
//!
//! Enable one feature will also enable all features before it. For example, enable `hdfs_2_4` will also enable `hdfs_2_3` and `hdfs_2_2`.
//!
//! ## Dependencies
//!
//! This crate will link to `libhdfs` and `libjvm` dynamically.
//!
//! To make this crate works correctly, please make sure the following env set correctly:
//!
//! - `JAVA_HOME`: `hdfs-sys` will search `${JAVA_HOME}/lib/server` to link `libjvm`.
//! - `HADOOP_HOME`: `hdfs-sys` will search `{HADOOP_HOME}/lib/native` to link `libhdfs`.
//!
//! NOTE: `hdfs-sys` will ignore linking if `DOCS_RS` is set to build docs.
//!
//! ## Runtime
//!
//! `libhdfs` uses JNI to call functions provided by jars that provided by hadoop releases. Please make sure `CLASSPATH` is set correctly:
//!
//! ```shell
//! export JAVA_HOME=/path/to/java
//! export HADOOP_HOME=/path/to/hadoop
//! export LD_LIBRARY_PATH=${HADOOP_HOME}/lib/native:${JAVA_HOME}/lib/server
//! export CLASSPATH=${HADOOP_HOME}/share/hadoop/common/*:${HADOOP_HOME}/share/hadoop/common/lib/*:${HADOOP_HOME}/share/hadoop/hdfs/*:${HADOOP_HOME}/share/hadoop/hdfs/lib/*:${HADOOP_HOME}/etc/hadoop/*
//! ```

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

#[cfg(feature = "hdfs_2_2")]
mod hdfs_2_2;
#[cfg(feature = "hdfs_2_2")]
pub use hdfs_2_2::*;
#[cfg(feature = "hdfs_2_3")]
mod hdfs_2_3;
#[cfg(feature = "hdfs_2_3")]
pub use hdfs_2_3::*;
#[cfg(feature = "hdfs_2_4")]
mod hdfs_2_4;
#[cfg(feature = "hdfs_2_4")]
#[allow(unused_imports)]
pub use hdfs_2_4::*;
#[cfg(feature = "hdfs_2_5")]
mod hdfs_2_5;
#[cfg(feature = "hdfs_2_5")]
#[allow(unused_imports)]
pub use hdfs_2_5::*;
#[cfg(feature = "hdfs_2_6")]
mod hdfs_2_6;
#[cfg(feature = "hdfs_2_6")]
pub use hdfs_2_6::*;
#[cfg(feature = "hdfs_2_7")]
mod hdfs_2_7;
#[cfg(feature = "hdfs_2_7")]
pub use hdfs_2_7::*;
#[cfg(feature = "hdfs_2_8")]
mod hdfs_2_8;
#[cfg(feature = "hdfs_2_8")]
#[allow(unused_imports)]
pub use hdfs_2_8::*;
#[cfg(feature = "hdfs_2_9")]
mod hdfs_2_9;
#[cfg(feature = "hdfs_2_9")]
pub use hdfs_2_9::*;
#[cfg(feature = "hdfs_2_10")]
mod hdfs_2_10;
#[cfg(feature = "hdfs_2_10")]
#[allow(unused_imports)]
pub use hdfs_2_10::*;
#[cfg(feature = "hdfs_3_0")]
mod hdfs_3_0;
#[cfg(feature = "hdfs_3_0")]
pub use hdfs_3_0::*;
#[cfg(feature = "hdfs_3_1")]
mod hdfs_3_1;
#[cfg(feature = "hdfs_3_1")]
#[allow(unused_imports)]
pub use hdfs_3_1::*;
#[cfg(feature = "hdfs_3_2")]
mod hdfs_3_2;
#[cfg(feature = "hdfs_3_2")]
#[allow(unused_imports)]
pub use hdfs_3_2::*;
#[cfg(feature = "hdfs_3_3")]
mod hdfs_3_3;
#[cfg(feature = "hdfs_3_3")]
pub use hdfs_3_3::*;

#[cfg(test)]
mod tests;
