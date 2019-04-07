# slkv

### A simple lightweight top key-value type database.

[![Build Status](https://travis-ci.com/ltoddy/slkv.svg?branch=master)](https://travis-ci.com/ltoddy/slkv)

## Guide

<video width="320" height="240" controls>
  <source src="assets/guide.mkv" type="video/mkv">
</video>

## Communicate protocol

- `get` command: prefix: '*'
    e.g.: get foo bar:

    `server will received: '*foo bar'`

    `client will received: foo => bar\n!`

- `put` command: prefix: '+'
    e.g.: get foo bar:

    `server will received: '+foo bar'`

    `client will received: Ok\n!`

- `delete` command: prefix: '-'
    e.g.: get foo bar:

    `server will received: '-foo bar'`

    `client will received: Ok\n!`

- `scan` command: prefix: '/'
    e.g.: get foo bar:

    `server will received: '/foo bar'`

    `client will received: foo => bar\n!`

`!` indicates the end of transfer.


## Persistence protocol

Due to time, store file content liks:

```
key1 value1
key2 value2
...
```
