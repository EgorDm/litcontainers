use litcontainers::*;

#[test]
fn ops_scalar() {
	let s = ContainerRM::from_vec(Size::new(U3, D!(2)), &[1., 2., 3., 4., 5., 6.]);
	let res = s * 2.;
	assert_eq!(res.as_slice(), [2., 4., 6., 8., 10., 12.]);
}
/*
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
}*/
/*

#[test]
fn complex_norm() {
	let s = ContainerCM::from_vec(
		U2,
		Dynamic::new(2),
		&[c32::new(1., 2.), c32::new(2., 3.), c32::new(3., 4.), c32::new(4., 5.)]
	);

	assert_eq!(s.norm().as_slice(), [(5f32).sqrt(), (25f32).sqrt(), (13f32).sqrt(), (41f32).sqrt()]);
}

#[test]
fn ops_sci() {
	let s = ContainerRM::from_vec(U3, Dynamic::new(2), &[1., 2., 3., 4., 5., 6.]);

	assert_eq!((&s - 0.1).ceil().as_slice(), [1., 2., 3., 4., 5., 6.]);
	assert_eq!((&s - 0.1).floor().as_slice(), [0., 1., 2., 3., 4., 5.]);
	assert_eq!((&s).max(2.).as_slice(), [2., 2., 3., 4., 5., 6.]);
	assert_eq!((&s).pow(2).as_slice(), [1., 4., 9., 16., 25., 36.]);
}*/
