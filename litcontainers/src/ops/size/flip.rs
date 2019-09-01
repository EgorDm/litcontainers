use crate::{Element, Storage, StorageMut, Axis, AxisType, Container};

pub trait Flip<T: Element>: Storage<T> {
	fn flip(&self) -> Container<T, Self::OwnedType> {
		let mut ret = self.clone_owned();
		for (i, out_elem) in ret.as_iter_mut().enumerate() {
			*out_elem = self[self.len() - 1 - i]
		}
		ret
	}

	fn flip_axis<A: Axis<Self::Rows, Self::Cols>>(&self, _: A) -> Container<T, Self::OwnedType> {
		let mut ret = self.clone_owned();
		match A::axis_type() {
			AxisType::Row => {
				for (mut out_row, row) in ret.as_row_slice_iter_mut().zip(self.as_row_slice_iter()) {
					for (i, out_col) in out_row.as_iter_mut().enumerate() {
						*out_col = row.get(0, self.cols() - 1 - i);
					}
				}
			},
			AxisType::Col => {
				for (mut out_col, col) in ret.as_col_slice_iter_mut().zip(self.as_col_slice_iter()) {
					for (i, out_row) in out_col.as_iter_mut().enumerate() {
						*out_row = col.get(self.rows() - 1 - i, 0);
					}
				}
			},
		}
		ret
	}
}

impl<T: Element, S: Storage<T>> Flip<T> for S {}