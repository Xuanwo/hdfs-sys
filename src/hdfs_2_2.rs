//! hdfs 2.2.0 is the first stable release of hadoop 2.
//!
//! We take this release as the baseline of hdfs-sys.

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
    #[cfg(feature = "hdfs_2_3")]
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
    pub fn hdfsFileIsOpenForRead(file: hdfsFile) -> c_int;
    pub fn hdfsFileIsOpenForWrite(file: hdfsFile) -> c_int;
    pub fn hdfsFileGetReadStatistics(file: hdfsFile, stats: *mut *mut hdfsReadStatistics) -> c_int;
    pub fn hdfsReadStatisticsGetRemoteBytesRead(stats: *const hdfsReadStatistics) -> i64;
    pub fn hdfsFileFreeReadStatistics(stats: *mut hdfsReadStatistics);
    pub fn hdfsConnectAsUser(nn: *const c_char, port: u16, user: *const c_char) -> hdfsFS;
    pub fn hdfsConnect(nn: *const c_char, port: u16) -> hdfsFS;
    pub fn hdfsConnectAsUserNewInstance(
        nn: *const c_char,
        port: u16,
        user: *const c_char,
    ) -> hdfsFS;
    pub fn hdfsConnectNewInstance(nn: *const c_char, port: u16) -> hdfsFS;
    pub fn hdfsBuilderConnect(bld: *mut hdfsBuilder) -> hdfsFS;
    pub fn hdfsNewBuilder() -> *mut hdfsBuilder;
    pub fn hdfsBuilderSetForceNewInstance(bld: *mut hdfsBuilder);
    pub fn hdfsBuilderSetNameNode(bld: *mut hdfsBuilder, nn: *const c_char);
    pub fn hdfsBuilderSetNameNodePort(bld: *mut hdfsBuilder, port: u16);
    pub fn hdfsBuilderSetUserName(bld: *mut hdfsBuilder, userName: *const c_char);
    pub fn hdfsBuilderSetKerbTicketCachePath(
        bld: *mut hdfsBuilder,
        kerbTicketCachePath: *const c_char,
    );
    pub fn hdfsFreeBuilder(bld: *mut hdfsBuilder);
    pub fn hdfsBuilderConfSetStr(
        bld: *mut hdfsBuilder,
        key: *const c_char,
        val: *const c_char,
    ) -> c_int;
    pub fn hdfsConfGetStr(key: *const c_char, val: *mut *mut c_char) -> c_int;
    pub fn hdfsConfGetInt(key: *const c_char, val: *mut i32) -> c_int;
    pub fn hdfsConfStrFree(val: *mut c_char);
    pub fn hdfsDisconnect(fs: hdfsFS) -> c_int;
    pub fn hdfsOpenFile(
        fs: hdfsFS,
        path: *const c_char,
        flags: c_int,
        bufferSize: c_int,
        replication: c_short,
        blocksize: i32,
    ) -> hdfsFile;

    pub fn hdfsCloseFile(fs: hdfsFS, file: hdfsFile) -> c_int;
    pub fn hdfsExists(fs: hdfsFS, path: *const c_char) -> c_int;
    pub fn hdfsSeek(fs: hdfsFS, file: hdfsFile, desiredPos: i64) -> c_int;
    pub fn hdfsTell(fs: hdfsFS, file: hdfsFile) -> i64;
    pub fn hdfsRead(fs: hdfsFS, file: hdfsFile, buffer: *mut c_void, length: i32) -> i32;
    pub fn hdfsPread(
        fs: hdfsFS,
        file: hdfsFile,
        position: i64,
        buffer: *mut c_void,
        length: i32,
    ) -> i32;
    pub fn hdfsWrite(fs: hdfsFS, file: hdfsFile, buffer: *const c_void, length: i32) -> i32;
    pub fn hdfsFlush(fs: hdfsFS, file: hdfsFile) -> c_int;
    pub fn hdfsHFlush(fs: hdfsFS, file: hdfsFile) -> c_int;
    pub fn hdfsHSync(fs: hdfsFS, file: hdfsFile) -> c_int;
    pub fn hdfsAvailable(fs: hdfsFS, file: hdfsFile) -> c_int;
    pub fn hdfsCopy(srcFS: hdfsFS, src: *const c_char, dstFS: hdfsFS, dst: *const c_char) -> c_int;
    pub fn hdfsMove(srcFS: hdfsFS, src: *const c_char, dstFS: hdfsFS, dst: *const c_char) -> c_int;
    pub fn hdfsDelete(fs: hdfsFS, path: *const c_char, recursive: c_int) -> c_int;
    pub fn hdfsRename(fs: hdfsFS, oldPath: *const c_char, newPath: *const c_char) -> c_int;
    pub fn hdfsGetWorkingDirectory(
        fs: hdfsFS,
        buffer: *mut c_char,
        bufferSize: c_ulong,
    ) -> *mut c_char;
    pub fn hdfsSetWorkingDirectory(fs: hdfsFS, path: *const c_char) -> c_int;
    pub fn hdfsCreateDirectory(fs: hdfsFS, path: *const c_char) -> c_int;
    pub fn hdfsSetReplication(fs: hdfsFS, path: *const c_char, replication: i16) -> c_int;
    pub fn hdfsListDirectory(
        fs: hdfsFS,
        path: *const c_char,
        numEntries: *mut c_int,
    ) -> *mut hdfsFileInfo;
    pub fn hdfsGetPathInfo(fs: hdfsFS, path: *const c_char) -> *mut hdfsFileInfo;
    pub fn hdfsFreeFileInfo(hdfsFileInfo: *mut hdfsFileInfo, numEntries: c_int);
    pub fn hdfsGetHosts(
        fs: hdfsFS,
        path: *const c_char,
        start: i64,
        length: i64,
    ) -> *mut *mut *mut c_char;
    pub fn hdfsFreeHosts(blockHosts: *mut *mut *mut c_char);
    pub fn hdfsGetDefaultBlockSize(fs: hdfsFS) -> i64;
    pub fn hdfsGetDefaultBlockSizeAtPath(fs: hdfsFS, path: *const c_char) -> i64;
    pub fn hdfsGetCapacity(fs: hdfsFS) -> i64;
    pub fn hdfsGetUsed(fs: hdfsFS) -> i64;
    pub fn hdfsChown(
        fs: hdfsFS,
        path: *const c_char,
        owner: *const c_char,
        group: *const c_char,
    ) -> c_int;
    pub fn hdfsChmod(fs: hdfsFS, path: *const c_char, mode: c_short) -> c_int;
    pub fn hdfsUtime(fs: hdfsFS, path: *const c_char, mtime: c_long, atime: c_long) -> c_int;
}
