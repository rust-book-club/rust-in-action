# Chapter 03: Compound data types

## 3.1 Using plain functions to experiment with an API

Macros like

```rs
#![allow(unused_variables)]
```

and

```rs
#[allow(dead_code)]
```

allow us to silence some compiler warnings.

Q: What's the difference between `#!` and `#`?

A: "When attributes apply to a whole crate, their syntax is `#![crate_attribute]`, and when they apply to a module or item, the syntax is `#[item_attribute]`..." [Rust by Example](https://doc.rust-lang.org/rust-by-example/attribute.html)

`()` is the "Unit" type, `!` is the "Never" type.

## 3.2 Modeling files with struct

"Newtype" is the term for a type like `struct Hostname(String)`, where a type just wraps another type