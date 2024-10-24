---
title: A Beginner's Guide to Rust
author: Josh Jeppson
aspectratio: 169
fonttheme: "professionalfonts"
mainfont: "Hack Nerd Font"
fontsize: 10pt
---

# Rust

- A compiled, safe, systems programming language.
- Started by Grayden Hoare in 2006 (akaik no relation to logician Tony Hoare of Hoare-triple/design-by-contract fame).
	+ Officially picked up by Mozilla in 2009, now developed by Rust foundation.
- One of three languages (next to C and ASM) used in the Linux Kernel.
	+ Linus Torvalds, who notably despises C++, finds Rust favorable.

# Why Rust?

- It's memory-safe (mostly) without a garbage collector.
	+ Memory lifetime is enforced by a "borrow checker" at compile-time.
- It prevents data races, enforces type safety, and is tuned to multithreaded programs.
- Its linter, `clippy`, blurs the line slightly between linter and static verifier.

# Cargo

- A Rust package is called a "crate", hence...
- "Cargo" is Rust's package manager and build system.
- Super easy to declare dependencies. If they're on crates.io, Cargo will just go automatically retrieve them.

# How to install and use

- The most popular way to install Rust is [rustup](https://rustup.rs/)
- Create a new project via `cargo new <project name> --bin|lib`
	+ This bootstraps a project with a `Cargo.toml` and a skeleton directory
	+ The `Cargo.toml` provides metadata about your project including license, dependencies, etc.
	+ Dependencies can be automatically downloaded from [crates.io](https://crates.io) on build

# The Rust Philosophy

- *Safe*, *explicit* programs.
- Requires the programmer to think about:
	+ Memory locations and lifetimes
	+ Ownership and resource control
	+ Thread safety
- The difference is that, unlike C, Rust programs will not compile if memory errors are present.

# Hello World

Let's create a "hello world" program in Rust, shall we?

# Hello World

Please navigate to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) to download and install Rust. If you're on Linux or Mac, you may use your preferred package manager (`pacman`, `apt`, `dnf`, `brew`, etc) to install the `rustup` package.

Next, create a directory for our program to live.

# Hello World

Let's navigate to our projects directory and execute the following command:

```bash
cargo new hello-world-rs --bin
```

This initializes the bare minimum for a Rust project built by Cargo.

# Hello World

We should now have the following folder structure

```
hello-world-rs/		(root project directory)
	∟ Cargo.toml	(project metadata and deps)
	∟ src/			(source directory)
		∟ main.rs	(entrypoint)
```

# Hello World

Let's take a look at `Cargo.toml`

```toml
[package]
name = "hello-world-rs"
version = "0.1.0"
edition = "2021"

[dependencies]
```

Pretty self explanatory. No need to edit for now.

# Hello World

`Cargo.toml` is in TOML, a type of markup language used for config files for various languages.

Syntax:

```toml
[category-name]
variable_name = "value"

[category-name.subcategory]
subvariable_name = "other value"

# A comment
```

Full spec located at [https://toml.io/en/](https://toml.io/en/). Supported Cargo manifest keys are located at [https://doc.rust-lang.org/cargo/reference/manifest.html](https://doc.rust-lang.org/cargo/reference/manifest.html).

# Hello World

Now, let's look at `src/main.rs`:

```rust
fn main() {
	println!("Hello world!");
}
```

Looks like Cargo did it for us!

# Hello World

To build: `cargo build` (executables are placed in `target/`)
To run: `cargo run` (auto-runs the executable in `target/`)

# Not so fast

We haven't written any Rust code ourselves!

**An exercise**: using the online Rust handbook, figure out how to get the program to print `Hello {name}!` for each name passed in via argv.

Example output:
```
$ cargo run Josh Landon Beckey
Hello Josh!
Hello Landon!
Hello Beckey!
```

# Solution

- You'll need to `use std::env;`
- This contains the function `env::args()` which returns the argv list (as an iterator!)
- Skip the program name using `env::args().skip(1)`
	+ For indexable iterators (which `args` is *not* unless you `.collect()` it) you can also do `[1..]`

# Solution

A more traditional approach:
```rust
for arg in env::args().skip(1) {
	println!("Hello {}!", arg);
}
```

The for loop can also be written as...

```rust
for arg in &env::args().collect::<Vec<_>>()[1..] {
```

...but this is less memory efficient because it copies the args into a `Vec`. 

# Solution

A more "functional" approach:
```rust
env::args().skip(1).for_each(|arg| {
	println!("Hello {}!", arg);
});
```

# Control flow in Rust

Rust has all of the standard control flow statements:

- `if` statements
- `while` and `for` loops
- `match` expressions

# `if` Statements

We can use `if` traditionally...

```rust
if condition { 
	... 
} else if condition2 { 
	... 
} else { 
	... 
}
```

...or like the ternary operator...

```rust
let myVal = if condition { option1 } else { option2 };
```
...but it's more idiomatic to use a `match` here.

# `while` and `for` Loops

Executes while `condition` is true:

```rust
while condition { 
	// Do something here
}
```
Iterates over `iterable`:

```rust
for item in iterable {
	// Do something here
}
```

Some primitive iterables:

+ `0..5` denotes a *range* from 0 to (*not through*) 5
+ `0..=5` denotes a *range* from 0 *through* 5

# Match expressions (like `case` in C/++)

```rust
let foo = match expression {
	possibility1 => return_value1,
	possibility2 => return_value2,
	_ => catchall_value
};
```

# Exercise 2

- Print *all* of the names on one line:
	+ Commas in between all names but the last one
	+ The word "and" in between the first and last name in the list
- If no names are provided, print `Hello World!`
- **Hint:** use the `format!()` macro.

# Exercise 2

Example outputs:
```bash
$ cargo run # No arguments, prints "Hello World"
Hello World!
$ cargo run Josh Landon # Two arguments, prints "and" between them
Hello Josh and Landon!
$ cargo run Josh Beckey # Also two arguments
Hello Josh and Beckey!
$ cargo run Josh Landon Beckey # >2 args, prints as a list
Hello Josh, Landon and Beckey!
```

# Exercise 2

A simple, elegant solution:

```rust
// Get a slice of the args from just where we want,
// then collect so we know the length and can index them.
let args = &env::args().collect::<Vec<_>>()[1..];
// Match is like a case statement in C
let names = match args.len() {
	// Prints "Hello World!" if no names provided
	0 => String::from("World"),
	// If only one name, just print that name
	1 => args[0].to_string(),
	// If only two names, just print them separated by an " and "
	2 => format!("{} and {}", args[0], args[1]),
	// If more, join the first n - 1 names with ", " and then put an " and " between that
	// and the last name provided. Note `_` is a catchall in a match statement
	_ => format!("{} and {}", args[..args.len() - 1].join(", "), args[args.len() - 1])
};
println!("Hello {}!", names);
```

# Exercise 3

- The user will input a list of numbers via `argv`. 
- You are to compute the factorial of each number and print them on stdout.
	+ You are also to show the *intermediate* steps of the computation. 
    + I.e., if you get `3`, you should print `3! = 1 * 2 * 3 = 6`. 
- If you get a float or something that doesn't parse to an int, just ignore it.
- No need to memoize or do any dynamic programming at this point.

```
$ cargo run 2 3 4 blah
2! = 1 * 2 = 2
3! = 1 * 2 * 3 = 6
4! = 1 * 2 * 3 * 4 = 24
```

# Exercise 3

Hints:

- Store the factorial result as an `i128`. Rust programs panic at integer overflow
- The `.fold()` method can be used to compute the factorial as a one-liner
- `.join(" * ")` can be used to convert a list of *strings* to a single string, delimited by ` * `
- Consts are denoted by `let my_variable : Type = value;`
- Variables are denoted by `let mut my_variable : Type = value;`
    + Types can be omitted/inferred if they are obvious at compile time.

# Exercise 3

A novice solution:

```rust
for arg in env::args().skip(1) {
	// Try to parse the args as an i32
	let i_option = arg.parse::<i32>();
	if !i_option.is_ok() {
		continue;
	}
	let i_value = i_option.unwrap();
	print!("{}! = ", i_value);
	let mut fac : i128 = 1;
	for j in 1..=i_value {
		// Compute this step of the factorial
		fac *= j as i128;
		// If j < i, we should print a * following our integer
		if j < i_value {
			print!("{} * ", j);
		} else {
			print!("{} = ", j);
		}
	}
	println!("{}", fac);
}
```

# Exercise 3

An intermediate solution:

```rust
for arg in env::args().skip(1) {
	// Try to parse the args as an i32
	let i_option = arg.parse::<i32>();
	if !i_option.is_ok() {
		continue;
	}
	let i_value = i_option.unwrap();
	let range : Vec<i32> = (1..=i_value).collect();
	let intermediate_steps = range
		.iter()
		.map(|j| j.to_string())
		.collect::<Vec<_>>()
		.join(" * ");
	let mut fac : i128 = 1;
	for j in range {
		// Compute this step of the factorial
		fac *= j as i128;
	}
	println!("{}! = {} = {}", i_value, intermediate_steps, fac);
}
```

# Exercise 3

A more advanced solution:

```rust
env::args()
	.skip(1)                       // Skip the first argument (the executable name),
	.map(|arg| arg.parse::<i32>()) // parse each argument to an i32 int,
	.filter(|i| i.is_ok())         // only operate on successful parsings,
.for_each(|i| {                    // and iterate on the results
	// At this point, we know this won't panic
	let i_value = i.unwrap();
	let range : Vec<i32> = (1..=i_value).collect();
	let intermediate_steps = range
		.iter()
		.map(|j| j.to_string())
		.collect::<Vec<_>>()
		.join(" * ");
	// Compute the factorial via closure and .fold()
	let fac : i128 = range.iter().fold(1, |acc, j| acc * (*j as i128));
	println!("{}! = {} = {}", i_value, intermediate_steps, fac)
});
```

# Advanced doesn't always equal efficient

- All of these solutions run in $O(n)$ time.
- However, the "intermediate" and "advanced" solutions also use $O(n)$ memory, whereas the "beginner" solution only uses $O(1)$.
	+ Is this true if the intermediate printing step is omitted?
	+ Considering we're sending output to the screen anyway, does it really matter?

# Exercise 4: Some Optional "Homework"

Combine Exercises 2 and 3 so that if the parameter is an integer, compute its factorial. Otherwise, say `Hello {parameter}!`.

# Conclusion

- In this slide show, we covered:
    + Bootstrapping a Rust project
    + Cargo, crates.io, and TOML
    + Basic Rust control flow

- Next time: functions, closures, types, and traits.
