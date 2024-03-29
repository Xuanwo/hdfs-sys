# hdfs-sys &emsp; [![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/github/actions/workflow/status/Xuanwo/hdfs-sys/ci.yml
[actions]: https://github.com/Xuanwo/hdfs-sys/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/hdfs-sys.svg
[crates.io]: https://crates.io/crates/hdfs-sys

Bindings to `HDFS` Native C API.

Work with these bindings directly is boring and error proven, please use [hdrs](https://github.com/Xuanwo/hdrs) instead if ever possible.

## Supported Platforms

- Linux
- macOS
- Windows

## Supported Versions

To target a version of `libhdfs`, enable a Cargo features such as one of the following:

- `hdfs_2_2` : requires hdfs 2.2 or later releases.
- `hdfs_2_3`: requires hdfs 2.3 or later releases.
- `hdfs_2_4`: requires hdfs 2.4 or later releases.
- `hdfs_2_5`: requires hdfs 2.5 or later releases.
- `hdfs_2_6`: (**default**) requires hdfs 2.6 or later releases.
- `hdfs_2_7`: requires hdfs 2.7 or later releases.
- `hdfs_2_8`: requires hdfs 2.8 or later releases.
- `hdfs_2_9`: requires hdfs 2.9 or later releases.
- `hdfs_2_10`: requires hdfs 2.10 or later releases.
- `hdfs_3_0`: requires hdfs 3.0 or later releases.
- `hdfs_3_1`: requires hdfs 3.1 or later releases.
- `hdfs_3_2`: requires hdfs 3.2 or later releases.
- `hdfs_3_3`: requires hdfs 3.3 or later releases.

Please note:

- If you do not enable one of these features, the API provided by `hdfs_2_6` will be available by default.
- Enable one feature will also enable all features before it. For example, enable `hdfs_2_4` will also enable `hdfs_2_3` and `hdfs_2_2`.
- Too old version of hdfs could contain bugs or can't compile on your platform.

## Compile

`hdfs-sys` supports both dynamic link, static link and vendor:

- If `vnedored` feature has been enabled, `hdfs-sys` will compile and link `libhdfs` in static.
- Use `HDFS_LIB_DIR` to specify the path of `libhdfs.so` or `libhdfs.a`
- Use `HDFS_STATIC=1` to choose to switch between dynamic link and static link
- If `HDFS_LIB_DIR` is not set, we will try to find `${HADOOP_HOME}/lib/native`
- If all env are empty, we will try to compile libhdfs and link it in static

## Dependencies

This crate will link to `libjvm` dynamically.

To make this crate works correctly, please make sure the following env set correctly:

- `JAVA_HOME`: `hdfs-sys` will search path like `${JAVA_HOME}/lib/server` to link `libjvm`.

NOTE: `hdfs-sys` will ignore linking if `DOCS_RS` is set to build docs.

## Runtime

`hdfs-sys` uses JNI to call functions provided by jars that provided by hadoop releases. Please make sure `CLASSPATH` is set correctly before calling any functions provided by `hdfs-sys`:

```shell
export JAVA_HOME=/path/to/java
export HADOOP_HOME=/path/to/hadoop
export LD_LIBRARY_PATH=${JAVA_HOME}/lib/server
export CLASSPATH=$(find $HADOOP_HOME -iname "*.jar" | xargs echo | tr ' ' ':')
```

## Contributing

Check out the [CONTRIBUTING.md](./CONTRIBUTING.md) guide for more details on getting started with contributing to this project.

## Getting help

Submit [issues](https://github.com/Xuanwo/hdfs-sys/issues/new/choose) for bug report or asking questions in [discussion](https://github.com/Xuanwo/hdfs-sys/discussions/new?category=q-a).

## Related Projects

- [hdfs-rs](https://github.com/hyunsik/hdfs-rs/): libhdfs binding and wrapper APIs for Rust, not maintained since 2015.
- [fs-hdfs](https://github.com/yahoNanJing/fs-hdfs): forks based on [hdfs-rs](https://github.com/hyunsik/hdfs-rs/), C files are from hadoop `2.7.3`.
- [dataFusion-hdfs-native](https://github.com/datafusion-contrib/datafusion-hdfs-native): bindings of [libhdfs3](https://github.com/ClickHouse/libhdfs3/)

## Acknowledgment

This project is highly inspired by [clang-sys](https://github.com/KyleMayes/clang-sys)

#### License

<sup>
Licensed under <a href="./LICENSE">Apache License, Version 2.0</a>.
</sup>
