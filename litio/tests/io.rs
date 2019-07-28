use litcontainers::*;
use litio::*;
use std::path::PathBuf;

#[test]
fn test_binary_read_write() {
	let s1 = ContainerRM::regspace_rows(U2, U2, 0.);

	let mut tmp: Vec<u8> = Vec::new();
	BinarySerializer::write(&mut tmp, &s1).unwrap();
	let s2: ContainerRM<f64, U2, U2> = BinaryDeserializer::read(&tmp[..]).unwrap();

	assert_eq!(s1.size(), s2.size());
	assert_eq!(s1.as_slice(), s2.as_slice());
}

#[test]
fn test_to_file() {
	let mut in_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	in_path.push( "tmp/U4_container.lit");

	let s1 = ContainerRM::regspace_rows(U2, U2, 0.);
	litio::write_binary_file(in_path.as_path(), &s1).unwrap();
	let s2: ContainerRM<f64, U2, U2> = litio::read_binary_file(in_path.as_path()).unwrap();

	assert_eq!(s1.size(), s2.size());
	assert_eq!(s1.as_slice(), s2.as_slice());
}