use litcontainers::*;
use rayon::prelude::*;

fn mock_container() -> ContainerRM<f64, U3, Dynamic> {
	ContainerRM::from_vec(Size::new(U3, Dynamic::new(2)), &[1., 2., 3., 4., 5., 6.])
}

#[test]
fn iter() {
	let mut s = mock_container();
	assert_eq!(s.as_row_range_iter(1).cloned().collect::<Vec<_>>(), vec![3., 4.]);
	assert_eq!(s.as_row_range_iter_mut(1).map(|x| *x).collect::<Vec<_>>(), vec![3., 4.]);
	assert_eq!(s.slice_rows(1..3).as_row_range_iter(0).cloned().collect::<Vec<_>>(), vec![3., 4.]);
	assert_eq!(s.slice_cols(1).as_row_range_iter(1).cloned().collect::<Vec<_>>(), vec![4.]);

	assert_eq!(s.as_col_range_iter(1).cloned().collect::<Vec<_>>(), vec![2., 4., 6.]);
	assert_eq!(s.as_col_range_iter_mut(1).map(|x| *x).collect::<Vec<_>>(), vec![2., 4., 6.]);
	assert_eq!(s.slice_rows(1..3).as_col_range_iter(1).cloned().collect::<Vec<_>>(), vec![4., 6.]);
	assert_eq!(s.slice_cols(1).as_col_range_iter(0).cloned().collect::<Vec<_>>(), vec![2., 4., 6.]);

	assert_eq!(s.as_row_range_iter(2).cloned().collect::<Vec<_>>(), vec![5., 6.]);
	assert_eq!(s.iter().collect::<Vec<_>>(), vec![1., 2., 3., 4., 5., 6.]);
}

#[test]
fn size() {
	let s = mock_container();
	assert_eq!(s.as_row_range_iter(1).len(), s.cols());
	assert_eq!(s.as_row_range_iter(0..2).len(), 2 * s.cols());
	assert_eq!(s.as_col_range_iter(1).len(), s.rows());
	assert_eq!(s.as_col_range_iter(0..2).len(), 2 * s.rows());
}

#[test]
fn splittable_iter() {
	let mut s = ContainerRM::from_vec(Size::new(U3, Dynamic::new(2)), &[1., 2., 3., 4., 5., 6.]);
	let slice = s.slice_rows(0..s.rows());
	let iter = slice.as_row_slice_iter();

	let (i1, i2) = iter.split_at(1);
	assert_eq!(i1.map(|sl| sl.iter()).flatten().collect::<Vec<_>>(), &[1., 2.]);
	assert_eq!(i2.map(|sl| sl.iter()).flatten().collect::<Vec<_>>(), &[3., 4., 5., 6.]);
}

#[test]
fn parallel_slice() {
	let s = ContainerRM::from_vec(Size::new(U3, Dynamic::new(2)), &[1, 2, 3, 4, 5, 6]);

	let res: i32 = s.as_row_slice_iter().into_par_iter()
		.map(|col| col.iter().sum::<i32>())
		.sum();

	assert_eq!(res, 21);
}

#[test]
fn inline_mutation() {
	let mut s = mock_container();

	s.mapv_inplace(|v| v * 2.);
	assert_eq!(s.as_slice(), &[2., 4., 6., 8., 10., 12.]);

	s.mapv_inplace_zip((&[2., 4., 6., 8., 10., 12.]).iter(), |a, b| a + b);
	assert_eq!(s.as_slice(), &[4., 8., 12., 16., 20., 24.]);

}

/*




*/

/*


#[test]
fn flip() {
	let s = mock_container();
	assert_eq!(s.flip_rows().as_slice(), [2., 1., 4., 3., 6., 5.]);
	assert_eq!(s.flip_cols().as_slice(), [5., 6., 3., 4., 1., 2.]);
}

#[test]
fn join() {
	let s1 = ContainerRM::from_vec(U2, Dynamic::new(2), &[1., 2., 3., 4.]);
	let s2 = ContainerRM::from_vec(U2, Dynamic::new(2), &[1., 2., 3., 4.]);

	let j1 = ContainerRM::zeros(s1.row_dim(), DimAdd::add(s1.col_dim(), s2.col_dim()));
	assert_eq!(j1.join_cols(&s1, &s2).as_slice(), [1., 2., 1., 2., 3., 4., 3., 4.]);

	let j2 = ContainerRM::zeros(DimAdd::add(s1.row_dim(), s2.row_dim()), s1.col_dim());
	assert_eq!(j2.join_rows(&s1, &s2).as_slice(), [1., 2., 3., 4., 1., 2., 3., 4.]);
}*/
