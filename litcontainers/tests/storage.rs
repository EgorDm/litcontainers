use litcontainers::*;

fn mock_container() -> ContainerRM<f64, U3, Dynamic> {
	ContainerRM::from_vec(Size::new(U3, Dynamic::new(2)), &[1., 2., 3., 4., 5., 6.])
}

#[test]
fn sized() {
	let s = mock_container();
	let s2 = mock_container();

	assert_eq!(s.rows(), 3);
	assert_eq!(s.cols(), 2);
	assert_eq!(s.row_stride(), s.cols());
	assert_eq!(s.col_stride(), 1);
	assert!(s.equal_size(&s2));
}

#[test]
fn indexing() {
	let s = mock_container();

	assert_eq!(s.index(1, 1), 3);
	assert_eq!(s.row_index(2), 4);
	assert_eq!(s.col_index(2), 2);
	assert_eq!(s.get(1, 1), 4.);
	assert_eq!(*s.get_ref(1, 1), 4.);
}

#[test]
fn resizing_upsize() {
	let mut s = ContainerRM::from_vec(Size::new(D!(2), D!(2)), &[1., 2., 3., 4.]);
	s.set_cols(3);
	assert_eq!(s.cols(), 3);
	assert_eq!(s.as_slice(), [1., 2., 0., 3., 4., 0.]);

	let mut s = ContainerRM::from_vec(Size::new(D!(2), D!(2)), &[1., 2., 3., 4.]);
	s.set_rows(3);
	assert_eq!(s.rows(), 3);
	assert_eq!(s.as_slice(), [1., 2., 3., 4., 0., 0.]);

	let mut s = ContainerCM::from_vec(Size::new(D!(2), D!(2)), &[1., 2., 3., 4.]);
	s.set_cols(3);
	assert_eq!(s.cols(), 3);
	assert_eq!(s.as_slice(), [1., 3., 2., 4., 0., 0.]);

	let mut s = ContainerCM::from_vec(Size::new(D!(2), D!(2)), &[1., 2., 3., 4.]);
	s.set_rows(3);
	assert_eq!(s.rows(), 3);
	assert_eq!(s.as_slice(), [1., 3., 0., 2., 4., 0.]);
}


#[test]
fn resizing_downsize() {
	let mut s = ContainerRM::from_vec(Size::new(D!(2), D!(2)), &[1., 2., 3., 4.]);
	s.set_cols(1);
	assert_eq!(s.cols(), 1);
	assert_eq!(s.as_slice(), [1., 3.]);

	let mut s = ContainerRM::from_vec(Size::new(D!(2), D!(2)), &[1., 2., 3., 4.]);
	s.set_rows(1);
	assert_eq!(s.rows(), 1);
	assert_eq!(s.as_slice(), [1., 2.]);

	let mut s = ContainerCM::from_vec(Size::new(D!(2), D!(2)), &[1., 2., 3., 4.]);
	s.set_cols(1);
	assert_eq!(s.cols(), 1);
	assert_eq!(s.as_slice(), [1., 3.]);

	let mut s = ContainerCM::from_vec(Size::new(D!(2), D!(2)), &[1., 2., 3., 4.]);
	s.set_rows(1);
	assert_eq!(s.rows(), 1);
	assert_eq!(s.as_slice(), [1., 2.]);
}

#[test]
fn mutable() {
	let mut s = mock_container();

	*s.get_mut(1, 1) = 1337.;
	assert_eq!(s.get(1, 1), 1337.);
}

/*

#[test]
fn slice() {
	let s = mock_container();
	assert_eq!(s.slice_rows(1..3).rows(), 2);
	assert_eq!(s.slice_rows(1..3).as_slice(), [3., 4., 5., 6.]);
	assert_eq!(s.slice_cols(1).cols(), 1);
	assert_eq!(s.slice_cols(1).as_iter().cloned().collect::<Vec<_>>(), vec![2., 4., 6.]);
	assert_eq!(s.slice_rows(1..3).slice_cols(1).as_iter().cloned().collect::<Vec<_>>(), vec![4., 6.]);
	assert_eq!(s.slice_rows(1..3).slice_cols(1).slice_rows(1).as_iter().cloned().collect::<Vec<_>>(), vec![6.]);
}


#[test]
fn transpose() {
	let s = RowVec::regspace_rows(U1, U4, 0.);
	assert_eq!(s.t().iter().collect::<Vec<_>>(), [0., 1., 2., 3.]);
	assert_eq!(s.t().iter().len(), 4)
}*/
