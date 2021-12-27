
# About

FOR LEARNING PURPOSES ONLY

This is a greatly simplified implementation of libcore with an implementation of arithmetic, pointer arithmetic, `?` operator and rt to run Hello World on Linux x86-64.

# Hello world

There are two hello worlds in this project: a freestanding and with a libcore. You can run them:

```bash
$ cd examples/hello_world
$ cargo run
```

or

```bash
$ rustc +nightly-2021-12-19 -C link-args=-nostartfiles freestanding_hello_world.rs
$ ./freestanding_hello_world
```
