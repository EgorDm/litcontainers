use crate::format::*;

pub trait Operation {
	type Type: Element;
	type Rows: Dim;
	type Cols: Dim;
	type Result;

	fn apply(self) -> Self::Result;
}

pub trait IntoOperation {
	type OpType: Operation;

	fn into_operation(self) -> Self::OpType;
}


/*
Into<O::Result>
impl<O> Into<O> for O::Result
	where O: Operation {
	fn into(self) -> <O as Operation>::Result { self.apply() }
}*/
