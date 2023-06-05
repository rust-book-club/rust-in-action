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

# 3.3 Adding methods to a struct with impl

> "Methods are inherently impure, given that one of their arguments is effectively a side effect."
> -- ...what?

> "An explicit type needs to be provided as vec! and can’t infer the necessary type through the function boundary."
> -- Not (or at least no longer) true

# 3.4 Returning errors

> Experienced programmers will know that using the global variable `errno` is commonly adjusted by the OS during system calls. This style of programming would typically be discouraged in Rust because it omits both type safety (errors are encoded as plain integers) and can reward sloppy programmers with unstable programs when they forget to check the `errno` value. However, it’s an important style to be aware of because
> - Systems programmers may need to interact with OS-defined global values.
> - Software that interacts with CPU registers and other low-level hardware needs to get used to inspecting flags to check that operations were completed successfully.

[_What's the difference between `let` and `const`?_](https://nickymeuleman.netlify.app/garden/rust-let-const)

- this book's answer to this question is very unclear... the above is much easier for me to understand.

# 3.6 Defining common behavior with traits

[_What does the `'_` lifetime signify?_](https://yegeun542.github.io/rust-edition-guide-ko/rust-2018/ownership-and-lifetimes/the-anonymous-lifetime.html)

