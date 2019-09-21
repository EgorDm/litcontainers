use litcontainers::*;

#[test]
fn split_at() {
	let s = ContainerRM::regspace(Size::new(U4, U4), RowAxis, 0);
	let slice = s.into_slice();

	let (l, r) = slice.split_at_row(U2);

	assert_eq!(l.as_slice(), [
		0, 1, 2, 3,
		0, 1, 2, 3,
	]);
}

#[test]
fn split_into() {
	{
		let s = ContainerRM::regspace(Size::new(U4, U4), RowAxis, 0);
		let slice = s.into_slice();

		let chunks = split_into!(slice, 4; RowAxis);
		assert_eq!(chunks.len(), 4);

		for chunk in chunks {
			assert_eq!(chunk.as_slice(), [0, 1, 2, 3]);
		}
	}

	{
		let s = ContainerCM::regspace(Size::new(U4, U4), RowAxis, 0);
		let slice = s.into_slice();

		let chunks = split_into!(slice, 4; ColAxis);
		assert_eq!(chunks.len(), 4);

		for (i, chunk) in chunks.iter().enumerate() {
			let i = i as i32;
			let test =
				assert_eq!(chunk.as_slice(), [i, i, i, i]);
		}
	}
}