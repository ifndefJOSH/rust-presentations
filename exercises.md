# Rust Iterator Calisthenics

A little set of exercises to handle iteration in Rust. If you do any of this type of thing in production, you really should use a database. For these exercises, you are free to use `itertools`.

## Exercise 1

The following data types are present.

```rust
#[derive(Copy, Clone)]
struct StudentRecord {
	name: String,
	a_number: u16,
	classes: Vec<Class>,
	rank: ClassRank,
}

enum ClassRank {
	Freshman,
	Sophomore,
	Junior,
	Senior,
	Graduate,
}

#[derive(Copy, Clone)]
struct Class {
	credit_hours: u8,
	grade: String,
	name: String,
	crn: u16,
	class_num: u16,
	subject: String,
	
}

impl Class {
	fn grade_points(&self) -> Option<f32> {
		// Implement me
	}
}
```

### Part A

Implement the `grade_points` function in `Class`'s `impl` block. Use the following grade points (taken from `calculator.net/gpa-calculator.html`)

```
A+ = 4.3 grade points
A = 4 grade points
A- = 3.7 grade points
B+ = 3.3 grade points
B = 3 grade points
B- = 2.7 grade points
C+ = 2.3 grade points
C = 2 grade points
C- = 1.7 grade points
D+ = 1.3 grade points
D = 1 grade point
D- = 0.7 grade points
F = 0 grade points
P (pass), NP (not pass), I (incomplete), W (withdrawal) will be ignored.
```

Treat the existence of a valid grade as providing viable grade points, and the lack thereof of being a P, NP, I, W, or other grade, thus ignored. Multiply the grade points by the number of credit hours.

#### Part A - An Answer

```rust
fn grade_points(&self) -> Option<f32> {
	match self.grade.trim().to_uppercase() {
		"A+" => Some(4.3 * self.credit_hours),
		"A"  => Some(4.0 * self.credit_hours),
		"A-" => Some(3.7 * self.credit_hours),
		"B+" => Some(3.3 * self.credit_hours)
		"B"  => Some(3.0 * self.credit_hours),
		"B-" => Some(2.7 * self.credit_hours),
		"C+" => Some(2.3 * self.credit_hours),
		"C"  => Some(2.0 * self.credit_hours),
		"C-" => Some(1.7 * self.credit_hours),
		"D+" => Some(1.3 * self.credit_hours),
		"D-" => Some(0.7 * self.credit_hours),
		"F"  => Some(0.0 * self.credit_hours),
		_ => None // Treat these as P, NP, I, or W
	}
}
```

### Part B

You have either a `HashSet`, `BTreeSet`, or `Vector` (or something else that implements the `Iterator` trait) of `StudentRecord` entries. Let it be called `students`. Create an iterator that associates student records with their GPA. As in, each iterator should return a `(StudentRecord, f32)` tuple, where the `f32` represents the student's GPA. Collect that iterator into a vector.

```rust
pub fn get_gpas(students: &I) -> Vec<(StudentRecord, f32)>
where
	I: Iterator<Item = StudentRecord>
{
	// Implement me!
}
```

#### Part B - An Answer

```rust
impl StudentRecord {
	fn gpa(&self) -> f32 {
		self.classes
			.filter_map(|class| {
				// Only invoke grade_points once
				if let gp = Some(class.grade_points()) {
					Some(gp / (self.credit_hours as f32))
				} else {
					None
				}
			}) // use filter_map to ignore the Nones
			.sum() // Sum all of the grade points and divide by the total to get the gpa
	}
}

pub fn get_gpas(students: &I) -> Vec<(StudentRecord, f32)>
where
	I: Iterator<Item = StudentRecord>
{
	students.iter()
		.map(|student| {
			(student, student.gpa())
		}).collect::<Vec<_>>()
}
```

### Part C

*Without using any auxiliary space,* create a list of ONLY THE HONOR ROLL STUDENTS. Honor roll is defined as having a GPA at or above 3.5. Sort the list from highest to lowest GPA.

```rust
pub fn get_honor_roll(students: &I) -> Vec<(StudentRecord, f32)>
where
	I: Iterator<Item = StudentRecord>
{
	// Implement me!
}
```

#### Part C - An Answer

(Assume the added `impl` block for `StudentRecord` is still present)

```rust
pub fn get_honor_roll(students: &I) -> Vec<(StudentRecord, f32)>
where
	I: Iterator<Item = StudentRecord>
{
	let mut honor_roll = students.iter()
		.filter_map(|student| {
			let gpa = student.gpa();
			if gpa >= 3.5 {
				Some((student, gpa))
			} else {
				None
			}
		}).collect::<Vec<_>>();
	honor_roll.sort_by(|s1, s2| s2.1.cmp(s1.1));
	honor_roll
}
```

(Alternatively, you can implement the `Ord: Eq + PartialOrd` trait for `StudentRecord` and just use `sort()`)

### Part D

Create a list of students who should receive "special attention". There are two criteria that classify a student as needing special attention:

- A student is taking 16+ credit hours OR
- A student's GPA is below 2.0

Do not use any auxiliary storage.

```rust
pub fn get_attn_students(students: &I) -> Vec<StudentRecord>
where
	I: Iterator<Item = StudentRecord>
{
	// Implement me!
}
```

#### Part D - An Answer

```rust
pub fn get_attn_students(students: &I) -> Vec<StudentRecord>
where
	I: Iterator<Item = StudentRecord>
{
	students.iter()
		.filter(|student| {
			let total_credit_hours = student.classes
				.iter()
				.map(|class| class.credit_hours)
				.sum();
			student.gpa() < 2.0 || total_credit_hours >= 16
		}).collect::<Vec<_>>()
}
```

## Problem 2

This program you're writing is used on a server-side application used to render HTML for a portal for parents and students to view their grades, GPAs, etc. In order to do this, first create an `HtmlTable` struct that stores tabulated data and renders to an HTML string.

The table should be index-able by row and column, and you should be able to iterate over both rows and columns.

### Problem 2 - A solution

```rust
struct HtmlTable {
	rows_internal: Vec<String>,
}

struct Rows {
	parent: &HtmlTable,
}

struct Cols {
	parent: &HtmlTable,
}

impl HtmlTable {
	fn rows(&self) -> Rows {
		Rows{ parent: self }
	}
	fn cols(&self) -> Cols {
		Cols{ parent: self }
	}
}
```

TODO
