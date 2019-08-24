
#[macro_export]
macro_rules! ichain_by_slices {
	($f: ident, $i: expr) => ($i);
	($f: ident, $i0: expr, $s1: expr $(, $s: expr)*) => {
		ichain_by_slices!($f, $i0.zip($s1).map(|(mut i0, s1)| i0.chain(s1.$f())) $(, $s)*)
	}
}

// TODO: these are rowwise joins. Can we make a add colwise variants?
#[macro_export]
macro_rules! join_cols {
	($v0: expr $(, $v: expr)*; $d: expr; $S: ident) => {
		{
			assert_eq!($d.value(), $v0.cols() $(+ $v.cols())*);
			let mut ret = $S::zeros(Size::new($v0.row_dim(), $d));
			let slices_extended = ichain_by_slices!(iter, $v0.as_row_slice_iter().map(|v0| v0.iter()) $(, $v.as_row_slice_iter())*);
			for (mut out_col, in_col_iter) in ret.as_row_slice_iter_mut().zip(slices_extended) {
				for (mut out_sample, in_sample) in out_col.as_iter_mut().zip(in_col_iter) {
					*out_sample = in_sample;
				}
			}
			ret
		}
	};
	($v0: expr $(, $v: expr)*; $d: expr) => (join_cols!($v0 $(, $v)*; $d; ContainerRM));
	($v0: expr $(, $v: expr)*) => (join_cols!($v0 $(, $v)*; Dynamic::new($v0.cols() $(+ $v.cols())*); ContainerRM));
}

#[macro_export]
macro_rules! join_rows {
	($v0: expr $(, $v: expr)*; $d: expr; $S: ident) => {
		{
			assert_eq!($d.value(), $v0.rows() $(+ $v.rows())*);
			let mut ret = $S::zeros(Size::new($d, $v0.col_dim()));
			let slices_extended = ichain_by_slices!(iter, $v0.as_col_slice_iter().map(|v0| v0.iter()) $(, $v.as_col_slice_iter())*);
			for (mut out_row, in_row_iter) in ret.as_col_slice_iter_mut().zip(slices_extended) {
				for (mut out_sample, in_sample) in out_row.as_iter_mut().zip(in_row_iter) {
					*out_sample = in_sample;
				}
			}
			ret
		}
	};
	($v0: expr $(, $v: expr)*; $d: expr) => (join_rows!($v0 $(, $v)*; $d; ContainerRM));
	($v0: expr $(, $v: expr)*) => (join_rows!($v0 $(, $v)*; Dynamic::new($v0.rows() $(+ $v.rows())*); ContainerRM));
}