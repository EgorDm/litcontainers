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