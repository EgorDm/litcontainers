use crate::ops::*;
use crate::iterator::*;
use crate::format::{StorageSize};
use crate::storage::{InplaceMapOrdered};
use std::ops::{Add, Sub, Mul, Div, Rem};
use crate::{Element};

operation_storage_binary_op!(
    AddStorage | AddAssignStorage => Add: add,
	SubStorage | SubAssignStorage => Sub: sub,
	MulStorage | MulAssignStorage => Mul: mul,
	DivStorage | DivAssignStorage => Div: div,
	RemStorage | RemAssignStorage => Rem: rem,
);

operation_storage_binary_rev_op!(
	SubStorageRev | SubAssignStorageRev => Sub: sub,
	DivStorageRev | DivAssignStorageRev => Div: div,
	RemStorageRev | RemAssignStorageRev => Rem: rem,
);


pub trait ArithmeticStorageOps: IntoOperation + Sized
	where <Self::OpType as Operation>::Result: InplaceMapOrdered<<Self::OpType as Operation>::Type>
{
	operation_group_storage_binary!(
		AddStorage: add_storage | AddAssignStorage: add_assign_storage => Add,
		SubStorage: sub_storage | SubAssignStorage: sub_assign_storage => Sub,
		MulStorage: mul_storage | MulAssignStorage: mul_assign_storage => Mul,
		DivStorage: div_storage | DivAssignStorage: div_assign_storage => Div,
		RemStorage: rem_storage | RemAssignStorage: rem_assign_storage => Rem,
	);

	operation_group_storage_binary_rev!(
		SubStorageRev: sub_storage_rev => Sub,
		DivStorageRev: div_storage_rev => Div,
		RemStorageRev: rem_storage_rev => Rem,
	);
}


impl<O> ArithmeticStorageOps for O
	where O: IntoOperation, <O::OpType as Operation>::Result: InplaceMapOrdered<<O::OpType as Operation>::Type>
{}