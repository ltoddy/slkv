# slkv

### A simple lightweight top key-value type database.

[![Build Status](https://travis-ci.com/ltoddy/slkv.svg?branch=master)](https://travis-ci.com/ltoddy/slkv)

题目
KV Server
编写一个 C/S Demo。服务器上存储有一个有序的 Key-Value 集合（类似于 c++ 中的 std::map<string, string>）。
要求：
1. 需要实现 Get，Put，Delete 和 Scan 接口

2. Key 固定 8 字节，value 固定 256 字节

3.（加分项）需要能对内存数据进行持久化保存，重启之后数据不丢失

提示：

1. 编程语言请使用 c/c++/go/rust。

2. 设计协议时要考虑到返回的数据量可能会很大。

3. 注意代码风格，添加必要的单元测试和文档。

4. 注意异常处理，尝试优化性能。


## More

前后端协议:

- get: '*'

- put: '+'

- delete: '-'

- scan: '/'


## Communicate protocol

- `get` command: prefix: '*'
    e.g.: get foo bar, server will received: '*foo bar'
