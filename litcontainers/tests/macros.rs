use litcontainers::*;

#[test]
fn construct() {
	assert_eq!(rvec_zeros!(U3; f32).as_slice(), &[0., 0., 0.]);
	assert_eq!(rvec!(&[1., 2., 3.]).as_slice(), &[1., 2., 3.]);
	assert_eq!(rvec!(U3, &[1., 2., 3.]).as_slice(), &[1., 2., 3.]);
	assert_eq!(rvec!(U3, &[1., 2., 3.]; f64).as_slice(), &[1., 2., 3.]);

	assert_eq!(cvec_zeros!(U3; f32).as_slice(), &[0., 0., 0.]);
	assert_eq!(cvec!(&[1., 2., 3.]).as_slice(), &[1., 2., 3.]);
	assert_eq!(cvec!(U3, &[1., 2., 3.]).as_slice(), &[1., 2., 3.]);
	assert_eq!(cvec!(U3, &[1., 2., 3.]; f64).as_slice(), &[1., 2., 3.]);
}

fn mock_container() -> ContainerRM<f64, U3, Dynamic> {
	ContainerRM::from_vec(U3, Dynamic::new(2), &[1., 2., 3., 4., 5., 6.])
}

#[test]
fn join() {
	let a = ContainerRM::from_vec(U2, Dynamic::new(2), &[1., 2., 3., 4.]);
	let b = ContainerRM::from_vec(U2, Dynamic::new(2), &[1., 2., 3., 4.]);
	let c = ContainerRM::from_vec(U2, Dynamic::new(2), &[1., 2., 3., 4.]);
	assert_eq!(join_cols!(a, b, c; U6; ContainerRM).as_slice(), [1., 2., 1., 2., 1., 2., 3., 4., 3., 4., 3., 4.]);
	assert_eq!(join_cols!(a, b, c; U6).as_slice(), [1., 2., 1., 2., 1., 2., 3., 4., 3., 4., 3., 4.]);
	assert_eq!(join_cols!(a, b, c).as_slice(), [1., 2., 1., 2., 1., 2., 3., 4., 3., 4., 3., 4.]);

	assert_eq!(join_rows!(a, b, c; U6; ContainerRM).as_slice(), [1., 2., 3., 4., 1., 2., 3., 4., 1., 2., 3., 4.]);
	assert_eq!(join_rows!(a, b, c; U6).as_slice(), [1., 2., 3., 4., 1., 2., 3., 4., 1., 2., 3., 4.]);
	assert_eq!(join_rows!(a, b, c).as_slice(), [1., 2., 3., 4., 1., 2., 3., 4., 1., 2., 3., 4.]);
}