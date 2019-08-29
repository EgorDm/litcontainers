use crate::ops::*;
use crate::{InplaceMap, InplaceMapOrdered, StorageSize, Element, ContainerRM, StorageConstructor, IntoOrderedIterator};
use num_traits::{Pow};
use std::ops::{Neg};

operation_unary_op!(
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
	NegOp      => Neg: neg,
);

operation_scalar_binary_op!(
	PowOp | PowAssignOp => Pow: pow,
	LogOp | LogAssignOp => Log: log,
	MaxOp | MaxAssignOp => Max: max,
	MinOp | MinAssignOp => Min: min,
);

#[derive(new)]
pub struct ClampOp<L>
	where L: Operation
{
	data: L,
	min: L::Type,
	max: L::Type,
}

impl<L, S> Operation for ClampOp<L>
	where L: Operation<Result=S>,
	      S: InplaceMap<L::Type>,
	      L::Type: Clamp<L::Type, Output=L::Type>
{
	type Type = L::Type;
	type Rows = L::Rows;
	type Cols = L::Cols;
	type Result = L::Result;

	fn apply(self) -> Self::Result {
		let min = self.min.clone();
		let max = self.max.clone();
		let mut ret = self.data.apply();
		ret.mapv_inplace(|v| v.clamp(min, max));
		ret
	}
}

#[derive(new)]
pub struct NormOp<L>
	where L: Operation
{
	left: L
}

impl<L> Operation for NormOp<L>
	where L: Operation, L::Result: IntoOrderedIterator<L::Type> + StorageSize,
	      L::Type: Norm, <L::Type as Norm>::Output: Element
{
	type Type = L::Type;
	type Rows = <L::Result as StorageSize>::Rows;
	type Cols = <L::Result as StorageSize>::Cols;
	type Result = ContainerRM<<L::Type as Norm>::Output, Self::Rows, Self::Cols>;

	fn apply(self) -> Self::Result {
		let data = self.left.apply();
		let mut ret = Self::Result::zeros(data.size());
		ret.mapv_inplace_zip_ordered(data.into_ordered_iter(), |_, i| i.norm());
		ret
	}
}

#[derive(new)]
pub struct NormSqrOp<L>
	where L: Operation
{
	left: L
}

impl<L> Operation for NormSqrOp<L>
	where L: Operation, L::Result: IntoOrderedIterator<L::Type> + StorageSize,
	      L::Type: NormSqr, <L::Type as NormSqr>::Output: Element
{
	type Type = L::Type;
	type Rows = <L::Result as StorageSize>::Rows;
	type Cols = <L::Result as StorageSize>::Cols;
	type Result = ContainerRM<<L::Type as NormSqr>::Output, Self::Rows, Self::Cols>;

	fn apply(self) -> Self::Result {
		let data = self.left.apply();
		let mut ret = Self::Result::zeros(data.size());
		ret.mapv_inplace_zip_ordered(data.into_ordered_iter(), |_, i| i.norm_sqr());
		ret
	}
}

pub trait ScientificOps: IntoOperation + Sized
{
	operation_group_unary!(
		NegOp   : neg_op     => Neg,
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
//		NormOp  : norm_op     => Norm,
//		NormSqrOp  : norm_sqr_op     => NormSqr,
	);

	operation_group_scalar_binary!(
		PowOp   : pow_op => Pow,
		LogOp   : log_op => Log,
		MaxOp   : max_op => Max,
		MinOp   : min_op => Min,
	);

	fn clamp_op<O>(self, min: O, max: O) -> ClampOp<Self::OpType>
		where <Self::OpType as Operation>::Type: Clamp<<Self::OpType as Operation>::Type,
			  Output=<Self::OpType as Operation>::Type> + From<O>
	{
		ClampOp::new(self.into_operation(), min.into(), max.into())
	}

	fn norm_op(self) -> NormOp<Self::OpType>
		where <Self::OpType as Operation>::Type: Norm
	{
		NormOp::new(self.into_operation())
	}

	fn norm_sqr_op(self) -> NormSqrOp<Self::OpType>
		where <Self::OpType as Operation>::Type: NormSqr
	{
		NormSqrOp::new(self.into_operation())
	}
}

impl<O: IntoOperation> ScientificOps for O {}
