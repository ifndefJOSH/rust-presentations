---
title: Functions and Generators in Rust, C++, and Python
author: Josh Jeppson
---

# The plan for today

We're going to compare some of the features in Rust with two languages you're probably familiar with, namely, Python and C++.

# Functions in Rust, C++, and Python

In Python and Rust, functions are *first-class citizens*. Thus they can be:

- assigned to variables, renamed or be anonymous (a.k.a "nameless"; see `lambda` keyword in Python)
- passed into functions as parameters.

C++ supports this somewhat as well, but it is limited, often requiring the use of `std::function`.

# Functions in Rust, C++, and Python

As we know, the usual syntax for function declaration in C++ is:

```c++
return_type_t functionName(param_type_t param1) {
	/* Function body */
}
```

If you want a pointer to this, it will be of type `std::function`.

# Functions in Rust, C++, and Python

For Python, the syntax is pretty straightforward:

```python
def functionName(param1 : type) -> type:
	# Function body
```
(The Python interpreter *does not enforce* type hints)

# Functions in Rust, C++, and Python

In Rust, this syntax is sort of a happy medium between the two:
```rust
fn functionName(param1 : Type) -> Type {
	/* Function body */
}
```
Note that types in Rust functions are *required*. Rust also supports *implicit return*, meaning the
last line's output is returned if there's no `;`. (See also: JavaScript)

# Closures

- A *closure* is just a record that stores a function.
	+ Often it is used synonymously with *lambda expression* which means the function is anonymous
- Closures and lambdas are supported in C++, Python and Rust.

# Closures (Python)

- As we know, Python functions are first-class, and can be treated as closures themselves (though
they are by definition not anonymous).

```py
def foo():
	print("Hello world")

printHelloWorld = foo
printHelloWorld() # Prints "Hello world"
```

But Python also provides a `lambda` keyword for true anonymous lambda expressions:

```py
printHelloWorld = lambda : print("Hello world")
```

# Closures (C++)

In C++, closures are defined in the following syntax:

```cpp
[capture_list](param_type_t param1) {
	// Closure body (return type is auto/implied)
}
```

To store this as a variable, we have to declare our variable of type `std::function`

# Closures (Rust)

Rust also lets us use closures:

```rust
|param : Type | -> ReturnType {
	// Closure body
}
```
In Rust, the parameter and return types for a closure are often inferred based on where it's used.

# Function Nesting (Python)

This is legal Python code:

```python
def foo(x : int): # No way to (correctly) type-hint
	def bar(y : int) -> int:
		return x + y
	return bar
```

- Because Python is memory managed, nested functions are valid closures, and can be returned.
- Type hint for `function` type invalid.
	+ Can use statically typed `typing.Callable` but `type(foo(a)) is Callable` will be false for our example.

# Function Nesting (Rust)

Rust allows function nesting...

```rust
fn foo(x : i32) -> i32{
	fn bar(y : i32) -> i32 {
		y + x
	}
	bar(2) / bar(3)
}
```
*Note: This is an example of __implicit return__ where the last line of a function has no semicolon and its output is returned.*

...but nested functions are not valid closures.

```rust
fn foo(x : i32) -> dyn Fn(i32) -> i32 { // ERROR!
	fn bar(y : i32) -> i32 {
		x + y
	}
	bar // ERROR!
}
```

# Function Nesting (Rust)

If you want a closure in Rust to be available outside of its initial scope, you need a `Box` to put it in.

```rust
fn foo(x : i32) -> Box<dyn Fn(i32) -> i32> {
	Box::new(move |y| {
		x + y
	})
}
```

Or more commonly (but less flexibly):

```rust
fn foo(x : i32) -> fn(i32) -> i32 {
	Box::new(move |y| {
		x + y
	})
}
```
Why is the first more flexible? Because `Fn*` is a trait, while `fn` is a function pointer thus is restricted to one type!

# Into Practice

Now that we have some background, let's see what we can put into practice.

# Rust/C++/Python Equivalent Code: Generators

We have some vector `myVec` and we want a *copy* of `myVec` with double the value of each element of `myVec`.

The icky way to do it (in C++ of course):

```cpp
std::vector<int> others(myVec.size());
for (int i = 0; i < myVec.size(); ++i) {
	others[i] = myVec[i] * 2;
}
```

But there's a better way to do it.

# Rust/C++/Python Equivalent Code: Generators

If you've used Python, you're probably familiar with *generators*:
```python
others = [2 * x for x in myVec]
```
But did you know these exist in C++, too?

```cpp
std::vector<int> others(myVec.size());
std::generate(others.begin(), others.end(),
	[](int x) { return 2 * x; });
// See also: std::generate_n() for n-length lists
```
And, naturally, they exist in Rust.
```rust
// NOTE: collect() iterates over the
// mapping and puts them all into new memory
let others = myVec.iter().map(|x| 2 * x).collect();
```

# Side Note: Iterators

- All three of these languages support the concept of *iterators*, objects which represent iteration
over a list or iterable.
- Iterator functionality is overload-able, so we can customize order of and what is iterated over.
- Iterators must support the following functionality:
	+ Some way of resolving the value underlying the iterator
	+ Some way of resolving the next iterator
- Iterators can be used as a form of *lazy loading* for elements in an iterable by defining how the next element is found rather than placing it next in a contiguous list.

# Rust/C++/Python Equivalent Code: Filters

I only want to iterate over values of a list *where some condition holds*. I.e., I only want even numbers.

In Python, this is easy:

```python
# This doesn't duplicate myList, f just
# points to the elements where the lambda
# returns True. This technique is called
# *lazy loading*
f = filter(myList, lambda elem : elem % 2 == 0)
fCopy = list(f) # Un-lazies your loading
```

# Rust/C++/Python Equivalent Code: Filters

Of course, it (kind of) exists in C++:

```cpp
// This does NOT do lazy loading
std::vector<int> fCopy(myList.size());
auto it = std::copy_if(myList.begin(), myList.end(),
	fCopy.begin(), [](int i) { return i % 2 == 0; });
// Iterator shenanigans to do resizing
fCopy.resize(std::distance(fCopy.begin(), it));

// Or in C++20 and newer, you can do the following:
// NOTE: you have to include <ranges>
// and this is not as portable
auto f = std::ranges::views::filter(
	[](int i){ return i % 2 == 0; });
```

# Rust/C++/Python Equivalent Code: Filters

And, naturally, Rust has a one-liner:

```rust
// Note, omit collect() for lazy loading/resolution
let f = myList.iter().filter(|elem| elem % 2 == 0);
let fCopy = f.collect(); // Un-lazies your loading
```

# Rust Polymorphism

- Rust supports generic functions using the following syntax:

```rust
fn generic_foo<T>(params : T) -> T
where
	T : SomeTrait
{
	// Function body
}
```

Any `T` that implements `SomeTrait` may be operated on by `generic_foo`.

# An Opinion

- Generators, filters, and mappings, when not syntactically cumbersome, are:
	+ Less prone to error due to code-reuse
	+ More readable than the alternative
	+ Faster (especially in interpreted languages like Python)

# Generate/Map and Filter all in one

- You've seen `filter_map()` and `flat_map()` before.
    + `filter_map` only includes `x` when your lambda returns `Some(x)` (Option)
    + `flat_map` only includes `x` when your lambda returnes `Ok(x)` (Result)
- So we can use `filter_map`/`flat_map` much like the following Python syntax.

```python
hasFactor6 = [2 * x for x in myVec if x % 3 == 0]
```
Now for the Rust:

```rust
let hasFactor6 = myVec.filter_map(|x| if x % 3 == 0 { Some(2 * x) } else { None }).collect();
```

# `fold` (and `reduce`)

- If you've used Scala, Haskell, Lisp, etc., you're familiar with the concept of `reduce` and `fold`.
- Rust has these, too!
    + `fold` is built in, as is `reduce`, as well as some variants such as `try_fold` and `try_reduce`

Some examples:
```rust
// `fold` requires an initial value
let n_factorial = (2..=n).fold(1, |acc, i| acc * i).unwrap();
// Whereas `reduce` does not (just assumes the first element in the list is initial)
let also_n_factorial = (2..=n).reduce(|acc, i| acc * i).unwrap();
// We can utilize try_fold's short-circuit'ing to see if a number is prime in a oneliner
let n_is_prime = (2..n).try_fold(0, |_acc, i| if n % i == 0 { None } else { Some(i) } ).is_some();
```

You may have seen this in Python if you are familiar with `functools`

```python
from functools import reduce
nFactorial = reduce(lambda acc, i : acc * i, range(n))
```

Technically, C++ allows the same with `std::accumulate` in `<numeric>`, but does not provide nice range iterators (until C++20):

```c++
#include <numeric>

// This is horrible code. Do not do this.
// It uses WAY too much auxiliary space to compute a factorial.
// Just use a for loop.
std::vector<uint32_t> nRange(n);
std::iota(nRange.begin(), nRange.end(), 1);
uint32_t nFactorial = std::accumulate(nRange.begin(), nRange.end(),
        [](uint32_t acc, uint32_t i) { return acc * i; });
        // You can also give std::multiplies<uint32_t>(). Because the C++ STL is overdefined.
```

But at this point we're abusing the C++ STL and should stop.

# Exercise

Using `fold` or `reduce`, reimpliment `join` on an arbitrary slice.

```rust
fn my_join(list: &[impl ToString], delimiter : &str) -> String {
    // your code here.
}
```

# A solution

```rust
fn my_join(list: &[impl ToString], delimiter : &str) -> String {
	list.iter().fold(String::new(), |mut acc, x| {
        if !acc.is_empty() {
			acc.push_str(delimiter);
		}
		acc.push_str(&x.to_string());
		acc
	})
}
```

# We've achieved with traits what in other languages we'd have to do via generics

```rust
// Generic
let numbers = vec![1, 2, 3, 4, 5];
let strings = vec!["hello", "world"];

println!("{}", my_join(&numbers, "-")); // Legal
println!("{}", my_join(&strings, "-")); // Legal
```

Extra credit, modify your code to join the following mixed-type vector:

```rust
let mixed : Vec<Box<dyn ToString>> = vec![
	Box::new(1), 
	Box::new("two"), 
	Box::new(3.14), 
	Box::new("four")]; // Both &str and u16, as well as f32 all implement ToString

```

# Extra Credit Solution

There's a good chance all you need to do is change your function signature:

```rust
fn my_join_alt(list: &[Box<dyn ToString>], delimiter : &str) -> String { ...
```

# Exercise

*Using only generators and functional tools*, create a list of the first `n` primes.

- No need for memoization right now

# A Solution

```rust
fn is_prime(n : u32) -> bool {
	// try_fold short circuites if it sees None
	(2..n).try_fold(0, |_acc, i| if n % i == 0 { None } else { Some(i) } ).is_some()
}

fn first_n_primes(n : u32) -> Vec<u32> {
	let mut last_prime = 0;
	(0..n).map(|_i| {
		last_prime += 1;
		while !is_prime(last_prime) {
			last_prime += 1;
		}
		last_prime
	}).collect::<Vec<u32>>()
}
```

# A Memoization Solution

```rust
// Create `memoize` automatically can memoize (side-effect free) functions
use memoize::memoize;

#[memoize] // All we need to do is add the `memoize` decorator
fn is_prime(n : u32) -> bool {
	// try_fold short circuites if it sees None
	(2..n).try_fold(0, |_acc, i| if n % i == 0 { None } else { Some(i) } ).is_some()
}

fn first_n_primes(n : u32) -> Vec<u32> {
	let mut last_prime = 0;
	(0..n).map(|_i| {
		last_prime += 1;
		while !is_prime(last_prime) {
			last_prime += 1;
		}
		last_prime
	}).collect::<Vec<u32>>()
}
```

# Another Exercise (Homework)

- Using the Prime Number Theorem and the Sieve of Eratosthenes, re-implement `first_n_primes`.

