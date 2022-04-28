use super::*;

#[test]
#[cfg(feature = "hdfs_2_2")]
fn test_hdfs_abi_2_2() {
    let _ = hdfsFileIsOpenForRead;
    let _ = hdfsFileIsOpenForWrite;
    let _ = hdfsFileGetReadStatistics;
    let _ = hdfsReadStatisticsGetRemoteBytesRead;
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

#[test]
#[cfg(feature = "hdfs_2_3")]
fn test_hdfs_abi_2_3() {
    test_hdfs_abi_2_2();

    let _ = hadoopRzOptionsAlloc;
    let _ = hadoopRzOptionsSetSkipChecksum;
    let _ = hadoopRzOptionsSetByteBufferPool;
    let _ = hadoopRzOptionsFree;
    let _ = hadoopReadZero;
    let _ = hadoopRzBufferLength;
    let _ = hadoopRzBufferGet;
    let _ = hadoopRzBufferFree;
}

#[test]
#[cfg(feature = "hdfs_2_4")]
fn test_hdfs_abi_2_4() {
    test_hdfs_abi_2_3();
}

#[test]
#[cfg(feature = "hdfs_2_5")]
fn test_hdfs_abi_2_5() {
    test_hdfs_abi_2_4();
}

#[test]
#[cfg(feature = "hdfs_2_6")]
fn test_hdfs_abi_2_6() {
    test_hdfs_abi_2_5();

    let _ = hdfsFileIsEncrypted;
}

#[test]
#[cfg(feature = "hdfs_2_7")]
fn test_hdfs_abi_2_7() {
    test_hdfs_abi_2_6();

    let _ = hdfsFileClearReadStatistics;
    let _ = hdfsTruncateFile;
    let _ = hdfsUnbufferFile;
}

#[test]
#[cfg(feature = "hdfs_2_8")]
fn test_hdfs_abi_2_8() {
    test_hdfs_abi_2_7();
}

#[test]
#[cfg(feature = "hdfs_2_9")]
fn test_hdfs_abi_2_9() {
    test_hdfs_abi_2_8();

    let _ = hdfsStreamBuilderAlloc;
    let _ = hdfsStreamBuilderFree;
    let _ = hdfsStreamBuilderSetBufferSize;
    let _ = hdfsStreamBuilderSetReplication;
    let _ = hdfsStreamBuilderSetDefaultBlockSize;
    let _ = hdfsStreamBuilderBuild;
}

#[test]
#[cfg(feature = "hdfs_2_10")]
fn test_hdfs_abi_2_10() {
    test_hdfs_abi_2_9();

    let _ = hdfsStreamBuilderAlloc;
    let _ = hdfsStreamBuilderFree;
    let _ = hdfsStreamBuilderSetBufferSize;
    let _ = hdfsStreamBuilderSetReplication;
    let _ = hdfsStreamBuilderSetDefaultBlockSize;
    let _ = hdfsStreamBuilderBuild;
}

#[test]
#[cfg(feature = "hdfs_3_0")]
fn test_hdfs_abi_3_0() {
    test_hdfs_abi_2_10();

    let _ = hdfsGetHedgedReadMetrics;
    let _ = hdfsFreeHedgedReadMetrics;
    let _ = hdfsGetLastExceptionRootCause;
    let _ = hdfsGetLastExceptionStackTrace;
}

#[test]
#[cfg(feature = "hdfs_3_1")]
fn test_hdfs_abi_3_1() {
    test_hdfs_abi_3_0();
}

#[test]
#[cfg(feature = "hdfs_3_2")]
fn test_hdfs_abi_3_2() {
    test_hdfs_abi_3_1();
}
#[test]
#[cfg(feature = "hdfs_3_3")]
fn test_hdfs_abi_3_3() {
    test_hdfs_abi_3_2();

    let _ = hdfsPreadFully;
}
