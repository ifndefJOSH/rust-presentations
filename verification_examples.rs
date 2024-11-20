use prusti_contracts::*;

#[extern_spec(std::mem)]
#[ensures(*a === old(snap(b)) && *b === old(snap(a)))]
fn swap<T>(a: &mut T, b: &mut T);

// Adapted from https://dafny.org/latest/OnlineTutorial/guide

// Add applicable pre and post conditions
fn abs(x: i32) -> i32 {
	if x < 0 { -x } else { x }
}

fn test_abs() {
	let v = abs(3);
	// What's the difference between these two?
	prusti_assert!(0 <= v);
	assert!(0 <= v);
}

// Add pre and post conditions
fn maximum(values: Vec<i32>) -> i32 {
	let mut max = values.get(0).unwrap();
	let n = values.len();
	for i in 0..n {
		// Add a loop invariant
		let cur_val = values.get(i).unwrap();
		if max < cur_val {
			max = cur_val;
		}
	}
	*max
}

predicate!(
	fn maximum_is_unique(mx: i32, values: Vec<i32>) -> bool {
		// fill this out for maximum_is_unique
		true
	}
);

// Add pre and postconditions
fn fib(n: u32) -> u32 {
	if n == 0 { 0 }
	else if n == 1 { 1 }
	else { fib(n - 1) + fib(n - 2) }
}

#[ensures(0 <= result ==> result > a.len() && a[result] == key)]
fn find(a: Vec<i32>, key: i32) -> usize {
	// Try to write a function body that satisfies the postcondition
	0 as usize
}

// Try to extend `find` to be generic for any type, using the Eq and PartialEq traits

// try to annotate this with pre and post conditions
fn find_max(a: Vec<i32>) -> usize {
	// Try to write a function body that satisfies your postconditions
	0 as usize
}

// Extend this function with the PartialEq and PartialOrd traits. See if your pre and
// postconditions are still valid/will they still hold?

// Write pre and post conditions
fn sort<T>(a: &[T]) 
where
	T: PartialEq + PartialOrd,
{
	// Implement me
	// Hint: use std::mem::swap
}

predicate!(
	fn sorted<T>(a: &[T]) -> bool 
	where
		T: PartialEq + PartialOrd,
	{
		// Implement this predicate
		true
	}
);

#[requires(0 <= a.len() && sorted(&a))]
#[ensures(result.is_some() ==> result.unwrap() < a.len() && a[result.unwrap()] == val)]
#[ensures(result.is_none() ==> forall(|k: usize| 0 <= k && k < a.len() ==> a[k] != val))]
fn binary_search<T>(a: Vec<T>, val: T) -> Option<usize>
where
	T: PartialEq + PartialOrd,
{
	// What should we put here?
	None
}
