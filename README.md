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
echo Vasilios | zargs -r name echo "Hello, name!"
Hello, Vasilios!
```

#### Should work on Windows!
```shell
echo "write-output 'hello world :)'" | zargs powershell
hello world :)
```

Help prompt
-----------

```
$ zargs --help
zargs 0.1.1
Vasilios Syrakis <cetanu@gmail.com>

USAGE:
    zargs [OPTIONS] [command]...

ARGS:
    <command>...    The command to execute against the args

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --arg-file <arg-file>      Read args from a file
    -d, --delimiter <delimiter>    Split the args by a particular delimiter
    -p, --parallel <parallel>      The number of threads to run in parallel [default: 1]
    -r, --replace <replace>        Replace occurences of this with args read from stdin
```
