// Discussion 11 exercises, due 4/20/2018

// HOW TO SUBMIT:
// Upload lib.rs via web submission.

// IF YOU ARE USING GRACE: You can download lib.rs on
// https://dav.terpconnect.umd.edu

use std::collections::HashMap;
use std::ops::Add;

// Returns all substrings (as slices), excluding the empty string.
// substrings("abc") -> ["a", "ab", "abc", "b", "bc", "c"]

pub fn substrings(s: &str) -> Vec<&str> {
	let mut vec = Vec::new();
	let len = s.len();
	for i in 0..len {
	  	for j in (i+1)..(len+1) {
	  		vec.push(s.get(i..j).unwrap());
	  	}
	}
	vec
}

// Sorts the array w/ the quicksort algorithm. partition is given.

fn partition(arr: &mut [i32]) -> usize {
	let len = arr.len() - 1;
    let p = arr[len];
    let mut i: usize = 0;
    for j in i..len {
        if arr[j] < p {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, len);
    i
}

pub fn quicksort(arr: &mut [i32]) {
	if arr.len() >= 1 {
		let p = partition(arr);
		let rest : &mut [i32] = &mut arr[(p+1)..];
		quicksort(rest);
	}
}

// Given 2 word counters (hashmaps which map strings to integers) a and b, get the total count of &str k across both counters.
// if a is { "a": 4, "b": 2 } and b is { "b": 2 }...
// get_from_both(&a, &b, "a") -> 4
// get_from_both(&a, &b, "b") -> 4
// get_from_both(&a, &b, "c") -> 0

pub fn get_from_both(a: &HashMap<&str, i32>, b: &HashMap<&str, i32>, k: &str) -> i32 {
  let mut i = 0;
  for (key, val) in a {
  	if key == &k {
  		i += val;
  	}
  }
  for (key, val) in b {
  	if key == &k {
  		i += val;
  	}
  }
  i
}

// Overload operator + for Vec2.
// Vec2 { x: 1., y: 2. } + Vec2 { x: 3., y: 4. } -> Vec2 { x: 4., y: 6. }

#[derive (Debug, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
        	x: self.x + other.x,
        	y: self.y + other.y
        }
    }
}

#[cfg(test)]
mod public;
