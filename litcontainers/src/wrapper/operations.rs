use crate::format::*;
use crate::storage::*;
use crate::*;
use std::marker::PhantomData;


pub struct Mutable<T, S>
	where T: Scalar, S: Storage<T>
{
	data: S,
	_phantoms: PhantomData<(T)>
}

impl<T, S> Mutable<T, S>
	where T: Scalar, S: Storage<T>
{
	pub fn new(data: S) -> Self { Self { data, _phantoms: PhantomData } }
}

impl<'a, T, S> Operation for Mutable<T, S>
	where T: Scalar, S: Storage<T>
{
	type Type = T;
	type Rows = S::Rows;
	type Cols = S::Cols;
	type Result = S::OwnedType;

	fn apply(self) -> Self::Result {
		self.data.owned()
	}
}

/*
#[test]
fn test() {
	let aa = rvec_zeros![U10; f64];
	let op = AddScalar::new(Mutable::new(aa), 1f32);
	let res = op.apply();
	println!("{:#?}", res);
}
*/
