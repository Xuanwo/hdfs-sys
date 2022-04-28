//! hdfs 3.0 is the first stable release of hadoop 3.
//!
//! This release adds new APIs:
//!
//! - New Struct: hdfsHedgedReadMetrics
//! - hdfsGetHedgedReadMetrics
//! - hdfsFreeHedgedReadMetrics
//! - hdfsGetLastExceptionRootCause
//! - hdfsGetLastExceptionStackTrace

use std::os::raw::*;

use crate::hdfsFS;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hdfsHedgedReadMetrics {
    pub hedgedReadOps: u64,
    pub hedgedReadOpsWin: u64,
    pub hedgedReadOpsInCurThread: u64,
}

extern "C" {
    pub fn hdfsGetHedgedReadMetrics(fs: hdfsFS, metrics: *mut *mut hdfsHedgedReadMetrics) -> c_int;
    pub fn hdfsFreeHedgedReadMetrics(metrics: *mut hdfsHedgedReadMetrics);
    pub fn hdfsGetLastExceptionRootCause() -> *mut c_char;
    pub fn hdfsGetLastExceptionStackTrace() -> *mut c_char;
}
