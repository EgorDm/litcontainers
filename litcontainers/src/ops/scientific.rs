use super::ops::*;
use super::ops_traits::*;
use crate::{InplaceMap};
use num_traits::{Float, Signed};

macro_rules! operation_scientific_scalar_unary_op (
	($($Name: ident => $Trait: ident: $op_fn: ident),* $(,)*) => {$(
		#[derive(new)]
		pub struct $Name<L>
			where L: Operation
		{
			left: L
		}

		impl<L> Operation for $Name<L>
			where L: Operation, L::Result: InplaceMap<L::Type>,
			      L::Type: $Trait<Output=L::Type>
		{
			type Type = L::Type;
			type Rows = L::Rows;
			type Cols = L::Cols;
			type Result = L::Result;

			fn apply(self) -> Self::Result {
				let mut ret = self.left.apply();
				ret.mapv_inplace(|v| v.$op_fn());
				ret
			}
		}
	)*}
);

operation_scientific_scalar_unary_op!(
	ASinOp     => ASin: asin,
	SinOp      => Sin: sin,
	ACosOp     => ACos: acos,
	CosOp      => Cos: cos,
	ATanOp     => ATan: atan,
	TanOp      => Tan: tan,
	ExpOp      => Exp: exp,
	Exp2Op     => Exp2: exp2,
	CeilOp     => Ceil: ceil,
	FloorOp    => Floor: floor,
	RoundOp    => Round: round,
	AbsOp      => Abs: abs,
	SqrtOp     => Sqrt: sqrt,
	Log2Op     => Log2: log2,
	Log10Op    => Log10: log10,
	LnOp       => Ln: ln,
);

macro_rules! operation_scientific_scalar_binary_op (
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

operation_scientific_scalar_binary_op!(
	LogOp => Log: log,
	MaxOp => Max: max,
	MinOp => Min: min,
);

macro_rules! scientific_ops (
	(
		$($Name: ident: $op_fn: ident => $Trait: ident),* $(,)*
		;
		$($NameBi: ident: $op_fn_bi: ident => $TraitBi: ident),* $(,)*
	) => {
		pub trait ScientificOps: IntoOperation + Sized
		{
		$(
			fn $op_fn(self) -> $Name<Self::OpType>
				where <Self::OpType as Operation>::Type: $Trait<Output=<Self::OpType as Operation>::Type>
			{
				$Name::new(self.into_operation())
			}
		)*
		$(
			fn $op_fn_bi<O>(self, rhs: O) -> $NameBi<Self::OpType>
				where <Self::OpType as Operation>::Type: $TraitBi<<Self::OpType as Operation>::Type, Output=<Self::OpType as Operation>::Type> + From<O>
			{
				$NameBi::new(self.into_operation(), rhs.into())
			}
		)*
		}
	}
);

scientific_ops!(
	ASinOp  : asin_op     => ASin,
	SinOp   : sin_op      => Sin,
	ACosOp  : acos_op     => ACos,
	CosOp   : cos_op      => Cos,
	ATanOp  : atan_op     => ATan,
	TanOp   : tan_op      => Tan,
	ExpOp   : exp_op      => Exp,
	Exp2Op  : exp2_op     => Exp2,
	CeilOp  : ceil_op     => Ceil,
	FloorOp : floor_op    => Floor,
	RoundOp : round_op    => Round,
	AbsOp   : abs_op      => Abs,
	SqrtOp  : sqrt_op     => Sqrt,
	Log2Op  : log2_op     => Log2,
	Log10Op : log10_op    => Log10,
	LnOp    : ln_op       => Ln,
	;
	LogOp   : log_op => Log,
	MaxOp   : max_op => Max,
	MinOp   : min_op => Min,
);

impl<O: IntoOperation> ScientificOps for O {}
