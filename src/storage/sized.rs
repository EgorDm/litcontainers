use crate::format::*;

pub trait SizedStorage<R, C>
	where R: Dim, C: Dim
{
	#[inline]
	fn row_dim(&self) -> R;

	#[inline]
	fn row_count(&self) -> usize { self.row_dim().value() }

	#[inline]
	fn col_dim(&self) -> C;

	#[inline]
	fn col_count(&self) -> usize { self.col_dim().value() }

	#[inline]
	fn equal_size<RO, CO, SO>(&self, other: &SO) -> bool
		where RO: Dim, CO: Dim, SO: SizedStorage<RO, CO>
	{
		self.row_count() == other.row_count() && self.col_count() == other.col_count()
	}
}