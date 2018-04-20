// Discussion 10 exercises, due 4/13/2018

// HOW TO SUBMIT:
// Upload lib.rs (that's it) via web submission.

// IF YOU ARE USING GRACE: You can download lib.rs on
// https://dav.terpconnect.umd.edu

// YOU CANNOT USE "ruby submit.rb" because I wasn't able to get it to
// work in time. Hopefully it will work for next week's discussion and
// the project.

// Returns the sum of the even integers in the range [i, j).
// sum_evens(0, 6) -> 6 (0 + 2 + 4)
pub fn sum_evens(i: i32, j: i32) -> i32 {
    let mut sum = 0;
    
    for x in i..j {
    	if (x % 2) == 0 {
    		sum += x
    	}
    }
    
    sum
}

// Returns the Euclidean distance between 2-dimensional points a and b.
// The points are represented as 2-tuples of f64s.
// distance((0.0, 0.0), (1.0, 1.0) -> 1.41...
pub fn distance((ax, ay): (f64, f64), (bx, by): (f64, f64)) -> f64 {
    ((ax - bx).powi(2) + (ay - by).powi(2)).sqrt()
}

// Returns the sum of the squared elements of arr.
// sum_squares(&[1, 2] -> 5 (1^2 + 2^2)
pub fn sum_squares(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for x in arr.iter() {
    	sum += x.pow(2);
    }
    
    sum
}

// Adds 1 to each element of the array. (Mutates the array.)
// let mut arr: [i32; 3] = [0, 1, 2];
// raise_1(&mut arr)
// (arr is now [1, 2, 3])
pub fn raise_1(arr: &mut [i32]) {
	for x in 0..arr.len() {
		arr[x] += 1;
	}
}

// CHALLENGE PROBLEM (UNGRADED)

// Returns the max consecutive 1s in the array.
// consecutive_1s(&[1, 1, 1, 0, 1, 1]) -> 3
pub fn consecutive_1s(arr: &[i32]) -> i32 {
    let mut cur = 0;
   	let mut max = 0;
   	
   	for x in arr.iter() {
   		if *x == 1 {
   			cur += 1;
   			max = max.max(cur);
   		} else {
   			cur = 0;
   		}
   	}
   	
   	max
}

#[cfg(test)]
mod public;

