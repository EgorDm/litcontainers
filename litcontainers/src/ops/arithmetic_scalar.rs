use super::ops::*;
use std::ops::Add;
use crate::{InplaceMap, InplaceZipMap};

// TODO: 1 impl for refexive and 1 for non reflexive
#[derive(new)]
pub struct AddScalar<L, R>
	where L: Operation, R: Into<L::Type>
{
	left: L,
	right: R,
}

impl<L, R, S> Operation for AddScalar<L, R>
	where L: Operation<Result=S>, R: Into<L::Type>, S: InplaceMap<L::Type>, L::Type: Add<L::Type, Output=L::Type>
{
	type Type = L::Type;
	type Rows = L::Rows;
	type Cols = L::Cols;
	type Result = L::Result;

	fn apply(self) -> Self::Result {
		let r = self.right.into();
		let mut ret = self.left.apply();
		ret.mapv_inplace(|v| v.add(r.clone()));
		ret
	}
}

#[derive(new)]
pub struct AddOps<L, R>
	where L: Operation, R: Operation
{
	left: L,
	right: R,
}

impl<'a, L, R, S> Operation for AddOps<L, R>
	where
		L: Operation<Result=S>,
		R: Operation<Type=L::Type, Rows=L::Rows, Cols=L::Cols>,
		L::Type: Add<R::Type, Output=L::Type>,
		S: InplaceZipMap<L::Type, R::Type>
{
	type Type = L::Type;
	type Rows = L::Rows;
	type Cols = L::Cols;
	type Result = L::Result;

	fn apply(self) -> Self::Result {
		let mut ret = self.left.apply();
		//ret.mapv_inplace_zip(self.right.iter(), |v, u| v.add(u));
		ret
	}
}