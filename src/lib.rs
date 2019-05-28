#[derive(Debug)]
pub struct Heap<T> {
	array: Vec<T>,
	size: usize,
}

impl<T> Heap<T> 
	where T: std::cmp::PartialOrd {
	
	pub fn peek_max(&self) -> &T {
		&self.array[0]
	}

	pub fn pop_max(&mut self) -> Option<T> {
		self.array.swap(0, self.size-1);

		self.size = self.size - 1;
		self.max_heapify(0);

		self.array.pop()
	}

	fn _parent(i: usize) -> usize {
		(i as f64 / 2.0).floor() as usize
	}

	fn left(i: usize) -> usize {
		2*i
	}

	fn right(i: usize) -> usize {
		2*i+1
	}

	fn max_heapify(&mut self, i: usize) {
		let l = Heap::<T>::left(i);
		let r = Heap::<T>::right(i);
		let mut largest = 
			if l < self.size && self.array[l] >= self.array[i] {
			l	
		} else {
			i
		};
		 
		if r < self.size && self.array[r] >= self.array[largest] {
			largest = r;
		}

		if largest != i {
			self.array.swap(i, largest);

			self.max_heapify(largest);
		}

	}

	pub fn build_max_heap(array: Vec<T>) -> Heap<T> {
		let size = array.len();
		let mut h = Heap {array, size};
		let upper = size / 2;
		for i in (0..upper).rev() {
			h.max_heapify(i);
		}
		h
	}

	pub fn heapsort(mut self) -> Vec<T> {
		for i in (1..self.size).rev() {
			self.array.swap(0, i);
			self.size = self.size - 1;
			self.max_heapify(0);
		}
		self.array	
	}
}


#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn build_heap_and_peek() {
    	let v = vec![19, 34, 132, 88, 1, 98];
    	let vh = Heap::build_max_heap(v);

    	assert_eq!(vh.peek_max(), &132);
    	assert_eq!(vh.peek_max(), &132);
    }

    #[test]
    fn pop_max() {
    	let v = vec![19, 34, 132, 88, 1, 98];
    	let mut vh = Heap::build_max_heap(v);

    	assert_eq!(vh.pop_max().unwrap(), 132);
    	assert_eq!(vh.pop_max().unwrap(), 98);
    	assert_eq!(vh.pop_max().unwrap(), 88);
    }

    #[test]
    fn heapsort() {
		let v = vec![19, 34, 132, 88, 1, 98];
    	let vh = Heap::build_max_heap(v);

    	assert_eq!(vh.heapsort(), vec![1, 19, 34, 88, 98, 132]);    	
    }
}
