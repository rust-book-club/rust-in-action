# Chapter 02: Language Foundations

This book seems confused about its target audience. It jumps right into a code example in Chapter 1, then backpedals in Chapter 2 and explains the "magic" of a compiler. Who are you talking to?

## 2.1 Creating a running program

Commands in the book sometimes don't work. For example, compiling `ok.rs` creates an executable called `ok`. The book then instructs to create a directory with `mkdir ok`, which fails, because there's already a file called `ok`.

## 2.2 A glance at Rust's syntax

> "Rust is boring and predictable where possible." Same.

`main()` returns unit by default but can also return a `Result`

> "Code blocks, also known as _lexical scopes_..."

Single quotes for characters in Rust, double quotes for strings.

> "The dagger (`->`) or thin arrow syntax indicates the return type."

## 2.3 Numbers

Rust does not automatically widen integer types.

In Rust, stack-based variables can have methods called on them. For example `24.5.round()`.

Wider integer types can represent more values, but at the cost of passing lots of zeroes around most of the time.

`PartialOrd` and `PartialEq` traits allow numbers to be ordered and compared.

Very cool Rust REPL -- https://github.com/evcxr/evcxr/blob/main/evcxr_repl/README.md

I also did `alias rust=evcxr` so I can just type `rust` at the command line and get a Rust REPL

```rs
>> let a: i32 = 10;
let b: u16 = 100;
if a < b {
println!("Ten is less than one hundred.");
}
[E0308] Error: mismatched types
   ╭─[command:1:1]
   │
 3 │ if a < b {
   ·    ┬   ┬│
   ·    ╰────── expected because this is `i32`
   ·        ││
   ·        ╰── expected `i32`, found `u16`
   ·         │
   ·         ╰─ help: you can convert a `u16` to an `i32`: `.into()`
───╯
>>
```

> Typo: "If the conversion between `u16` and `i32` were to fail, then calling `unsafe()` would crash the program." (meant `unwrap()`, not `unsafe()`)

> "A distinguishing characteristic of Rust is that it only allows a type's methods to be called when the trait is within local scope. An implicit prelude enables common operations such as addition and assignment to be used without explicit imports."

Avoid testing floating-point numbers for equality. Use epsilons instead.

```rs
>> 0.1 + 0.2
0.30000000000000004
>> f32::EPSILON
1.1920929e-7
```

Lots of mathematical detail here. Very different from The Book, which glossed over all of this.

The `num` crate provides

- rational numbers
- complex numbers
- arbitrary-size integers
- arbitrary-precision floating point numbers
- fixed-point decimal numbers for working with currencies

> "Rust does not have constructors; instead, every type has a literal form"

install `cargo-edit` and use `cargo add` to add a dependency to a crate, rather than manually editing `Cargo.toml`

```shell
cargo install cargo-edit
cargo add num
```

## 2.4 Flow control

Rust `for` loop shorthand:

- use `for item in collection {}` rather than `for item in IntoIterator::into_iter(collection) {}` (ownership)
- use `for item in &collection {}` rather than `for item in collection.iter()` (immutable borrow)
- use `for item in &mut collection {}` rather than `for item in collection.iter_mut()` (mutable borrow)

Anonymous loops

```rs
for _ in 1..=10 {
  // do thing exactly 10 times
}
```

Prefer `loop {}` to `while true {}`.

`loop` endlessly loops until the program is forcibly terminated from the outside, or until a `break` is encountered

Use loop labels to `break` out of nested loops (or `continue` to an outer loop)

`break` can return a value, as well (e.g. `let n = loop { break 42; };`)

Everything in Rust is an expression except for

- expressions delimited by a semicolon `;` (called "expression statements")
- variable bindings using `=`
- type declarations like `fn`s, `struct`s, `enum`s, etc.

...the last two are "declaration statements"

## 2.6 Using references

aside: review "interior mutability" in Rust

When creating a reference like `let r = &a;`, the thing that the reference, `r`, refers to (`a`) is called the _referent_.

## 2.7 Project: Rendering the Mandelbrot set

This seems very much geared towards people with degrees in math and physics. Maybe not so much computer science. Like, are people comfortable with complex numbers?

Doesn't explain why to use `usize` vs `u32` etc.

Don't need type annotations on lines 14 and 17, Rust can figure this out.

Be careful to not add semicolons to the final lines of functions, or you'll return unit instead of the value you meant to return

This imperative style of programming is very different from Scala programming... is anyone having difficulties with it?

typo: lines 55-62 give "patterns overlap on their endpoints" compiler warnings

 - on all lines except 57-58, where the author correctly skips the value 11
 - this is fixed [in the source code on GitHub](https://github.com/rust-in-action/code/blob/1st-edition/ch2/ch2-mandelbrot/src/main.rs), would have been nice if they went back and updated the book

Note: you can do `println!("{line}")` instead of `println!("{}", line)`

Another typo: line 28: there is no variable called `all_rows`. This should be `rows`

Oof. This example literally does not work as written in the book. Very low expectations for this book.

typo: 2.0 on line 73 should be -2.0. Fixed on GitHub.

Really cool once you get it working but man are there so many typos.

## 2.8 Advanced function definitions

This section talks about lifetime elision but doesn't explain how it works, like The Book does.

"It's common to see lifetime parameters when using references. While Rust can infer lifetimes in other cases, references require the programmer to specify the intent." -- This is simply not true. References are _the only_ thing in Rust which require lifetimes and they _always_ require lifetimes. It's just that sometimes they're elided.

> "During the compilation process, `a + b` is converted to `a.add(b)`." Just like in Scala :')

