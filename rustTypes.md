---
title: Typing in Rust
author: Josh Jeppson
---

# Introduction

- Rust is *strongly typed*, meaning there are a strict set of rules regarding how a type may be interpreted.
	+ Integer casting is generally explicit, unlike in C and C++ where it may be implicit.
- Rust is *statically typed*, meaning an object of type `Foo` stays type `Foo` throughout its lifetime.
- Rust allows *type inferrence*/*implicit typing* which means that an object's type may be assumed at initialization.

# Declaring Variables and Constants

The `let` keyword declares something to be *immutable*...

```rust
let a : u8 = 5;
a = 6; // ERROR
```

...wherease `let mut` allows *mutation*.

```rust
let mut a : u8 = 5;
a = 6; // No error
```

# Primitive Types: Bool and Integers

- Rust contains a number of primitive types, including boolean, numeric, text, and "never" types.
- Boolean type is `bool`
- Supported integer types are: `u8`, `u16`, `u32`, `u64`, `u128` (unsigned), and `i8`, `i16`, `i32`, `i64`, `i128` (signed 2's compliment).
	+ The numbers indicate the number of bits used.
	+ `usize` and `isize` are also provided, which are the width of the platform's memory addresses. `usize` is used a lot in iteration.

# Primitive Types: Floats and Textual

- Supported IEEE 754-2008 floating point types are `f32` (rougly equivalent to C's `float`) and `f64` (double precision).
- Rust also supports `char`, string (`String`), and string slice (`str`) types.
	- `char` is 32 bits and supports UTF-8

# `str` vs `String`

- `str` is generally used via reference (`&str`).
	+ You can modify the contents of a `str` somewhere if you have a slice of it of type `&mut str`
- `str` is used for strings that already exist.
- `String` is dynamically sized string that you own.

# Strings, an example

```rust
// We don't own `greeting` or `name`, but instead,
// have references to them. `greeting` cannot live
// in static memory else it couldn't be mutable.
// `name` can live wherever, as long as
// its lifetime exceeds this function
fn add_name(greeting : &mut str, name : &str) {
	greeting.push_str(name);
}

// Converts string literal `&'static str` to type `String` so we own and can modify it
// I.e., moves the string "Hello, " to the heap.
let mut greeting = "Hello, ".to_string();
let name = "Zhen";

// greeting (type String) now has value "Hello, Zhen"
add_name(greeting, name);
```

# Type Casting

- *No implicit type conversion or casting!*
- To convert or cast some value `x` to another type, the `as` keyword is used.

```rust
let x : u8 = 125;
let y : u16 = 1054;
// let z : u16 = x + y; // ERROR!
let z : u16 = (x as u16) + y;
```

# Primitive Types: Never

- A *never* (denoted `!`) is a special type where $! = \emptyset$.
    + That is, there are *no* values that can be represented by the never type.
	+ This is *not* the same as `nullptr`, `null`, `nil`, or `None`.
	+ It can only be used as a return type in functions.

# Primitive Types: Never

...but why? Why would we ever want that?

```rust
fn errorAndExit(errMsg : &str) -> ! {
	panic!("Sorry, panicked :( {}", errMsg);
}
```
- `!` denotes that a function will never return.
- It's useful for processes that run forever or terminate the program.

# Builtin Types: Sequences

- Rust supports the following *sequence* types:
	+ Tuple types, denoted via `()`
	+ Array types, denoted via `[T; length]` (`T` is the type of array element)
		+ Arrays are fixed size, and *all elements must be initialized at creation*
		+ For dynamically-sized lists, `Vec<T>`
	+ Slice types, denoted `[]`

# Builtin Types: Slice Types

- Slices are pointers or references to portions of arrays and array-likes.
- There are 3 different ways to do slices:
	+ `&[T]`: the standard slice. Allows immutable borrowing/access to parent data.
	+ `&mut [T]`: Like `&[T]`, but allows mutation of the data accessed by the slice
	+ `Box<[T]>`: We'll talk about boxes later ;)

# Pointer Types

- Rust enforces memory safety, but allows pointer and reference types
- A *shared reference* is denoted with `&`, and is immutable, unless denoted `&mut`.
	+ Shared mutable references are the only way to access values elsewhere and not copy them.
- A *raw pointer* is a very rare type that can *only* be dereferenced in an `unsafe` block.
- Multiple smart pointer types

# Function Types

# The `Option` and `Result` Types

- Because Rust requires all types to hold a value, and does not have any equivalent of `null` or `nil`, we need some way to handle when a value cannot be created.
- Some languages have type unions (e.g. `T | null` in Dafny or similar in TypeScript), but Rust takes a different approach.
- **The goal:** We want a wrapper that can contain a computation result, or may not.
    + If you're into functional programming, you may recognize these as types of *monad*.

# Choice 1: `Option`

```rust
pub enum Option<T> {
	None,
	Some(T)
}
```

This may hold either a value of `T`, or nothing.

# An example of `Option`

```rust
pub fn sqrt(input : f32) -> Option<f32> {
	/* Do something here */
}

let m = sqrt(someVariable);
match m {
	Some(x) => /* Handle x = sqrt(someVariable) */
	None => /* We couldn't sqrt (someVariable < 0) */
}
```

# Choice 2: `Result`

```rust
pub enum Result<T, E> {
	Ok(T),
	Err(E)
}
```

This may hold a `T` if present, or an `E` (error) if not successful.

# Handling `Option` and `Result`

- You have a few options (ha!) to handle `Option` and `Result` types:
	+ `unwrap()`: gets the value of type `T` if exists, or panics if not.
	+ `unwrap_or(default : T)`: gets the value of type `T`, or, `default` if it doesn't exist.
	+ `unwrap_or_default()`: gets the value of type `T` or the *absolute default* for all type `T` (iff `T` implements the `Default` trait).
	+ `unwrap_unchecked()`: Can only be called in an `unsafe` block. Assumes there is no error and treats the value in the `Result` or `Option`.
<!--	+ `unwrap_or_else()`-->

# Heap-Allocated Pointers

- Rust provides the `Box` type for heap allocated pointers.
	+ A `Box<T>` represents a pointer to `T` on the heap.
		* The `T` it points to must be valid and initialized (enforced by the language).
	+ An `Option<Box<T>>` represents a *nullable* heap pointer.

# An example

```rust
// Type inference of someVec being Vec<u8>
let mut someVec = Vec<u8>::new();
```
