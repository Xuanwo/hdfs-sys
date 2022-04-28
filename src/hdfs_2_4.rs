//! hdfs 2.4 added no API changes.
//!
//! The diff between hdfs.h:
//!
//! ```diff
//! 749c749
//! <      * @return           On success, returns a new hadoopRzBuffer.
//! ---
//! >      * @return           On success, we will return a new hadoopRzBuffer.
//! 752a753,756
//! >      *                   You can access the data within the hadoopRzBuffer with
//! >      *                   hadoopRzBufferGet.  If you have reached EOF, the data
//! >      *                   within the hadoopRzBuffer will be NULL.  You must still
//! >      *                   free hadoopRzBuffer instances containing NULL.
//! 754c758
//! <      *                   NULL plus an errno code on an error.
//! ---
//! >      *                   On failure, we will return NULL plus an errno code.
//! ```
