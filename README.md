# Hello

A 'hello world' package for Rust.

## Installation

You can install `hello` with `rustpkg`:

```
$ rustpkg install github.com/steveklabnik/hello
```


## Usage

Here's a sample program using hello:

```
extern mod hello;

fn main() {
  hello::world();
}
```

If you put that into a file called `hello.rs`, you can:

```
$ rustc hello.rs
$ ./hello
```

## License

MIT licensed.
