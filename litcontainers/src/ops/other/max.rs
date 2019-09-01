use crate::*;

#[derive(new)]
pub struct Maximum<L>
	where L: Operation
{
	left: L
}

impl<L> Operation for Maximum<L>
	where L: Operation, L::Result: InplaceForeach<L::Type>,
	      L::Type: Scalar
{
	type Type = L::Type;
	type Rows = U1;
	type Cols = U1;
	type Result = L::Type;

	fn apply(self) -> Self::Result {
		let mut ret = L::Type::min_val();
		self.left.apply().foreach(|v|
			if ret < *v {
				ret = *v
			}
		);
		ret
	}
}

#[derive(new)]
pub struct Minimum<L>
	where L: Operation
{
	left: L
}

impl<L> Operation for Minimum<L>
	where L: Operation, L::Result: InplaceForeach<L::Type>,
	      L::Type: Scalar
{
	type Type = L::Type;
	type Rows = U1;
	type Cols = U1;
	type Result = L::Type;

	fn apply(self) -> Self::Result {
		let mut ret = L::Type::max_val();
		self.left.apply().foreach(|v| if ret > *v { ret = *v});
		ret
	}
}

pub trait MaxOperations<T: Scalar>: Storage<T>
{
	fn maximum(&self) -> T { Maximum::new(BorrowedProvider::new(self)).apply() }

	fn minimum(&self) -> T { Minimum::new(BorrowedProvider::new(self)).apply() }
}

impl<T: Scalar, S: Storage<T>> MaxOperations<T> for S {}