use litcontainers::*;

#[test]
fn complex_norm() {
	let s = ContainerCM::from_vec(
		U2,
		Dynamic::new(2),
		&[c32::new(1., 2.), c32::new(2., 3.), c32::new(3., 4.), c32::new(4., 5.)]
	);

	assert_eq!(s.norm().as_slice(), [(5f32).sqrt(), (25f32).sqrt(), (13f32).sqrt(), (41f32).sqrt()]);
}