# JK - json_key_extractor

This tool can read `jsonl` files (a file with many `json` separated by newlines) and extract the common structure of the data.

```json lines
{"name": "Alice", "surname": "Red", "address":{ "street": "Berkeley road", "cap": 1111, "country": "USA"}, "orders_id": [1,2,3,4,5]}
{"name": "Bob", "surname": "Blue", "address":{ "street": "Garden street", "cap": 2222, "country": "UK"}}
```

```commandline
$ jk test.json
+-------------------+----------+
|  address          |  object  |
|  address.cap      |  int     |
|  address.country  |  string  |
|  address.street   |  string  |
|  name             |  string  |
|  orders_id        |  [int]   |
|  surname          |  string  |
+-------------------+----------+
```

It can also generate the code for de/serializing in scala.

```commandline
jk test.json -tscala
```

```scala
case class Address(
    cap: Int,
    country: String,
    street: String
)
case class RenameMe(
    address: Address,
    name: String,
    orders_id: List[Int],
    surname: String
)
```

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


