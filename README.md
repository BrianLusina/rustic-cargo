# RustiCargo

Tiny mini Rust programs, data structures and algorithms.

## Running the tests

Execute the tests with:

```bash
$ cargo test
```

If you wish to run _only ignored_ tests without editing the tests source file, use:

```bash
$ cargo test -- --ignored
```

If you are using Rust 1.51 or later, you can run _all_ tests with

```bash
$ cargo test -- --include-ignored
```

To run a specific test, for example `some_test`, you can use:

```bash
$ cargo test some_test
```

If the specific test is ignored, use:

```bash
$ cargo test some_test -- --ignored
```

To learn more about Rust tests refer to the online [test documentation][rust-tests].

[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## Rust Installation

Refer to the [download page](https://rust-lang.org/tools/install/) for Rust installation and learning resources.
