use std::fmt::{Formatter, Error};
use crate::format::*;
use num_traits::cast::cast;
use crate::Storage;

pub fn print_storage<T, S>(s: &S, f: &mut Formatter) -> Result<(), Error>
	where T: Element, S: Storage<T>
{
	let use_sci_fmt = match T::numeric() {
		false => false,
		true => {
			if T::NumericType::is_complex() { true }
			else if T::NumericType::is_float() {
				s.as_iter().any(|x| {
					let n = T::NumericType::from(*x).as_scalar();
					n >= cast(100).unwrap() || n <= cast(-100).unwrap()
						|| (n < cast(0.0001).unwrap() && n >= cast(-0.0001).unwrap()
						&& n != cast(0.0).unwrap())
				})
			} else {
				s.as_iter().any(|x| {
					let n = T::NumericType::from(*x).as_scalar();
					n >= cast(100).unwrap() || n <= cast(-100).unwrap()
				})
			}
		}
	};

	let padding = if T::NumericType::is_complex() { 27 } else if use_sci_fmt { 13 } else { 8 };

	writeln!(
		f,
		"Storage[Type = {:#?}, {}, {}] => ",
		T::element_type(),
		s.size(),
		s.strides()
	)?;
	for i in 0..s.rows() {
		for e in s.as_row_range_iter(i) {
			write!(f, "{:>pad$}", format!("{}", Fmt(|f| e.fmt_elem(f, 4, use_sci_fmt))), pad = padding)?;
		}
		write!(f, "\n")?;
	}

	// TODO: fixed size scientific format
	// TODO: cut some rows on super large containers
	Ok(())
}