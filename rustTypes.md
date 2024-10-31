---
title: Typing in Rust
author: Josh Jeppson
---

# Introduction

- Rust is *strongly typed*, meaning there are a strict set of rules regarding how a type may be interpreted.
	+ Integer casting is generally explicit, unlike in C and C++ where it may be implicit.
- Rust is *statically typed*, meaning an object of type `Foo` stays type `Foo` throughout its lifetime.
- Rust allows *type inference*/*implicit typing* which means that an object's type may be assumed at initialization.
- In Rust, "polymorphism" (in a loose sense) is introduced via *traits*.

# Declaring Variables and Constants

The `let` keyword declares something to be *immutable*...

```rust
let a : u8 = 5;
a = 6; // ERROR
```

...whereas `let mut` allows *mutation*.

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

- Supported IEEE 754-2008 floating point types are `f32` (roughly equivalent to C's `float`) and `f64` (double precision).
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

# Strings, an exercise

Create a program that:

- determines if strings passed in via `env::args()` are palindromes *without* copying or modifying the strings!
- If the word is a palindrome, simply print (`this word is a palindrome`).
- If not, compute the number of character changes needed to make it a palindrome, and apply those changes to a *new* string.
	+ Print the new string.
- Your program should contains a function which performs your evaluation.
	- It should take a reference to a string slice (i.e., a `&str`)
	- It should return a tuple `Option<(u8, String)>` where the option is empty iff the word passed in is a palindrome.
		+ We'll talk more about `Option`s later but you've already seen them.

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
- A *raw pointer* is a very rare type that can *only* be de-referenced in an `unsafe` block.
- Multiple smart pointer types

# Function Types

- Functions are first-class objects in Rust and are represented by the `fn` (function pointer) type
- It's more common to let the type-inference system assign the type of a function variable.
- It's also common to restrict the type to anything that implements the `Fn` or `FnMut` traits.
	+ We'll talk more about this later

# The `Option` and `Result` Types

- Because Rust requires all types to hold a value, and does not have any equivalent of `null` or `nil`, we need some way to handle when a value cannot be created.
- Some languages have type unions (e.g. `T | null` in Dafny or similar in TypeScript), but Rust takes a different approach.
- **The goal:** We want a wrapper that can contain a computation result, or may not.
	+ If you're into functional programming, you may recognize these as common types of *monad*.

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

# Trait Types

- A trait is a special type of object that defines *behavior*.
	+ Similar to an *interface* or abstract class in OOP languages.
	+ Traits *cannot* store instance level data (but can store what is called *associated data*)
- Any object that implements a trait `MyTrait` can be referred to by `dyn MyTrait` or `impl MyTrait`
	+ `dyn` is a dynamic dispatch object that can be referenced. `impl MyTrait` is passed by value
- Traits may be *implemented* for *any* type using the `impl` keyword.
- Some traits allow implicit implementation via `#[derive()]`

# Traits, Downcasting

- It is not, strictly speaking, impossible to check what *specific* type a reference to `dyn MyTrait` is, but it isn't easy.
- We need to use `downcast_ref::<Type>` and check to see if the type conversion is successful

```rust
// MyType implements MyTrait
let my_obj : Box<dyn MyTrait> = // ...
// ...
if let Some(as_type) = my_obj.downcast_ref::<MyType>() {
	// Handle specific MyType value
} else {
	// Do nothing or panic as it's not MyType
}
```

# Traits: A Basic Example

```rust
trait Geometry {
	fn area(&self) -> f32;
	fn circumference(&self) -> f32;
}

// this automatically gets us a free implementation for the
// Copy and Clone traits. This is like a `default` copy constructor
// in C++, added by the compiler.
#[derive(Copy, Clone)] 
struct Circle { radius: f32, }
#[derive(Copy, Clone)]
struct Rectangle { width: f32, height: f32 }

impl Geometry for Circle {
	fn area(&self) -> f32 { 3.1415 * self.radius * self.radius }
	fn circumference(&self) -> f32 { 2.0 * 3.1415 * self.radius }
}

impl Geometry for Rectangle {
	fn area(&self) -> f32 { self.width * self.height }
	fn circumference(&self) -> f32 { 2.0 * (self.width + self.height) }
}
```

# Using Our Example

```rust
// We can take a vector of things which implement the
// Geometry trait, rather than a vector of Circles or Rectangles
fn total_disjoint_area(shapes : Vec<Box<dyn Geometry>>) -> f32 {
	shapes.iter()
	.map(|x| x.area())
	.sum()
}

// Pass by value. This takes an actual copy of something that implements
// Geometry and uses it in the function. 
fn sum_of_circumferences(shape1 : impl Geometry, shape2 : impl Geometry) -> f32 {
	shape1.circumference() + shape2.circumference()
}

// Pass by reference. We take a reference to the dynamic dispatch object, `dyn Geometry`
fn sum_of_circumferences_ref(shape1 : &dyn Geometry, shape2 : &dyn Geometry) -> f32 {
	shape1.circumference() + shape2.circumference()
}

fn main() {
	let shapes : Vec<Box<dyn Geometry>> = vec![
		Box::<Circle>::new(Circle{ radius: 2.6 }),
		Box::<Circle>::new(Circle{ radius: 3.4 }),
		Box::<Rectangle>::new(Rectangle{ width: 3.2, height: 1.5 })];

	println!("Total used area {}", total_disjoint_area(shapes));

	let circle1 = Circle{ radius: 5.0 };
	let square = Rectangle{ width: 2.0, height: 2.0 };

	println!("Sum of first two circumferences (pbv) {}", sum_of_circumferences(circle1, square));
	println!("Sum of first two circumferences (pbr) {}", sum_of_circumferences_ref(&circle1, &square));
}
```

# Comprehension Question

In our example, what lives on the heap, and what lives on the stack?

# Exercise

- You work at a store with multiple different kinds of media products: books, movies, etc.
- Create a trait called `Display` which returns a nice string representation of a product.
- Create a `Book`, `Movie`, and `Album` struct which implement this trait.
	+ `Book` should contain fields `author` (string), `genre` (enum type), and `publisher` (string)
	+ `Movie` should contain fields `director` (string), `actors` (vector), `length` (float), `release_year` (unsigned int), `genre` (enum type) and `distributor` (string)
	+ `Album` should contain fields `artist` (string), `genre` (enum type), `release_year` (unsigned int), and `label` (string).
- The `genre` fields should all be *unique* enum types, e.g., `enum BookGenre`, `enum MovieGenre`, `enum MusicGenre`. You don't read a "synthwave" book.
- Create a list of mixed books, movies, and albums, and print each one.
- **Extra ~~Credit~~ Kudos:** use `partition()` or `filter()` (or some unholy overpowered thing in the crate `itertools`) to get vectors of just the books, movies, etc.
