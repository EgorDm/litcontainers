use crate::format::*;

pub trait Operation {
	type Type: Element;
	type Rows: Dim;
	type Cols: Dim;
	type Result;

	fn apply(self) -> Self::Result;
}