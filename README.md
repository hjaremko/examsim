# Examsim ![Build](https://github.com/hjaremko/examsim/workflows/Build/badge.svg?branch=master)
Closed question exam practice. 

## Running
```
cargo run --release -- <command line options>
```

## Usage
```
USAGE:
    examsim.exe [FLAGS] --file <FILE>

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    Print debug messages (debug build only)
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>    Specify exam file
```
## Exam file structure
```
[question]
Question text, one or
more lines

[ans]
Incorrect answer
[ans*]
Correct answer
[ans]
Another incorrect answer

[question]
Another question
...
```

## Downloads

Binary files are available [here](https://github.com/hjaremko/examsim/releases).
