use litcontainers::*;

fn mock_container() -> ContainerRM<f64, U3, Dynamic> {
	ContainerRM::from_vec(U3, Dynamic::new(2), vec![1., 2., 3., 4., 5., 6.])
}

#[test]
fn sized() {
	let s = mock_container();
	let s2 = mock_container();

	assert_eq!(s.row_count(), 3);
	assert_eq!(s.col_count(), 2);
	assert_eq!(s.row_stride(), s.col_count());
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
	assert_eq!(s.as_row_slice(1), [3., 4.]);
	assert_eq!(s.as_col_slice(1), [2., 3., 4., 5., 6.]);
}

#[test]
fn resizing() {
	let mut s = ContainerRM::from_vec(Dynamic::new(2), Dynamic::new(2), vec![1., 2., 3., 4.]);
	s.set_col_count(3);
	assert_eq!(s.col_count(), 3);
	assert_eq!(s.as_slice(), [1., 2., 0., 3., 4., 0.]);

	let mut s = ContainerRM::from_vec(Dynamic::new(2), Dynamic::new(2), vec![1., 2., 3., 4.]);
	s.set_row_count(3);
	assert_eq!(s.row_count(), 3);
	assert_eq!(s.as_slice(), [1., 2., 3., 4., 0., 0.]);

	let mut s = ContainerCM::from_vec(Dynamic::new(2), Dynamic::new(2), vec![1., 2., 3., 4.]);
	s.set_col_count(3);
	assert_eq!(s.col_count(), 3);
	assert_eq!(s.as_slice(), [1., 3., 2., 4., 0., 0.]);

	let mut s = ContainerCM::from_vec(Dynamic::new(2), Dynamic::new(2), vec![1., 2., 3., 4.]);
	s.set_row_count(3);
	assert_eq!(s.row_count(), 3);
	assert_eq!(s.as_slice(), [1., 3., 0., 2., 4., 0.]);
}

#[test]
fn mutable() {
	let mut s = mock_container();

	assert_eq!(s.as_row_mut_slice(1), [3., 4.]);
	assert_eq!(s.as_col_mut_slice(1), [2., 3., 4., 5., 6.]);

	*s.get_mut(1, 1) = 1337.;
	assert_eq!(s.get(1, 1), 1337.);
}