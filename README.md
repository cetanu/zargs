# zargs
A rewrite of xargs in Rust, for fun and science

Todo list
---------

Parameters implemented from `xargs`:

- [x] "Args file" ie. "Read items from file instead of standard input."
- [x] "Delimiter" ie. "Input items are terminated by the specified character."
- [x] "Replace" ie. "Replace occurrences of a string in the initial-arguments with names read from standard input."
- [x] ~"max-procs" ie. "Run up to max-procs processes at a time; the default is 1."~ uses Rayon for parallelization

Not sure if I can about other parameters! Maybe I can be convinced otherwise.

Installation
------------

```shell
cargo install zargs
```

Example Usage
-------------
#### Replacing strings, in the target command, with piped arguments
```shell
echo Vasilios | target/debug/zargs -r name echo "Hello, name!"
Hello, Vasilios!
```

#### Should work on Windows!
```shell
echo "write-output 'hello world :)'" | zargs powershell
hello world :)
```
