use litcontainers::*;
use rayon::prelude::*;

fn mock_container() -> ContainerRM<f64, U3, Dynamic> {
	ContainerRM::from_vec(U3, Dynamic::new(2), vec![1., 2., 3., 4., 5., 6.])
}

#[test]
fn iter() {
	let mut s = mock_container();
	assert_eq!(s.slice_as_row_iter(1).cloned().collect::<Vec<_>>(), vec![3., 4.]);
	assert_eq!(s.slice_as_row_mut_iter(1).map(|x| *x).collect::<Vec<_>>(), vec![3., 4.]);
	assert_eq!(s.slice_rows(1..3).slice_as_row_iter(0).cloned().collect::<Vec<_>>(), vec![3., 4.]);
	assert_eq!(s.slice_cols(1).slice_as_row_iter(1).cloned().collect::<Vec<_>>(), vec![4.]);

	assert_eq!(s.slice_as_col_iter(1).cloned().collect::<Vec<_>>(), vec![2., 4., 6.]);
	assert_eq!(s.slice_as_col_mut_iter(1).map(|x| *x).collect::<Vec<_>>(), vec![2., 4., 6.]);
	assert_eq!(s.slice_rows(1..3).slice_as_col_iter(1).cloned().collect::<Vec<_>>(), vec![4., 6.]);
	assert_eq!(s.slice_cols(1).slice_as_col_iter(0).cloned().collect::<Vec<_>>(), vec![2., 4., 6.]);

	assert_eq!(s.slice_as_row_iter(2).cloned().collect::<Vec<_>>(), vec![5., 6.]);
	assert_eq!(s.as_iter().cloned().collect::<Vec<_>>(), vec![1., 2., 3., 4., 5., 6.]);
}

#[test]
fn size() {
	let s = mock_container();
	assert_eq!(s.slice_as_row_iter(1).len(), s.col_count());
	assert_eq!(s.slice_as_row_iter(0..2).len(), 2 * s.col_count());
	assert_eq!(s.slice_as_col_iter(1).len(), s.row_count());
	assert_eq!(s.slice_as_col_iter(0..2).len(), 2 * s.row_count());
}

#[test]
fn ops() {
	let s = ContainerRM::from_vec(U3, Dynamic::new(2), vec![1., 2., 3., 4., 5., 6.]);
	let s1 = ContainerCM::from_vec(U3, Dynamic::new(2), vec![1., 2., 3., 4., 5., 6.]);

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
	let s = ContainerRM::from_vec(U3, Dynamic::new(2), vec![1., 2., 3., 4., 5., 6.]);

	assert_eq!((&s - 0.1).ceil().as_slice(), [1., 2., 3., 4., 5., 6.]);
	assert_eq!((&s - 0.1).floor().as_slice(), [0., 1., 2., 3., 4., 5.]);
	assert_eq!((&s).max(2.).as_slice(), [2., 2., 3., 4., 5., 6.]);
	assert_eq!((&s).pow(2).as_slice(), [1., 4., 9., 16., 25., 36.]);
}



#[test]
fn splittable_iter() {
	let mut s = ContainerRM::from_vec(U3, Dynamic::new(2), vec![1., 2., 3., 4., 5., 6.]);
	let slice = s.slice_rows_mut(0..s.row_count());
	let iter = RowSliceIterSplitMut::new(slice);

	let (i1, i2) = iter.split_at(1);

	for part in i1 {
		println!("{}", part);
	}

	println!("Split");

	for part in i2 {
		println!("{}", part);
	}
}


#[test]
fn parallel_slice() {
	let mut s = ContainerRM::from_vec(U3, Dynamic::new(2), vec![1., 2., 3., 4., 5., 6.]);
	let slice = s.slice_rows_mut(0..s.row_count());
	let iter = RowSliceIterSplitMut::new(slice);

	let res: f64 = iter.into_par_iter()
		.map(|col| col.sum())
		.sum();

	assert_eq!(res, 21.);
}