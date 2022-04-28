//! hdfs 3.3 add new APIs:
//!
//! - hdfsPreadFully

use std::os::raw::*;

use crate::{hdfsFS, hdfsFile};

extern "C" {
    pub fn hdfsPreadFully(
        fs: hdfsFS,
        file: hdfsFile,
        position: i64,
        buffer: *mut c_void,
        length: i32,
    ) -> c_int;
}
