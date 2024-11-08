# Strings

## Reverse String

### Introduction

Reversing strings (reading them from right to left, rather than from left to right) is a surprisingly common task in programming.

For example, in bioinformatics, reversing the sequence of DNA or RNA strings is often important for various analyses, such as finding complementary strands or identifying palindromic sequences that have biological significance.

### Instructions

Your task is to reverse a given string.

Some examples:

- Turn `"stressed"` into `"desserts"`.
- Turn `"strops"` into `"sports"`.
- Turn `"racecar"` into `"racecar"`.

### Bonus

Test your function on this string: `uüu` and see what happens.
Try to write a function that properly reverses this string.
Hint: grapheme clusters

To get the bonus test to run, remove the ignore flag (`#[ignore]`) from the last test, and execute the tests with:

```bash
cargo test --features grapheme
```

You will need to use external libraries (a `crate` in rust lingo) for the bonus task.
A good place to look for those is [crates.io](https://crates.io/), the official repository of crates.
