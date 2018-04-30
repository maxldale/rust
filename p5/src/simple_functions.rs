//! Part 1 - Warmup
//! These are functions to help familiarize yourself with the Rust language.  You have already implemented some of these in
//! your Ocaml projects.  Feel free to copy the logic from your Ocaml functions to Rust.
//!
//! Keep in mind that Rust Vectors, Slices and Arrays cannot be matched in the same way as Ocaml
//! lists, so some things may not translate directly.


/// Returns the value of the Ackermann–Péter function for m and n.
pub fn ack(m: i32, n: i32) -> i32 {
    if m == 0 {
    	return n + 1;
    } else if m > 0 && n == 0 {
    	return ack(m - 1, 1);
    } else {
    	return ack(m - 1, ack(m, n - 1));
    }
}

/// Returns the sum of the values from index 0 to index i (0-indexed) of lst inclusive. If i is
/// greater than greatest index of list, return the sum of all elements of list. 
///
/// It might be helpful to use the enumerate method of the Iterator trait.
pub fn partial_sum(i: usize, is: &[i32]) -> i32 {
	let mut counter = i + 1;
	let mut sum = 0;
    for ele in is.iter() {
    	if counter > 0 {
    		sum += ele;
    		counter -= 1;
    	} else {
    		sum += 0;
    	}
    }
    sum
}

/// Returns the mean of elements in lst. If the list is empty, return None. 
///
/// It might be helpful to use the fold method of the Iterator trait.
pub fn mean(lst: &[f64]) -> Option<f64> {
	let mut counter = 0;
	let mut sum = 0_f64;
	for ele in lst.iter() {
		sum += ele;
		counter += 1;
	}
	
	if counter == 0 {
		None
	} else {
		let div = counter as f64;
		let res = sum / div;
		Some(res)
	}
}

/// Performs a simple binary search, returning the index of the element if it was found, or None if not found
pub fn bsearch<T: Eq + PartialOrd> (lst: &[T], ele: &T) -> Option<usize> {
	let len = lst.len();
	if len == 0 {
		None
	} else {
    	let mid = (len - 1) / 2;
    	let mid_ele = &lst[mid];
    	if ele == mid_ele {
    		Some(mid)
    	} else if ele < mid_ele {
    		if mid == 0 {
    			None
    		} else {
    			bsearch(&lst[..mid], ele)
    		}
    	} else {
    		let res = bsearch(&lst[(mid + 1)..], ele);
    		match res {
    			Some( index ) => Some(index + 1 + mid),
    			None => None,
    		}
    	}
    }
}

/// We have a CSV of course information. The CSV has many lines that are formatted as "<Deptartment>,<CourseNumber>,<RoomNumber>,<DaysOfWeek>".
/// A str containing all of the CSV file, Calculate the number of courses being offered in each department, in each room, and at each time. 
/// Write the function get_course_data that does these calculations, and stores the results in three hashes that map departments, rooms, and times to number of occurrences.
/// Return these hashes in a tuple
///
/// For context, here is how the function will be used:
/// use std::io::Read;
/// use std::fs::File;
/// use std::path::Path;
/// fn process_csv<P: AsRef<Path>>(filename: P) -> () {
///     let mut file = File::open(filename).unwrap();
///     let mut buf = String::new();
///     file.read_to_string(&mut buf);
///     let (depthash, roomhash, timehash) = get_course_data(buf.as_str());
///     // run some tests >:)
/// }

use std::collections::HashMap;
pub fn get_course_data(csv_data: &str) -> (HashMap<&str,usize>,HashMap<&str,usize>,HashMap<&str,usize>)  {
	let mut dept_hash = HashMap::new();
	let mut room_hash = HashMap::new();
	let mut time_hash = HashMap::new();
	let lines = csv_data.split("\n");
	for line in lines {
		if line == "" {
			()
		} else {
			let mut data = line.split(",");
			match data.next() {
				Some (dept) => {
					let count = dept_hash.entry(dept).or_insert(0);
					*count += 1;
				},
				None => (),
			};
			data.next();
			match data.next() {
				Some (room) => {
					let count = room_hash.entry(room).or_insert(0);
					*count += 1;
				},
				None => (),
			};
			match data.next() {
				Some (days) => {
					let count = time_hash.entry(days).or_insert(0);
					*count += 1;
				},
				None => (),
			};
		}
	}
	(dept_hash, room_hash, time_hash)
}
