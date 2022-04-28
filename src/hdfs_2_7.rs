//! hdfs 2.7 adds new APIs:
//!
//! - hdfsFileClearReadStatistics
//! - hdfsTruncateFile
//! - hdfsUnbufferFile

use std::os::raw::*;

use crate::{hdfsFS, hdfsFile};

extern "C" {
    pub fn hdfsFileClearReadStatistics(file: hdfsFile) -> c_int;
    pub fn hdfsTruncateFile(fs: hdfsFS, path: *const c_char, newlength: i64) -> c_int;
    pub fn hdfsUnbufferFile(file: hdfsFile) -> c_int;
}
