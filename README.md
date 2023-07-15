# linkchecker
You can use this to automate the process of checking whether your links work. You may use this as part of a shell script to make sure that all the urls in your html or latex file are good.

## Build/Install
To build run `cargo build`. To install to a path run `cargo install --path <PATH>` with your path.

## Features
1. You can pass in urls through stdin separated by whitespace
2. You can pass in urls just listed as command line args
3. You can pass in urls from a file using `-i`
