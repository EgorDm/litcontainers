use std::fmt::{Display, Formatter, Error};
use crate::{Scalar, Storage, Dim};

pub struct Fmt<F>(pub F) where F: Fn(&mut Formatter) -> Result<(), Error>;

impl<F> Display for Fmt<F>
	where F: Fn(&mut Formatter) -> Result<(), Error>
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		(self.0)(f)
	}
}

pub fn print_storage<T, R, C, S>(s: &S, f: &mut Formatter) -> Result<(), Error>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	writeln!(f, "Storage[Type = {:#?}, Size = ({})] => ",
	       s.scalar_type(),
	       format!("Rows = {}, RowStride = {}, Cols = {}, ColStride = {}",
	               Fmt(|f| s.row_dim().pfmt(f)),
	               Fmt(|f| s.row_stride_dim().pfmt(f)),
	               Fmt(|f| s.col_dim().pfmt(f)),
	               Fmt(|f| s.col_stride_dim().pfmt(f)),
	       )
	)?;
	for i in 0..s.row_count() {
		for e in s.as_row_slice_iter(i) {
			write!(f, " {:.4?}", e);
		}
		write!(f, "\n");
	}

	Ok(())
}