//! hdfs 2.6 adds a new API hdfsFileIsEncrypted
//!
//! The diff between hdfs.h:
//!
//! ```diff
//! 594a595,602
//! >     /**
//! >      * hdfsFileIsEncrypted: determine if a file is encrypted based on its
//! >      * hdfsFileInfo.
//! >      * @return -1 if there was an error (errno will be set), 0 if the file is
//! >      *         not encrypted, 1 if the file is encrypted.
//! >      */
//! >     int hdfsFileIsEncrypted(hdfsFileInfo *hdfsFileInfo);
//! >
//! ```

use std::os::raw::*;

use crate::hdfsFileInfo;

extern "C" {
    pub fn hdfsFileIsEncrypted(hdfsFileInfo: *mut hdfsFileInfo) -> c_int;
}
