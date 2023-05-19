# Chapter 01: Introducing Rust

"variable binding" rather than "assignment"

> "Rust is the programming language that allows Haskell and Java programmers to get along. Rust comes close to the high-level, expressive feel of dynamic languages like Haskell and Java while achieving low-level, bare-metal performance."

> "Rust is not object-oriented as it does not support inheritance"

```rs
// "conditional compilation"
if cfg!(debug_assertions) { // <- not included in release builds
    eprintln!("debug: {:?} -> {:?}", record, fields);
}
```

```rs
// multiline strings
// "Rust strings are multiline by default"
//  - https://tourofrust.com/62_en.html
let penguin_data = "\
  common name,length (cm)
  Little penguin,33
  Yellow-eyed penguin,65
  Fiordland penguin,60
  Invalid,data
  ";
```

I'm really glad I read The Book before this one. This book really dives right in.

> "fearless concurrency"

"unit" type (`()`) only has a single value, unlike `bool`'s two values, `string`'s infinite values, etc.

Cargo is the frontend for `rustc`, the Rust compiler

Rust's `Box` is like boxed integers in Java, just realized that.

> "Our tools shape what we believe we can create."

Need some more explanation of "data-oriented programming"...

> "Cache-friendly data structures are provided by default. Arrays usually hold data within Rust programs rather than deeply nested tree structures that are created by pointers. This is referred to as data-oriented programming."

Book makes a good point that there's no official package manager for C / C++. Cargo is definitely a big win for Rust there.

Need to review static vs. dynamic dispatch

What is a "global interpreter lock" (GIL)? The book says Rust doesn't have one of those.

"précis" -- "a concise summary of essential points, statements, or facts", according to Merriam-Webster

Many Rust types are zero-sized, the types themselves are only hints to the compiler and take up no space in the compiled executable.

The three main Rust command-line tools: `cargo`, `rustup`, and `rustc`

