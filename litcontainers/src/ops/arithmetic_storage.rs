use super::ops::*;
use crate::iterator::*;
use crate::storage::InplaceMapOrdered;
use std::ops::{Add, Sub, Mul, Div, Rem};

macro_rules! operation_simple_storage_op (
	($($Name: ident => $Trait: ident: $op_fn: ident),* $(,)*) => {$(
		#[derive(new)]
		pub struct $Name<L, R>
			where L: Operation, R: Operation
		{
			left: L,
			right: R,
		}

		impl<L, R> Operation for $Name<L, R>
			where L: Operation, L::Result: InplaceMapOrdered<L::Type>,
			      R: Operation, R::Result: IntoOrderedIterator<R::Type>,
			      L::Type: $Trait<R::Type, Output=L::Type>
		{
			type Type = L::Type;
			type Rows = L::Rows;
			type Cols = L::Cols;
			type Result = L::Result;

			fn apply(self) -> Self::Result {
				let mut l = self.left.apply();
				let r = self.right.apply();
				l.mapv_inplace_zip_ordered(r.into_ordered_iter(), |l, r| l.$op_fn(r));
				l
			}
		}
	)*}
);

operation_simple_storage_op!(
    AddStorage => Add: add,
	SubStorage => Sub: sub,
	MulStorage => Mul: mul,
	DivStorage => Div: div,
	RemStorage => Rem: rem,
);

macro_rules! operation_simple_storage_rev_op (
	($($Name: ident => $Trait: ident: $op_fn: ident),* $(,)*) => {$(
		#[derive(new)]
		pub struct $Name<L, R>
			where L: Operation, R: Operation
		{
			left: L,
			right: R,
		}

		impl<L, R, LS, RS> Operation for $Name<L, R>
			where L: Operation<Result=LS>, LS: IntoOrderedIterator<L::Type>,
			      R: Operation<Result=RS>, RS: InplaceMapOrdered<R::Type>,
			      L::Type: $Trait<R::Type, Output=R::Type>
		{
			type Type = R::Type;
			type Rows = R::Rows;
			type Cols = R::Cols;
			type Result = R::Result;

			fn apply(self) -> Self::Result {
				let l = self.left.apply();
				let mut r = self.right.apply();
				r.mapv_inplace_zip_ordered(l.into_ordered_iter(), |r, l| l.$op_fn(r));
				r
			}
		}
	)*}
);

operation_simple_storage_rev_op!(
	SubStorageRev => Sub: sub,
	DivStorageRev => Div: div,
	RemStorageRev => Rem: rem,
);

/*
pub trait ArithmeticStorageOps: IntoOperation + Sized {
	fn add_storage<R>(self, rhs: R) -> AddStorage<Self::OpType, R::OpType>
		where <Self::OpType as Operation>::Type: Add<<R::OpType as Operation>::Type, Output=<Self::OpType as Operation>::Type>,
		      R: IntoOperation
	{
		AddStorage::new(self.into_operation(), rhs.into_operation())
	}
}
*/



macro_rules! arithmetic_ops (
	(
		$($Name: ident: $op_fn: ident => $Trait: ident),* $(,)*
		;
		$($NameRev: ident: $op_fn_rev: ident => $TraitRev: ident),* $(,)*
	) => {
		pub trait ArithmeticStorageOps<O>: IntoOperation + Sized
			where O: IntoOperation, <O::OpType as Operation>::Result: IntoOrderedIterator<<O::OpType as Operation>::Type>,
			      <Self::OpType as Operation>::Result: InplaceMapOrdered<<Self::OpType as Operation>::Type>
		{
		$(
			fn $op_fn(self, rhs: O) -> $Name<Self::OpType, O::OpType>
				where <Self::OpType as Operation>::Type: $Trait<<O::OpType as Operation>::Type, Output=<Self::OpType as Operation>::Type>,
			{
				$Name::new(self.into_operation(), rhs.into_operation())
			}
		)*
		$(
			fn $op_fn_rev<L>(self, lhs: O) -> $NameRev<O::OpType, Self::OpType>
				where <O::OpType as Operation>::Type: $TraitRev<<Self::OpType as Operation>::Type, Output=<Self::OpType as Operation>::Type>
			{
				$NameRev::new(lhs.into_operation(), self.into_operation())
			}
		)*
		}
	}
);

arithmetic_ops!(
	AddStorage: add_storage => Add,
    SubStorage: sub_storage => Sub,
	MulStorage: mul_storage => Mul,
	DivStorage: div_storage => Div,
	RemStorage: rem_storage => Rem,
	;
	SubStorageRev: sub_storage_rev => Sub,
	DivStorageRev: div_storage_rev => Div,
	RemStorageRev: rem_storage_rev => Rem,
);

impl<O, OT> ArithmeticStorageOps<OT> for O
	where O: IntoOperation, <O::OpType as Operation>::Result: InplaceMapOrdered<<O::OpType as Operation>::Type>,
	      OT: IntoOperation, <OT::OpType as Operation>::Result: IntoOrderedIterator<<OT::OpType as Operation>::Type>,
{}