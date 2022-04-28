# hdfs-sys &emsp; [![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/github/workflow/status/Xuanwo/hdfs-sys/CI/main
[actions]: https://github.com/Xuanwo/hdfs-sys/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/hdfs-sys.svg
[crates.io]: https://crates.io/crates/hdfs-sys

Bindings to `HDFS` Native C API.

Work with these bindings directly is boring and error proven, please use [hdrs](https://github.com/Xuanwo/hdrs) instead if ever possible.

## Supported Versions

To target a version of `libhdfs`, enable a Cargo features such as one of the following:

- `hdfs_2_10_1` (**default**): requires hdfs 2.10.1 or later releases.
- `hdfs_3_2_3`: requires hdfs 3.2.3 or later releases.
- `hdfs_3_3_2`: requires hdfs 3.3.2 or later releases.

If you do not enable one of these features, the API provided by `hdfs_2_10_1` will be available by default.

## Dependencies

This crate will link to `libhdfs` and `libjvm` dynamically.

To make this crate works correctly, please make sure the following env set correctly:

- `JAVA_HOME`: `hdfs-sys` will search `${JAVA_HOME}/lib/server` to link `libjvm`.
- `HADOOP_HOME`: `hdfs-sys` will search `${HADOOP_HOME}/lib/native` to link `libhdfs`.

NOTE: `hdfs-sys` will ignore linking if `DOCS_RS` is set to build docs.

## Runtime

`libhdfs` uses JNI to call functions provided by jars that provided by hadoop releases. Please make sure `CLASSPATH` is set correctly:

```shell
export JAVA_HOME=/path/to/java
export HADOOP_HOME=/path/to/hadoop
export LD_LIBRARY_PATH=${HADOOP_HOME}/lib/native:${JAVA_HOME}/lib/server
export CLASSPATH=${HADOOP_HOME}/share/hadoop/common/*:${HADOOP_HOME}/share/hadoop/common/lib/*:${HADOOP_HOME}/share/hadoop/hdfs/*:${HADOOP_HOME}/share/hadoop/hdfs/lib/*:${HADOOP_HOME}/etc/hadoop/*
```

## Contributing

Check out the [CONTRIBUTING.md](./CONTRIBUTING.md) guide for more details on getting started with contributing to this project.

## Getting help

Submit [issues](https://github.com/Xuanwo/hdfs-sys/issues/new/choose) for bug report or asking questions in [discussion](https://github.com/Xuanwo/hdfs-sys/discussions/new?category=q-a).

## Acknowledgment

This project is highly inspired by [clang-sys](https://github.com/KyleMayes/clang-sys)

#### License

<sup>
Licensed under <a href="./LICENSE">Apache License, Version 2.0</a>.
</sup>
