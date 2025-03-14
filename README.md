## say

this project is an `echo` alternative written in `rust` for fun.

## Compilation [Building]

- Copy this `git` repository
```bash
git clone https://github.com/namnam1105/say
```
- Install cargo and rustc
- To compile run:
```bash
cargo build --release
```
- To install say run:
```bash
cargo install --path .
```
- Use say


## Syntax


```bash
say [OPTIONS] "TEXT"
```
** Using quotes is not necessary but it helps reduce error with different shells.

Options:
- -f --format: enables formatting with ANSI escapes
- -h --help: print out built-in help
