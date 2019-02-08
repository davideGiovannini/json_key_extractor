## Build

Make sure to have rustc and cargo installed (see [rustup](https:/./rustup.rs/)).

Run `cargo build --release` from the project folder to build.

Run `cargo run --release -- <arguments>` from the project folder to run the command with `arguments`.

Run `cargo install --path .` to install the final binary to `~/.cargo/bin`.

## Usage
```
json_key_extractor 0.1.1
Davide Giovannini <giovannini.davide90@gmail.com>
Extract structure information from a jsonl file.

USAGE:
    jk [OPTIONS] [input_path]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --color <color>             When to use colors. Can be one of (never, always, auto) [default: auto]
    -t, --type <format>             Default output format. Can be one of (scala, terminal) [default: terminal]
    -n, --nthreads <num_threads>    Number of threads (defaults to the number of logical thread available)

ARGS:
    <input_path>    File to process, if not provided stdin will be used.
```


