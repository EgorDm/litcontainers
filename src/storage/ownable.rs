use crate::format::*;
use crate::storage::StorageMut;

/// Type can be turned or cloned into a container which owns its data.
pub trait Ownable<T: Scalar, R: Dim, C: Dim> {
	type OwnedType: StorageMut<T, R, C>;

	/// Converts itself to a container which owns its data. No guarantees that it wont be the same
	/// container if it is already owns its data.
	fn owned(self) -> Self::OwnedType;

	/// Clones it's data into a container which owns its data.
	fn clone_owned(&self) -> Self::OwnedType;
}