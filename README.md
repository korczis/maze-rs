# maze-rs
Maze for Programmers (Rust)

## Status

[![Build Status](https://travis-ci.org/korczis/maze-rs.svg?branch=master)](https://travis-ci.org/korczis/maze-rs)

## Prerequisites

- [rust](https://www.rust-lang.org/en-US/)

## Usage

```
$ ./target/debug/maze -h
Maze Generator 0.1.0
Tomas Korcak <korczis@gmail.com>

USAGE:
    maze [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -r, --rest       Run REST Server
    -V, --version    Prints version information
    -v, --verbose    Verbose mode

OPTIONS:
    -a, --algorithm <algorithm>    Algorithm to use [default: sidewinder]  [values: binary,
                                   sidewinder]
    -f, --format <format>          Output format to use [default: ascii]  [values: ascii, json]
    -y, --height <height>          Height of Maze [default: 5]
    -p, --rest-port <rest-port>    REST Port [default: 5000]
    -x, --width <width>            Width of Maze [default: 5]
```

## Generator Algoritms

- [x] Aldous-Broder
- [x] Binary
- [x] Sidewinder

## Output Formats

- [x] ASCII Art
- [ ] PNG
- [x] JSON

## Benchmark

### Aldous-Broder

```
$ time ./target/release/maze -a aldous-broder -x 1000 -y 1000 > maze.txt

real	0m16.928s
user	0m13.188s
sys	0m3.555s
```

### Binary

```
$ time ./target/release/maze -a binary -x 1000 -y 1000 > maze.txt

real	0m1.804s
user	0m1.476s
sys	0m0.268s
```

### Sidewinder

```
$ time ./target/release/maze -a sidewinder -x 1000 -y 1000 > maze.txt

real	0m1.953s
user	0m1.539s
sys	0m0.299s
```

## Example

```
$ ./target/debug/maze -x 20 -y 20

+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
|       |   |       |   |   |       |       |       |   |   |                   |
+---+   +   +   +---+   +   +---+   +   +---+   +---+   +   +---+---+   +---+---+
|   |   |   |   |   |                           |                           |   |
+   +   +   +   +   +   +---+---+---+---+---+---+---+   +---+---+---+---+---+   +
|   |       |       |           |       |   |   |       |   |       |   |   |   |
+   +   +---+   +---+   +---+---+   +---+   +   +   +---+   +   +---+   +   +   +
|                   |   |   |       |       |       |   |               |       |
+---+---+   +---+---+   +   +---+   +---+   +---+   +   +---+---+   +---+---+   +
|   |   |   |       |   |   |   |   |   |               |   |   |   |   |   |   |
+   +   +   +---+   +   +   +   +   +   +   +---+---+---+   +   +   +   +   +   +
|           |   |   |   |           |   |               |   |       |   |   |   |
+   +---+---+   +   +   +   +---+---+   +---+   +---+---+   +---+   +   +   +   +
|   |   |   |           |   |   |       |       |       |   |   |       |       |
+   +   +   +---+   +---+   +   +   +---+   +---+---+   +   +   +   +---+   +---+
|   |       |       |   |       |           |   |           |   |       |       |
+   +   +---+   +---+   +   +---+---+   +---+   +   +---+---+   +   +---+---+   +
|       |               |               |   |                   |   |   |   |   |
+   +---+---+   +---+---+---+---+   +---+   +   +---+---+---+---+   +   +   +   +
|               |       |       |   |               |       |       |           |
+---+   +---+---+---+   +---+   +   +---+   +---+---+   +---+---+   +---+   +---+
|       |   |       |                   |   |   |   |   |           |       |   |
+---+   +   +   +---+   +---+---+---+---+   +   +   +   +---+---+   +---+   +   +
|   |   |               |                   |       |       |   |               |
+   +   +   +---+---+---+   +---+---+---+---+   +---+---+   +   +   +---+---+---+
|           |   |   |               |   |   |   |       |   |           |       |
+---+   +---+   +   +---+   +---+---+   +   +   +---+   +   +---+   +---+---+   +
|   |   |   |   |   |   |       |                   |       |   |       |       |
+   +   +   +   +   +   +---+   +---+---+---+   +---+---+   +   +---+   +---+   +
|   |   |   |       |       |                   |       |   |       |   |   |   |
+   +   +   +   +---+---+   +---+---+   +---+---+---+   +   +   +---+   +   +   +
|       |   |   |   |   |       |       |       |   |   |               |   |   |
+   +---+   +   +   +   +   +---+---+   +---+   +   +   +---+   +---+---+   +   +
|   |       |       |       |   |           |           |   |                   |
+   +   +---+   +---+   +---+   +---+---+   +---+---+   +   +---+---+---+   +---+
|   |           |           |   |   |                           |               |
+   +   +---+---+---+---+   +   +   +---+---+---+   +---+---+---+---+---+---+   +
|   |       |   |       |           |               |   |           |   |       |
+   +   +---+   +---+   +---+   +---+   +---+---+---+   +---+   +---+   +   +---+
|                                                                               |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
```