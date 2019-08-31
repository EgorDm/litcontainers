use litcontainers::*;

#[test]
fn ops_scalar() {
	let s = ContainerRM::from_vec(Size::new(U3, D!(2)), &[1., 2., 3., 4., 5., 6.]);

	{
		let mut s = s.clone_owned();
		s += 2;
		assert_eq!(s.as_slice(), [3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
	}

	assert_eq!((&s * 2).as_slice(), [2., 4., 6., 8., 10., 12.]);
	assert_eq!((&s + 1.).as_slice(), [2., 3., 4., 5., 6., 7.]);
	assert_eq!((-&s).as_slice(), [-1., -2., -3., -4., -5., -6.]);
}

#[test]
fn ops_storage() {
	let l = ContainerRM::from_vec(Size::new(U3, D!(2)), &[1., 2., 3., 4., 5., 6.]);
	let r = ContainerRM::from_vec(Size::new(U3, D!(2)), &[1., 2., 3., 4., 5., 6.]);

	assert_eq!((&l + r.into_slice()).as_slice(), [2., 4., 6., 8., 10., 12.]);
	assert_eq!((&l * r.into_slice()).as_slice(), [1., 4., 9., 16., 25., 36.]);

	{
		let mut s = l.clone_owned();
		s += &r;
		assert_eq!(s.as_slice(), [2., 4., 6., 8., 10., 12.]);
	}

	assert_eq!((l.slice_rows(0..3) + r.into_slice()).as_slice(), [2., 4., 6., 8., 10., 12.]);
	assert_eq!((l.slice_rows(0..3) + r.into_slice()).as_slice(), [2., 4., 6., 8., 10., 12.]);
}

#[test]
fn ops_sci() {
	let s = ContainerRM::from_vec(Size::new(U3, D!(2)), &[1., 2., 3., 4., 5., 6.]);

	assert_eq!((&s - 0.1).ceil().as_slice(), [1., 2., 3., 4., 5., 6.]);
	assert_eq!((&s).clamp(3, 4).as_slice(), [3., 3., 3., 4., 4., 4.]);
	assert_eq!((&s - 0.1).floor().as_slice(), [0., 1., 2., 3., 4., 5.]);
	assert_eq!((&s).max(2.).as_slice(), [2., 2., 3., 4., 5., 6.]);
	assert_eq!((&s).pow(2).as_slice(), [1., 4., 9., 16., 25., 36.]);
}

#[test]
fn complex_norm() {
	let s = ContainerRM::from_vec(Size::new(U2, D!(2)),
	                              &[c32::new(1., 2.), c32::new(2., 3.), c32::new(3., 4.), c32::new(4., 5.)]
	);

	assert_eq!(s.norm().as_slice(), [(5f32).sqrt(), (13f32).sqrt(), (25f32).sqrt(), (41f32).sqrt()]);
	assert_eq!(s.norm_sqr().as_slice(), [5f32, 13f32, 25f32, 41f32]);
}
