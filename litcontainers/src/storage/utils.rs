pub trait InplaceMap<T: Clone> {
	fn map_inplace<F: FnMut(&mut T)>(&mut self, f: F);

	fn mapv_inplace<F: FnMut(T) -> T>(&mut self, mut f: F) {
		self.map_inplace(|v| *v = f(v.clone()))
	}

	fn map_inplace_zip<U, F: FnMut(&mut T, U) -> T, I: Iterator<Item=U>>(&mut self, mut i: I, mut f: F) {
		self.map_inplace(|v| *v = f(v, i.next().unwrap()))
	}

	fn mapv_inplace_zip<U, F: FnMut(T, U) -> T, I: Iterator<Item=U>>(&mut self, mut i: I, mut f: F) {
		self.map_inplace(|v| *v = f(v.clone(), i.next().unwrap()))
	}
}

pub trait InplaceMapOrdered<T: Clone> {
	fn map_inplace_ordered<F: FnMut(&mut T)>(&mut self, f: F);

	fn mapv_inplace_ordered<F: FnMut(T) -> T>(&mut self, mut f: F) {
		self.map_inplace_ordered(|v| *v = f(v.clone()))
	}

	fn map_inplace_zip_ordered<U, F: FnMut(&mut T, U), I: Iterator<Item=U>>(&mut self, mut i: I, mut f: F) {
		self.map_inplace_ordered(|v| f(v, i.next().unwrap()))
	}

	fn mapv_inplace_zip_ordered<U, F: FnMut(T, U) -> T, I: Iterator<Item=U>>(&mut self, mut i: I, mut f: F) {
		self.map_inplace_ordered(|v| *v = f(v.clone(), i.next().unwrap()))
	}
}

pub trait InplaceForeach<T: Clone> {
	fn foreach<F: FnMut(&T)>(&self, f: F);

	fn foreach_zip<U, F: FnMut(&T, U), I: Iterator<Item=U>>(&self, mut i: I, mut f: F) {
		self.foreach(|v| f(v, i.next().unwrap()))
	}
}