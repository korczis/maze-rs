# maze-rs
Maze for Programmers (Rust)

## Status

[![Build Status](https://travis-ci.org/korczis/maze-rs.svg?branch=master)](https://travis-ci.org/korczis/maze-rs)

## Prerequisites

- [rust](https://www.rust-lang.org/en-US/)

## Usage

```
$ ./target/release/maze -h
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
    -a, --algorithm <algorithm>    Algorithm to use [default: aldous-broder]  [values: aldous-broder, binary, sidewinder]
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

### Low-Level Benchmark

```
$ cd dev/maze-rs/
tomaskorcak@kx-mac ~/dev/maze-rs$ cargo bench
    Finished release [optimized] target(s) in 0.0 secs
     Running target/release/deps/maze-0b23e703e6e9e01c

running 6 tests
test generator::aldous_broder::tests::bench_generate_100x100 ... bench:  61,610,364 ns/iter (+/- 24,858,439)
test generator::aldous_broder::tests::bench_generate_10x10   ... bench:     214,587 ns/iter (+/- 59,722)
test generator::binary::tests::bench_generate_100x100        ... bench:   5,123,808 ns/iter (+/- 976,614)
test generator::binary::tests::bench_generate_10x10          ... bench:      38,307 ns/iter (+/- 10,100)
test generator::sidewinder::tests::bench_generate_100x100    ... bench:   5,063,132 ns/iter (+/- 1,187,545)
test generator::sidewinder::tests::bench_generate_10x10      ... bench:      38,009 ns/iter (+/- 9,281)

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured

     Running target/release/maze-1018e5569854def3

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

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
|   |       |   |   |           |               |   |                   |       |
+   +---+   +   +   +   +---+   +   +   +   +   +   +---+---+---+---+   +---+   +
|           |               |       |   |   |   |               |       |       |
+---+---+   +---+   +---+   +---+---+---+   +   +---+   +   +---+   +---+---+   +
|   |       |   |   |   |       |           |   |       |           |           |
+   +   +   +   +   +   +---+   +---+---+   +---+   +   +   +---+   +   +---+   +
|   |   |   |       |   |       |   |   |   |       |   |   |   |           |   |
+   +   +---+   +---+   +   +---+   +   +---+---+---+   +   +   +---+---+   +---+
|                   |           |       |               |   |   |       |       |
+---+---+---+---+   +---+   +   +---+   +   +   +---+   +   +   +---+   +   +   +
|                       |   |               |       |   |               |   |   |
+   +   +---+   +   +---+---+---+---+---+---+---+   +   +---+   +   +---+   +---+
|   |   |       |   |                   |   |   |   |   |       |       |       |
+   +   +---+---+   +   +---+---+   +   +   +   +   +---+---+   +---+---+---+---+
|   |           |       |   |       |   |                   |       |   |   |   |
+---+---+   +---+---+---+   +---+   +---+---+   +   +   +---+   +   +   +   +   +
|                   |   |       |           |   |   |   |   |   |       |       |
+   +---+   +   +   +   +---+   +---+---+---+---+   +   +   +---+   +---+---+   +
|   |       |   |       |   |           |       |   |   |       |           |   |
+   +---+---+---+   +   +   +---+   +---+---+   +---+---+   +---+---+   +---+   +
|   |   |   |       |                   |               |       |   |       |   |
+---+   +   +   +   +---+---+   +   +---+   +---+   +   +---+   +   +---+   +   +
|       |   |   |   |       |   |           |   |   |           |   |           |
+---+   +   +---+---+   +---+   +   +---+   +   +---+---+---+   +   +   +---+---+
|                               |       |   |   |   |           |   |           |
+   +---+   +   +---+---+   +---+   +---+---+   +   +   +---+   +   +---+   +   +
|   |       |   |   |       |   |       |   |       |   |   |       |   |   |   |
+---+---+   +---+   +   +   +   +   +   +   +---+   +---+   +   +   +   +---+---+
|       |   |           |       |   |           |   |           |       |       |
+---+   +---+   +---+---+---+   +---+---+   +   +   +   +   +---+---+   +---+   +
|   |   |       |                   |       |   |       |       |   |           |
+   +   +---+---+   +---+   +---+---+---+   +   +---+---+---+   +   +---+---+   +
|       |   |   |   |                   |   |   |   |           |   |   |       |
+   +---+   +   +   +   +---+---+   +---+   +   +   +---+---+   +   +   +---+   +
|           |       |   |       |   |       |   |       |   |           |   |   |
+   +---+---+   +---+---+   +   +   +---+---+---+   +   +   +   +   +   +   +   +
|               |           |           |   |       |   |       |   |       |   |
+   +---+   +---+---+   +---+---+---+---+   +   +---+---+   +---+---+   +   +   +
|   |       |           |                                   |           |   |   |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
```