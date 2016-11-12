# maze-rs
Maze for Programmers (Rust)

## Status

[![Build Status](https://travis-ci.org/korczis/maze-rs.svg?branch=master)](https://travis-ci.org/korczis/maze-rs)

## Usage

```
$ ./target/debug/maze -h
Maze Generator 0.1.0
Tomas Korcak <korczis@gmail.com>

USAGE:
    maze [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode

OPTIONS:
    -y, --height <height>    Height of Maze [default: 5]
    -x, --width <width>      Width of Maze [default: 5]
```

## Example

```
$ ./target/debug/maze -x 10 -y 10

+---+---+---+---+---+---+---+---+---+---+
|   |   |               |               |
+   +   +---+---+---+   +---+---+---+   +
|   |   |   |               |   |       |
+   +   +   +---+---+---+   +   +---+   +
|   |   |                           |   |
+   +   +---+---+---+---+---+---+   +   +
|       |                       |   |   |
+---+   +---+---+---+---+---+   +   +   +
|   |       |   |   |       |       |   |
+   +---+   +   +   +---+   +---+   +   +
|               |   |   |               |
+---+---+---+   +   +   +---+---+---+   +
|   |           |   |       |           |
+   +---+---+   +   +---+   +---+---+   +
|                               |       |
+---+---+---+---+---+---+---+   +---+   +
|   |   |       |       |   |   |   |   |
+   +   +---+   +---+   +   +   +   +   +
|                                       |
+---+---+---+---+---+---+---+---+---+---+
```