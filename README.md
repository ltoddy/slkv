# slkv

### A simple lightweight top key-value type database.

[![Build Status](https://travis-ci.com/ltoddy/slkv.svg?branch=master)](https://travis-ci.com/ltoddy/slkv)

## Guide

[![youtube](https://img.youtube.com/vi/stotBH8TCXY/0.jpg)](https://www.youtube.com/watch?v=stotBH8TCXY)

## 通信

- `get` 命令: 前缀: '*'

    e.g.: get foo bar:

    `服务端会收到: '*foo bar'`

    `客户端将会得到响应: foo => bar\n!`

- `put` 命令: 前缀: '+'

    e.g.: put foo bar:

    `服务端会收到: '+foo bar'`

    `客户端将会得到响应: Ok\n!`

- `delete` 命令: 前缀: '-'

    e.g.: delete foo bar:

    `服务端会收到: '-foo bar'`

    `客户端将会得到响应: Ok\n!`

- `scan` 命令: 前缀: '/'

    e.g.: scan 1 10:

    `服务端会收到: '/1 10'`

    `客户端将会得到响应: foo => bar\n!`

## 持久化

由于key与value都是String类型,目前来看,直接把字符串存到文件中是最简便的.

如下:

```
key1 value1
key2 value2
...
```
