#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hdfsBuilder {
    _unused: [u8; 0],
}

pub type tObjectKind = c_uint;
pub const tObjectKind_kObjectKindFile: tObjectKind = 70;
pub const tObjectKind_kObjectKindDirectory: tObjectKind = 68;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hdfsStreamBuilder {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hdfs_internal {
    _unused: [u8; 0],
}
pub type hdfsFS = *mut hdfs_internal;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hdfsFile_internal {
    _unused: [u8; 0],
}
pub type hdfsFile = *mut hdfsFile_internal;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hdfsReadStatistics {
    pub totalBytesRead: u64,
    pub totalLocalBytesRead: u64,
    pub totalShortCircuitBytesRead: u64,
    pub totalZeroCopyBytesRead: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hdfsFileInfo {
    pub mKind: tObjectKind,
    pub mName: *mut c_char,
    pub mLastMod: c_long,
    pub mSize: i64,
    pub mReplication: c_short,
    pub mBlockSize: i64,
    pub mOwner: *mut c_char,
    pub mGroup: *mut c_char,
    pub mPermissions: c_short,
    pub mLastAccess: c_long,
}

extern "C" {
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsFileIsOpenForRead(file: hdfsFile) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsFileIsOpenForWrite(file: hdfsFile) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsFileGetReadStatistics(file: hdfsFile, stats: *mut *mut hdfsReadStatistics) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsReadStatisticsGetRemoteBytesRead(stats: *const hdfsReadStatistics) -> i64;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsFileClearReadStatistics(file: hdfsFile) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsFileFreeReadStatistics(stats: *mut hdfsReadStatistics);
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsConnectAsUser(nn: *const c_char, port: u16, user: *const c_char) -> hdfsFS;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsConnect(nn: *const c_char, port: u16) -> hdfsFS;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsConnectAsUserNewInstance(
        nn: *const c_char,
        port: u16,
        user: *const c_char,
    ) -> hdfsFS;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsConnectNewInstance(nn: *const c_char, port: u16) -> hdfsFS;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsBuilderConnect(bld: *mut hdfsBuilder) -> hdfsFS;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsNewBuilder() -> *mut hdfsBuilder;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsBuilderSetForceNewInstance(bld: *mut hdfsBuilder);
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsBuilderSetNameNode(bld: *mut hdfsBuilder, nn: *const c_char);
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsBuilderSetNameNodePort(bld: *mut hdfsBuilder, port: u16);
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsBuilderSetUserName(bld: *mut hdfsBuilder, userName: *const c_char);
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsBuilderSetKerbTicketCachePath(
        bld: *mut hdfsBuilder,
        kerbTicketCachePath: *const c_char,
    );
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsFreeBuilder(bld: *mut hdfsBuilder);
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsBuilderConfSetStr(
        bld: *mut hdfsBuilder,
        key: *const c_char,
        val: *const c_char,
    ) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsConfGetStr(key: *const c_char, val: *mut *mut c_char) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsConfGetInt(key: *const c_char, val: *mut i32) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsConfStrFree(val: *mut c_char);
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsDisconnect(fs: hdfsFS) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsOpenFile(
        fs: hdfsFS,
        path: *const c_char,
        flags: c_int,
        bufferSize: c_int,
        replication: c_short,
        blocksize: i32,
    ) -> hdfsFile;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsStreamBuilderAlloc(
        fs: hdfsFS,
        path: *const c_char,
        flags: c_int,
    ) -> *mut hdfsStreamBuilder;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsStreamBuilderFree(bld: *mut hdfsStreamBuilder);
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsStreamBuilderSetBufferSize(bld: *mut hdfsStreamBuilder, bufferSize: i32) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsStreamBuilderSetReplication(bld: *mut hdfsStreamBuilder, replication: i16) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsStreamBuilderSetDefaultBlockSize(
        bld: *mut hdfsStreamBuilder,
        defaultBlockSize: i64,
    ) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsStreamBuilderBuild(bld: *mut hdfsStreamBuilder) -> hdfsFile;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsTruncateFile(fs: hdfsFS, path: *const c_char, newlength: i64) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsUnbufferFile(file: hdfsFile) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsCloseFile(fs: hdfsFS, file: hdfsFile) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsExists(fs: hdfsFS, path: *const c_char) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsSeek(fs: hdfsFS, file: hdfsFile, desiredPos: i64) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsTell(fs: hdfsFS, file: hdfsFile) -> i64;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsRead(fs: hdfsFS, file: hdfsFile, buffer: *mut c_void, length: i32) -> i32;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsPread(
        fs: hdfsFS,
        file: hdfsFile,
        position: i64,
        buffer: *mut c_void,
        length: i32,
    ) -> i32;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsWrite(fs: hdfsFS, file: hdfsFile, buffer: *const c_void, length: i32) -> i32;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsFlush(fs: hdfsFS, file: hdfsFile) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsHFlush(fs: hdfsFS, file: hdfsFile) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsHSync(fs: hdfsFS, file: hdfsFile) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsAvailable(fs: hdfsFS, file: hdfsFile) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsCopy(srcFS: hdfsFS, src: *const c_char, dstFS: hdfsFS, dst: *const c_char) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsMove(srcFS: hdfsFS, src: *const c_char, dstFS: hdfsFS, dst: *const c_char) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsDelete(fs: hdfsFS, path: *const c_char, recursive: c_int) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsRename(fs: hdfsFS, oldPath: *const c_char, newPath: *const c_char) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsGetWorkingDirectory(
        fs: hdfsFS,
        buffer: *mut c_char,
        bufferSize: c_ulong,
    ) -> *mut c_char;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsSetWorkingDirectory(fs: hdfsFS, path: *const c_char) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsCreateDirectory(fs: hdfsFS, path: *const c_char) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsSetReplication(fs: hdfsFS, path: *const c_char, replication: i16) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsListDirectory(
        fs: hdfsFS,
        path: *const c_char,
        numEntries: *mut c_int,
    ) -> *mut hdfsFileInfo;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsGetPathInfo(fs: hdfsFS, path: *const c_char) -> *mut hdfsFileInfo;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsFreeFileInfo(hdfsFileInfo: *mut hdfsFileInfo, numEntries: c_int);
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsFileIsEncrypted(hdfsFileInfo: *mut hdfsFileInfo) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsGetHosts(
        fs: hdfsFS,
        path: *const c_char,
        start: i64,
        length: i64,
    ) -> *mut *mut *mut c_char;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsFreeHosts(blockHosts: *mut *mut *mut c_char);
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsGetDefaultBlockSize(fs: hdfsFS) -> i64;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsGetDefaultBlockSizeAtPath(fs: hdfsFS, path: *const c_char) -> i64;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsGetCapacity(fs: hdfsFS) -> i64;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsGetUsed(fs: hdfsFS) -> i64;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsChown(
        fs: hdfsFS,
        path: *const c_char,
        owner: *const c_char,
        group: *const c_char,
    ) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsChmod(fs: hdfsFS, path: *const c_char, mode: c_short) -> c_int;
    #[cfg(feature = "hdfs_2_10_1")]
    pub fn hdfsUtime(fs: hdfsFS, path: *const c_char, mtime: c_long, atime: c_long) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "hdfs_2_10_1")]
    fn test_hdfs_abi_2_10_1() {
        let _ = hdfsFileIsOpenForRead;
        let _ = hdfsFileIsOpenForWrite;
        let _ = hdfsFileGetReadStatistics;
        let _ = hdfsReadStatisticsGetRemoteBytesRead;
        let _ = hdfsFileClearReadStatistics;
        let _ = hdfsFileFreeReadStatistics;
        let _ = hdfsConnectAsUser;
        let _ = hdfsConnect;
        let _ = hdfsConnectAsUserNewInstance;
        let _ = hdfsConnectNewInstance;
        let _ = hdfsBuilderConnect;
        let _ = hdfsNewBuilder;
        let _ = hdfsBuilderSetForceNewInstance;
        let _ = hdfsBuilderSetNameNode;
        let _ = hdfsBuilderSetNameNodePort;
        let _ = hdfsBuilderSetUserName;
        let _ = hdfsBuilderSetKerbTicketCachePath;
        let _ = hdfsFreeBuilder;
        let _ = hdfsBuilderConfSetStr;
        let _ = hdfsConfGetStr;
        let _ = hdfsConfGetInt;
        let _ = hdfsConfStrFree;
        let _ = hdfsDisconnect;
        let _ = hdfsOpenFile;
        let _ = hdfsStreamBuilderAlloc;
        let _ = hdfsStreamBuilderFree;
        let _ = hdfsStreamBuilderSetBufferSize;
        let _ = hdfsStreamBuilderSetReplication;
        let _ = hdfsStreamBuilderSetDefaultBlockSize;
        let _ = hdfsStreamBuilderBuild;
        let _ = hdfsTruncateFile;
        let _ = hdfsUnbufferFile;
        let _ = hdfsCloseFile;
        let _ = hdfsExists;
        let _ = hdfsSeek;
        let _ = hdfsTell;
        let _ = hdfsRead;
        let _ = hdfsPread;
        let _ = hdfsWrite;
        let _ = hdfsFlush;
        let _ = hdfsHFlush;
        let _ = hdfsHSync;
        let _ = hdfsAvailable;
        let _ = hdfsCopy;
        let _ = hdfsMove;
        let _ = hdfsDelete;
        let _ = hdfsRename;
        let _ = hdfsGetWorkingDirectory;
        let _ = hdfsSetWorkingDirectory;
        let _ = hdfsCreateDirectory;
        let _ = hdfsSetReplication;
        let _ = hdfsListDirectory;
        let _ = hdfsGetPathInfo;
        let _ = hdfsFreeFileInfo;
        let _ = hdfsFileIsEncrypted;
        let _ = hdfsGetHosts;
        let _ = hdfsFreeHosts;
        let _ = hdfsGetDefaultBlockSize;
        let _ = hdfsGetDefaultBlockSizeAtPath;
        let _ = hdfsGetCapacity;
        let _ = hdfsGetUsed;
        let _ = hdfsChown;
        let _ = hdfsChmod;
        let _ = hdfsUtime;
    }
}
