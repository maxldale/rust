//! Part 4 - traits
//!
//! Below, we have defined a trait for a queue data structure.  Implement this trait for Vec<T>

pub trait Queue<T> {
    // Add an element to the back of the queue
    fn enqueue(&mut self, ele: T) -> ();
    // Get the element from the front of the queue without removing it.  If the queue is empty, return None
    fn peek(&self) -> Option<&T>;
    // Get the element from the front of the queue and removes it.  If the queue is empty, return None
    fn poll(&mut self) -> Option<T>;
}



impl <T> Queue<T> for Vec<T> {

    fn enqueue(&mut self, ele: T) -> () {
        self.push(ele)
    }

    fn peek(&self) -> Option<&T> {
    	if self.len() == 0 {
    		None
    	} else {
    		let first: &T = &self[0];
        	Some(first)
        }
    }

    fn poll(&mut self) -> Option<T> {
        if self.len() == 0 {
        	None
        } else {
        	let first = self.remove(0);
        	Some(first)
        }
    }
}
