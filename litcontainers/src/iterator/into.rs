pub trait IntoOrderedIterator<T> {
	type IntoIter: Iterator<Item=T>;

	fn into_ordered_iter(self) -> Self::IntoIter;
}

/*
pub trait IntoOrderedIterator<T> {
	type IntoIter: Iterator<Item=T>;

	fn into_ordered_iter(self) -> Self::IntoIter;
}*/
