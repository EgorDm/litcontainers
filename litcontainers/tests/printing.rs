use litcontainers::*;

fn mock_container() -> ContainerRM<f64, U3, Dynamic> {
	ContainerRM::from_vec(U3, Dynamic::new(2), vec![1., 2., 3., 4., 5., 6.])
}

#[test]
fn test_printing() {
	let s = mock_container();
	println!("{}", format!("{}", Fmt(|f| print_storage(&s, f))));
}