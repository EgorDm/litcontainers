use crate::format::*;

pub trait Operation {
	type Type: Element;
	type Rows: Dim;
	type Cols: Dim;
	type Result;

	fn apply(self) -> Self::Result;
}

pub trait InplaceMap<T: Clone> {
	fn map_inplace<F: FnMut(&mut T)>(&mut self, f: F);

	fn mapv_inplace<F: FnMut(T) -> T>(&mut self, mut f: F) {
		self.map_inplace(|v| *v = f(v.clone()))
	}
}

pub trait InplaceZipMap<T: Clone, U> {
	fn map_inplace_zip<F: FnMut(&mut T, U), I: Iterator<Item=U>>(&mut self, i: I, f: F);

	fn mapv_inplace_zip<F: FnMut(T, U) -> T, I: Iterator<Item=U>>(&mut self, i: I, mut f: F)  {
		self.map_inplace_zip(i, |v, u| *v = f(v.clone(), u))
	}
}