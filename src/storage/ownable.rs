use crate::format::*;
use crate::storage::StorageMut;

pub trait Ownable<T: Scalar, R: Dim, C: Dim> {
	type OwnedType: StorageMut<T, R, C>;

	fn owned(self) -> Self::OwnedType;

	fn clone_owned(&self) -> Self::OwnedType;
}