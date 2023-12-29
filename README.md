# first_rust
## Getting Started
需要先下载[protoc](https://github.com/hyperium/tonic?tab=readme-ov-file#dependencies)。

## Example
把数据放在目录data下,文件格式
```
1,2
2,3
...
```
文件名为1000w.csv

一个终端
```
cargo run --release --bin a
```
另一个终端
```
cargo run --release --bin b
```
