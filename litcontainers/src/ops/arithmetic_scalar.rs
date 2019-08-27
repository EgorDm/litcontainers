use super::ops::*;
use std::ops::{Add, Sub, Mul, Div, Rem};
use crate::{InplaceMap};

macro_rules! operation_simple_scalar_op (
	($($Name: ident => $Trait: ident: $op_fn: ident),* $(,)*) => {$(
		#[derive(new)]
		pub struct $Name<L>
			where L: Operation
		{
			left: L,
			right: L::Type,
		}

		impl<L, S> Operation for $Name<L>
			where L: Operation<Result=S>,
			      S: InplaceMap<L::Type>,
			      L::Type: $Trait<L::Type, Output=L::Type>
		{
			type Type = L::Type;
			type Rows = L::Rows;
			type Cols = L::Cols;
			type Result = L::Result;

			fn apply(self) -> Self::Result {
				let r = self.right;
				let mut ret = self.left.apply();
				ret.mapv_inplace(|v| v.$op_fn(r.clone()));
				ret
			}
		}
	)*}
);

operation_simple_scalar_op!(
    AddScalar => Add: add,
	SubScalar => Sub: sub,
	MulScalar => Mul: mul,
	DivScalar => Div: div,
	RemScalar => Rem: rem,
);

macro_rules! operation_simple_scalar_rev_op (
	($($Name: ident => $Trait: ident: $op_fn: ident),* $(,)*) => {$(
		#[derive(new)]
		pub struct $Name<R>
			where R: Operation
		{
			left: R::Type,
			right: R,
		}

		impl<R, S> Operation for $Name<R>
			where R: Operation<Result=S>,
			      S: InplaceMap<R::Type>,
			      R::Type: $Trait<R::Type, Output=R::Type>
		{
			type Type = R::Type;
			type Rows = R::Rows;
			type Cols = R::Cols;
			type Result = R::Result;

			fn apply(self) -> Self::Result {
				let l = self.left;
				let mut ret = self.right.apply();
				ret.mapv_inplace(|v| l.clone().$op_fn(v));
				ret
			}
		}
	)*}
);

operation_simple_scalar_rev_op!(
	SubScalarRev => Sub: sub,
	DivScalarRev => Div: div,
	RemScalarRev => Rem: rem,
);

macro_rules! arithmetic_ops (
	(
		$($Name: ident: $op_fn: ident => $Trait: ident),* $(,)*
		;
		$($NameRev: ident: $op_fn_rev: ident => $TraitRev: ident),* $(,)*
	) => {
		pub trait ArithmeticScalarOps<O>: IntoOperation + Sized
			where O: Into< <Self::OpType as Operation>::Type>
		{
		$(
			fn $op_fn(self, rhs: O) -> $Name<Self::OpType>
				where <Self::OpType as Operation>::Type: $Trait<<Self::OpType as Operation>::Type, Output=<Self::OpType as Operation>::Type>
			{
				$Name::new(self.into_operation(), rhs.into())
			}
		)*
		$(
			fn $op_fn_rev(self, lhs: O) -> $NameRev<Self::OpType>
				where <Self::OpType as Operation>::Type: $TraitRev<<Self::OpType as Operation>::Type, Output=<Self::OpType as Operation>::Type>
			{
				$NameRev::new(lhs.into(), self.into_operation())
			}
		)*
		}
	}
);

arithmetic_ops!(
	AddScalar: add_scalar => Add,
    SubScalar: sub_scalar => Sub,
	MulScalar: mul_scalar => Mul,
	DivScalar: div_scalar => Div,
	RemScalar: rem_scalar => Rem,
	;
	SubScalarRev: sub_scalar_rev => Sub,
	DivScalarRev: div_scalar_rev => Div,
	RemScalarRev: rem_scalar_rev => Rem,
);

impl<O: IntoOperation, OT: Into<<O::OpType as Operation>::Type>> ArithmeticScalarOps<OT> for O {}

