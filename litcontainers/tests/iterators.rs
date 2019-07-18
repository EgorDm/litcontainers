use litcontainers::*;
use rayon::prelude::*;

fn mock_container() -> ContainerRM<f64, U3, Dynamic> {
	ContainerRM::from_vec(U3, Dynamic::new(2), &[1., 2., 3., 4., 5., 6.])
}

#[test]
fn iter() {
	let mut s = mock_container();
	assert_eq!(s.slice_rows_as_iter(1).cloned().collect::<Vec<_>>(), vec![3., 4.]);
	assert_eq!(s.slice_rows_as_mut_iter(1).map(|x| *x).collect::<Vec<_>>(), vec![3., 4.]);
	assert_eq!(s.slice_rows(1..3).slice_rows_as_iter(0).cloned().collect::<Vec<_>>(), vec![3., 4.]);
	assert_eq!(s.slice_cols(1).slice_rows_as_iter(1).cloned().collect::<Vec<_>>(), vec![4.]);

	assert_eq!(s.slice_cols_as_iter(1).cloned().collect::<Vec<_>>(), vec![2., 4., 6.]);
	assert_eq!(s.slice_cols_as_mut_iter(1).map(|x| *x).collect::<Vec<_>>(), vec![2., 4., 6.]);
	assert_eq!(s.slice_rows(1..3).slice_cols_as_iter(1).cloned().collect::<Vec<_>>(), vec![4., 6.]);
	assert_eq!(s.slice_cols(1).slice_cols_as_iter(0).cloned().collect::<Vec<_>>(), vec![2., 4., 6.]);

	assert_eq!(s.slice_rows_as_iter(2).cloned().collect::<Vec<_>>(), vec![5., 6.]);
	assert_eq!(s.as_iter().cloned().collect::<Vec<_>>(), vec![1., 2., 3., 4., 5., 6.]);
}

#[test]
fn size() {
	let s = mock_container();
	assert_eq!(s.slice_rows_as_iter(1).len(), s.col_count());
	assert_eq!(s.slice_rows_as_iter(0..2).len(), 2 * s.col_count());
	assert_eq!(s.slice_cols_as_iter(1).len(), s.row_count());
	assert_eq!(s.slice_cols_as_iter(0..2).len(), 2 * s.row_count());
}

#[test]
fn ops() {
	let s = ContainerRM::from_vec(U3, Dynamic::new(2), &[1., 2., 3., 4., 5., 6.]);
	let s1 = ContainerCM::from_vec(U3, Dynamic::new(2), &[1., 2., 3., 4., 5., 6.]);

	assert_eq!((&s + &s1).as_slice(), [2., 4., 6., 8., 10., 12.]);
	assert_eq!((&s * &s1).as_slice(), [1., 4., 9., 16., 25., 36.]);

	{
		let mut s = s.clone_owned();
		s += &s1;
		assert_eq!(s.as_slice(), [2., 4., 6., 8., 10., 12.]);
	}

	assert_eq!((s.slice_rows(0..3) + &s1).as_slice(), [2., 4., 6., 8., 10., 12.]);
	let s2 = s1.slice_rows(0..3);
	assert_eq!((s.slice_rows(0..3) + &s2).as_slice(), [2., 4., 6., 8., 10., 12.]);

	assert_eq!((&s + 1.).as_slice(), [2., 3., 4., 5., 6., 7.]);
	assert_eq!((-&s).as_slice(), [-1., -2., -3., -4., -5., -6.]);
}

#[test]
fn ops_sci() {
	let s = ContainerRM::from_vec(U3, Dynamic::new(2), &[1., 2., 3., 4., 5., 6.]);

	assert_eq!((&s - 0.1).ceil().as_slice(), [1., 2., 3., 4., 5., 6.]);
	assert_eq!((&s - 0.1).floor().as_slice(), [0., 1., 2., 3., 4., 5.]);
	assert_eq!((&s).max(2.).as_slice(), [2., 2., 3., 4., 5., 6.]);
	assert_eq!((&s).pow(2).as_slice(), [1., 4., 9., 16., 25., 36.]);
}

#[test]
fn splittable_iter() {
	let mut s = ContainerRM::from_vec(U3, Dynamic::new(2), &[1., 2., 3., 4., 5., 6.]);
	let slice = s.slice_rows_mut(0..s.row_count());
	let iter = RowSliceIterSplitMut::new(slice);

	let (i1, i2) = iter.split_at(1);
	assert_eq!(i1.map(|sl| sl.iter()).flatten().collect::<Vec<_>>(), &[1., 2.]);
	assert_eq!(i2.map(|sl| sl.iter()).flatten().collect::<Vec<_>>(), &[3., 4., 5., 6.]);
}

#[test]
fn parallel_slice() {
	let mut s = ContainerRM::from_vec(U3, Dynamic::new(2), &[1., 2., 3., 4., 5., 6.]);
	let slice = s.slice_rows_mut(0..s.row_count());
	let iter = RowSliceIterSplitMut::new(slice);

	let res: f64 = iter.into_par_iter()
		.map(|col| col.sum())
		.sum();

	assert_eq!(res, 21.);
}

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
}