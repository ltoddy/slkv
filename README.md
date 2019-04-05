# slkv

### A simple lightweight top key-value type database.

[![Build Status](https://travis-ci.com/ltoddy/slkv.svg?branch=master)](https://travis-ci.com/ltoddy/slkv)

## Communicate protocol

- `get` command: prefix: '*'
    e.g.: get foo bar, server will received: '*foo bar'

- `put` command: prefix: '+'
    e.g.: get foo bar, server will received: '+foo bar'

- `delete` command: prefix: '-'
    e.g.: get foo bar, server will received: '-foo bar'

- `scan` command: prefix: '/'
    e.g.: get foo bar, server will received: '/foo bar'
