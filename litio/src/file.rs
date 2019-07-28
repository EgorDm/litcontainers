use crate::{GeneralSerializer, IOResult, GeneralDeserializer};
use std::fs::File;
use std::path::Path;

pub fn write<S: GeneralSerializer<T>, T>(path: &Path, data: &T) -> IOResult<()>{
	let mut f = File::create(path)?;
	S::write(&mut f, data)
}

pub fn read<S: GeneralDeserializer<T>, T>(path: &Path) -> IOResult<T>{
	let mut f = File::open(path)?;
	S::read(&mut f)
}