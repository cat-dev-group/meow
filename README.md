# Meow

This repository contains the source code for Meow. It contains the interpreter,
standard library, and documentation.

## Use

Currently, Meow can only be used by building it yourself.

Before you begin, you must have the Rust toolchain installed. Installation
instructions can be found on the dedicated
[installation page](https://www.rust-lang.org/tools/install).

The following command can be used to generate an optimized build, which can
then be found at `target/release/meow`, perhaps with an extension appropriate
for your system.

```sh
cargo build --release
```

## Development

The previously described dependencies are necessary for development.

For development, it may be easier to use the command for debug builds. This
will compile faster, but execute more slowly, as `rustc` does not take time to
perform optimizations. This can be ran with:

```sh
cargo build
```

Also, "internal" documentation can be generated with `rustdoc`. This
documentation is auto-generated from doc-comments within the source code, and
do not benefit the end users of Meow in any way. This can be invoked with the
following command:

```sh
cargo doc
```

If you would like to open the docs in your default browser, pass in the
`--open` flag as well.
