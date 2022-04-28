//! hdfs 2.9 adds hdfsStreamBuilder related APIs.
//!
//! - New struct: hdfsStreamBuilder
//! - hdfsStreamBuilder
//! - hdfsStreamBuilderFree
//! - hdfsStreamBuilderSetBufferSize
//! - hdfsStreamBuilderSetReplication
//! - hdfsStreamBuilderSetDefaultBlockSize
//! - hdfsStreamBuilderBuild

use std::os::raw::*;

use crate::{hdfsFS, hdfsFile};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hdfsStreamBuilder {
    _unused: [u8; 0],
}

extern "C" {
    pub fn hdfsStreamBuilderAlloc(
        fs: hdfsFS,
        path: *const c_char,
        flags: c_int,
    ) -> *mut hdfsStreamBuilder;
    pub fn hdfsStreamBuilderFree(bld: *mut hdfsStreamBuilder);
    pub fn hdfsStreamBuilderSetBufferSize(bld: *mut hdfsStreamBuilder, bufferSize: i32) -> c_int;
    pub fn hdfsStreamBuilderSetReplication(bld: *mut hdfsStreamBuilder, replication: i16) -> c_int;
    pub fn hdfsStreamBuilderSetDefaultBlockSize(
        bld: *mut hdfsStreamBuilder,
        defaultBlockSize: i64,
    ) -> c_int;
    pub fn hdfsStreamBuilderBuild(bld: *mut hdfsStreamBuilder) -> hdfsFile;
}
