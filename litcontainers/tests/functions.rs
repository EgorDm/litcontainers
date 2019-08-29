use litcontainers::*;

#[test]
fn test_cumsum() {
	let data = ContainerRM::from_value(Size::new(U3, U3), 1.);

	assert_eq!(cumsum(&data, RowAxis).as_slice(), [
		1., 2., 3.,
		1., 2., 3.,
		1., 2., 3.
	]);

	assert_eq!(cumsum(&data, ColAxis).as_slice(), [ // Is colmajor
		1.0, 1.0, 1.0,
		2.0, 2.0, 2.0,
		3.0, 3.0, 3.0
	]);
}

#[test]
fn test_sum() {
	let data = ContainerRM::regspace_rows(Size::new(U3, U3), 1.);

	assert_eq!(sum_rows(&data).as_slice(), [6., 6., 6.]);

	assert_eq!(sum_cols(&data).as_slice(),  [3., 6., 9.]);
}

#[test]
fn test_mean() {
	let data = ContainerRM::regspace_rows(Size::new(U3, U3), 1.);

	assert_eq!(mean_rows(&data).as_slice(), [2., 2., 2.]);

	assert_eq!(mean_cols(&data).as_slice(),  [1., 2., 3.]);
}