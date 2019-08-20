use std::fmt::{Display, Formatter, Error};
use crate::format::*;
use num_traits::cast::cast;
use crate::Storage;

pub struct Fmt<F>(pub F) where F: Fn(&mut Formatter) -> Result<(), Error>;

impl<F> Display for Fmt<F>
	where F: Fn(&mut Formatter) -> Result<(), Error>
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		(self.0)(f)
	}
}

pub fn print_storage<T, S>(s: &S, f: &mut Formatter) -> Result<(), Error>
	where T: Scalar, S: Storage<T>
{
	let use_sci_fmt = if T::is_complex() {
		true
	} else if T::is_float() {
		s.as_iter().any(|x| {
			x.to_elementary() >= cast(100).unwrap() || x.to_elementary() <= cast(-100).unwrap()
				|| (x.to_elementary() < cast(0.0001).unwrap() && x.to_elementary() >= cast(-0.0001).unwrap()
				&& x.to_elementary() != cast(0.0).unwrap())
		})
	} else {
		s.as_iter().any(|x| x.to_elementary() >= cast(100).unwrap() || x.to_elementary() <= cast(-100).unwrap())
	};

	let padding = if T::is_complex() { 27 } else if use_sci_fmt { 13 } else { 8 };

	writeln!(
		f,
		"Storage[Type = {:#?}, Size = (Rows = {}, RowStride = {}, Cols = {}, ColStride = {})] => ",
		s.scalar_type(),
		Fmt(|f| s.row_dim().pfmt(f)),
		Fmt(|f| s.row_stride_dim().pfmt(f)),
		Fmt(|f| s.col_dim().pfmt(f)),
		Fmt(|f| s.col_stride_dim().pfmt(f)),
	)?;
	for i in 0..s.rows() {
		for e in s.as_row_range_iter(i) {
			write!(f, "{:>pad$}", format!("{}", Fmt(|f| e.fmt_num(f, 4, use_sci_fmt))), pad = padding)?;
		}
		write!(f, "\n")?;
	}

	// TODO: fixed size scientific format
	// TODO: cut some rows on super large containers
	Ok(())
}