//! hdfs 2.3 added zero-copy read support.
//!
//! The diff between hdfs.h:
//!
//! ```diff
//! 38a39,40
//! > #define ELASTIC_BYTE_BUFFER_POOL_CLASS \
//! >   "org/apache/hadoop/io/ElasticByteBufferPool"
//! 67a70,73
//! >     struct hadoopRzOptions;
//! >
//! >     struct hadoopRzBuffer;
//! >
//! 87a94
//! >       uint64_t totalZeroCopyBytesRead;
//! 683c690,790
//! <
//! ---
//! >
//! >     /**
//! >      * Allocate a zero-copy options structure.
//! >      *
//! >      * You must free all options structures allocated with this function using
//! >      * hadoopRzOptionsFree.
//! >      *
//! >      * @return            A zero-copy options structure, or NULL if one could
//! >      *                    not be allocated.  If NULL is returned, errno will
//! >      *                    contain the error number.
//! >      */
//! >     struct hadoopRzOptions *hadoopRzOptionsAlloc(void);
//! >
//! >     /**
//! >      * Determine whether we should skip checksums in read0.
//! >      *
//! >      * @param opts        The options structure.
//! >      * @param skip        Nonzero to skip checksums sometimes; zero to always
//! >      *                    check them.
//! >      *
//! >      * @return            0 on success; -1 plus errno on failure.
//! >      */
//! >     int hadoopRzOptionsSetSkipChecksum(
//! >             struct hadoopRzOptions *opts, int skip);
//! >
//! >     /**
//! >      * Set the ByteBufferPool to use with read0.
//! >      *
//! >      * @param opts        The options structure.
//! >      * @param className   If this is NULL, we will not use any
//! >      *                    ByteBufferPool.  If this is non-NULL, it will be
//! >      *                    treated as the name of the pool class to use.
//! >      *                    For example, you can use
//! >      *                    ELASTIC_BYTE_BUFFER_POOL_CLASS.
//! >      *
//! >      * @return            0 if the ByteBufferPool class was found and
//! >      *                    instantiated;
//! >      *                    -1 plus errno otherwise.
//! >      */
//! >     int hadoopRzOptionsSetByteBufferPool(
//! >             struct hadoopRzOptions *opts, const char *className);
//! >
//! >     /**
//! >      * Free a hadoopRzOptionsFree structure.
//! >      *
//! >      * @param opts        The options structure to free.
//! >      *                    Any associated ByteBufferPool will also be freed.
//! >      */
//! >     void hadoopRzOptionsFree(struct hadoopRzOptions *opts);
//! >
//! >     /**
//! >      * Perform a byte buffer read.
//! >      * If possible, this will be a zero-copy (mmap) read.
//! >      *
//! >      * @param file       The file to read from.
//! >      * @param opts       An options structure created by hadoopRzOptionsAlloc.
//! >      * @param maxLength  The maximum length to read.  We may read fewer bytes
//! >      *                   than this length.
//! >      *
//! >      * @return           On success, returns a new hadoopRzBuffer.
//! >      *                   This buffer will continue to be valid and readable
//! >      *                   until it is released by readZeroBufferFree.  Failure to
//! >      *                   release a buffer will lead to a memory leak.
//! >      *
//! >      *                   NULL plus an errno code on an error.
//! >      *                   errno = EOPNOTSUPP indicates that we could not do a
//! >      *                   zero-copy read, and there was no ByteBufferPool
//! >      *                   supplied.
//! >      */
//! >     struct hadoopRzBuffer* hadoopReadZero(hdfsFile file,
//! >             struct hadoopRzOptions *opts, int32_t maxLength);
//! >
//! >     /**
//! >      * Determine the length of the buffer returned from readZero.
//! >      *
//! >      * @param buffer     a buffer returned from readZero.
//! >      * @return           the length of the buffer.
//! >      */
//! >     int32_t hadoopRzBufferLength(const struct hadoopRzBuffer *buffer);
//! >
//! >     /**
//! >      * Get a pointer to the raw buffer returned from readZero.
//! >      *
//! >      * To find out how many bytes this buffer contains, call
//! >      * hadoopRzBufferLength.
//! >      *
//! >      * @param buffer     a buffer returned from readZero.
//! >      * @return           a pointer to the start of the buffer.  This will be
//! >      *                   NULL when end-of-file has been reached.
//! >      */
//! >     const void *hadoopRzBufferGet(const struct hadoopRzBuffer *buffer);
//! >
//! >     /**
//! >      * Release a buffer obtained through readZero.
//! >      *
//! >      * @param file       The hdfs stream that created this buffer.  This must be
//! >      *                   the same stream you called hadoopReadZero on.
//! >      * @param buffer     The buffer to release.
//! >      */
//! >     void hadoopRzBufferFree(hdfsFile file, struct hadoopRzBuffer *buffer);
//! >
//! ```

use std::os::raw::*;

use crate::hdfsFile;

pub const ELASTIC_BYTE_BUFFER_POOL_CLASS: &[u8; 43usize] =
    b"org/apache/hadoop/io/ElasticByteBufferPool\0";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hadoopRzOptions {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hadoopRzBuffer {
    _unused: [u8; 0],
}

extern "C" {
    pub fn hadoopRzOptionsAlloc() -> *mut hadoopRzOptions;
    pub fn hadoopRzOptionsSetSkipChecksum(opts: *mut hadoopRzOptions, skip: c_int) -> c_int;
    pub fn hadoopRzOptionsSetByteBufferPool(
        opts: *mut hadoopRzOptions,
        className: *const c_char,
    ) -> c_int;
    pub fn hadoopRzOptionsFree(opts: *mut hadoopRzOptions);
    pub fn hadoopReadZero(
        file: hdfsFile,
        opts: *mut hadoopRzOptions,
        maxLength: i32,
    ) -> *mut hadoopRzBuffer;
    pub fn hadoopRzBufferLength(buffer: *const hadoopRzBuffer) -> i32;
    pub fn hadoopRzBufferGet(buffer: *const hadoopRzBuffer) -> *const c_void;
    pub fn hadoopRzBufferFree(file: hdfsFile, buffer: *mut hadoopRzBuffer);
}
