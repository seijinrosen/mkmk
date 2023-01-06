# mkmk

_mkmk_ is `mkdir -p` and `touch`.

[![Crates.io](https://img.shields.io/crates/v/mkmk)](https://crates.io/crates/mkmk)
[![Crates.io](https://img.shields.io/crates/d/mkmk)](https://crates.io/crates/mkmk)
[![cargo test](https://github.com/seijinrosen/mkmk/actions/workflows/tests.yml/badge.svg)](https://github.com/seijinrosen/mkmk/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/seijinrosen/mkmk/branch/main/graph/badge.svg)](https://codecov.io/gh/seijinrosen/mkmk)

## Installation

```sh
cargo install mkmk
```

## Usage

```sh
touch aaa/bbb/ccc.txt
# touch: aaa/bbb/ccc.txt: No such file or directory
# but...

mkmk aaa/bbb/ccc.txt
# A folder named "aaa/bbb" will be created, followed by a file named "ccc.txt".
# Equivalent to:
# mkdir -p aaa/bbb && touch aaa/bbb/ccc.txt
```
