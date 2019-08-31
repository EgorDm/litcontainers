use crate::format::*;
use crate::storage::*;
use crate::container::*;
use crate::slice::*;
use crate::ops::*;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Neg};
use num_traits::{Pow, Float};
use num_complex::Complex;

macro_rules! impl_scalar_binary_traits (
	(
		$($Name: ident: $op_fn: ident => $Trait: ident: $trait_fn: ident
			$(| $NameAssign: ident: $op_assign_fn: ident => $TraitAssign: ident: $trait_assign_fn: ident)?),* $(,)*
	) => {
		$(
			impl<T, S, R> $Trait<R> for Container<T, S>
				where T: Element, R: Element, S: Storage<T>, T: $Trait<T, Output=T> + From<R>
			{
				type Output = Container<T, <Self as Ownable<T>>::OwnedType>;

				fn $trait_fn(self, rhs: R) -> Self::Output { self.$op_fn(rhs).apply() }
			}

			impl<'a, T, S, R> $Trait<R> for &'a Container<T, S>
				where T: Element, R: Element, S: Storage<T>, T: $Trait<T, Output=T> + From<R>
			{
				type Output = ContainerRM<T, S::Rows, S::Cols>;

				fn $trait_fn(self, rhs: R) -> Self::Output { self.into_slice().$op_fn(rhs).apply().into() }
			}

			$(
				impl<T, S, R> $TraitAssign<R> for Container<T, S>
					where T: Element, R: Element, S: StorageMut<T>, T: $Trait<T, Output=T> + From<R>
				{
					fn $trait_assign_fn(&mut self, rhs: R) { self.$op_assign_fn(rhs).apply() }
				}
			)?
		)*
	}
);

impl_scalar_binary_traits!(
	AddScalar: add_scalar => Add: add | AddAssignScalar: add_assign_scalar => AddAssign: add_assign,
    SubScalar: sub_scalar => Sub: sub | SubAssignScalar: sub_assign_scalar => SubAssign: sub_assign,
	MulScalar: mul_scalar => Mul: mul | MulAssignScalar: mul_assign_scalar => MulAssign: mul_assign,
	DivScalar: div_scalar => Div: div | DivAssignScalar: div_assign_scalar => DivAssign: div_assign,
	RemScalar: rem_scalar => Rem: rem | RemAssignScalar: rem_assign_scalar => RemAssign: rem_assign,
	PowOp    : pow_op     => Pow: pow,
	LogOp    : log_op     => Log: log,
	MaxOp    : max_op     => Max: max,
	MinOp    : min_op     => Min: min,
);

macro_rules! impl_storage_binary_traits (
	(
		$($Name: ident: $op_fn: ident => $Trait: ident: $trait_fn: ident
			$(| $NameAssign: ident: $op_assign_fn: ident => $TraitAssign: ident: $trait_assign_fn: ident)?),* $(,)*
	) => {
		$(
			impl<T, S, R> $Trait<Container<T, R>> for Container<T, S>
				where T: Element,
				      S: Storage<T>, R: Storage<T>,
				      T: $Trait<T, Output=T>
			{
				type Output = Container<T, <Self as Ownable<T>>::OwnedType>;

				fn $trait_fn(self, rhs: Container<T, R>) -> Self::Output { self.$op_fn(rhs).apply() }
			}

			impl<'a, T, S, R> $Trait<Container<T, R>> for &'a Container<T, S>
				where T: Element,
				      S: Storage<T>, R: Storage<T>,
				      T: $Trait<T, Output=T>
			{
				type Output = ContainerRM<T, S::Rows, S::Cols>;

				fn $trait_fn(self, rhs: Container<T, R>) -> Self::Output { self.into_slice().$op_fn(rhs).apply().into() }
			}

			$(
				impl<'a, T, S, R> $TraitAssign<&'a Container<T, R>> for Container<T, S>
					where T: Element, R: Storage<T>, S: StorageMut<T>, T: $Trait<T, Output=T>
				{
					fn $trait_assign_fn(&mut self, rhs: &'a Container<T, R>) { self.$op_assign_fn(rhs).apply() }
				}
			)?
		)*
	}
);

impl_storage_binary_traits!(
	AddStorage: add_storage => Add: add | AddAssignStorage: add_assign_storage => AddAssign: add_assign,
    SubStorage: sub_storage => Sub: sub | SubAssignStorage: sub_assign_storage => SubAssign: sub_assign,
	MulStorage: mul_storage => Mul: mul | MulAssignStorage: mul_assign_storage => MulAssign: mul_assign,
	DivStorage: div_storage => Div: div | DivAssignStorage: div_assign_storage => DivAssign: div_assign,
	RemStorage: rem_storage => Rem: rem | RemAssignStorage: rem_assign_storage => RemAssign: rem_assign,
);

macro_rules! impl_unary_traits (
	(
		$($Name: ident: $op_fn: ident => $Trait: ident: $trait_fn: ident),* $(,)*
	) => {
		$(
			impl<T, S> $Trait for Container<T, S>
				where T: Element, S: Storage<T>, T: $Trait<Output=T>
			{
				type Output = Container<T, <Self as Ownable<T>>::OwnedType>;

				fn $trait_fn(self) -> Self::Output { self.$op_fn().apply() }
			}

			impl<'a, T, S> $Trait for &'a Container<T, S>
				where T: Element, S: Storage<T>, T: $Trait<Output=T>
			{
				type Output = ContainerRM<T, S::Rows, S::Cols>;

				fn $trait_fn(self) -> Self::Output { self.into_slice().$op_fn().apply().into() }
			}
		)*
	}
);

impl_unary_traits!(
	ASinOp  : asin_op     => ASin:      asin,
	SinOp   : sin_op      => Sin:       sin,
	ACosOp  : acos_op     => ACos:      acos,
	CosOp   : cos_op      => Cos:       cos,
	ATanOp  : atan_op     => ATan:      atan,
	TanOp   : tan_op      => Tan:       tan,
	ExpOp   : exp_op      => Exp:       exp,
	Exp2Op  : exp2_op     => Exp2:      exp2,
	CeilOp  : ceil_op     => Ceil:      ceil,
	FloorOp : floor_op    => Floor:     floor,
	RoundOp : round_op    => Round:     round,
	AbsOp   : abs_op      => Abs:       abs,
	SqrtOp  : sqrt_op     => Sqrt:      sqrt,
	Log2Op  : log2_op     => Log2:      log2,
	Log10Op : log10_op    => Log10:     log10,
	LnOp    : ln_op       => Ln:        ln,
	NegOp   : neg_op      => Neg:       neg
);

impl<T, S, R> Clamp<R> for Container<T, S>
	where T: Element, R: Element, S: Storage<T>, T: Clamp<T, Output=T> + From<R>
{
	type Output = Container<T, <Self as Ownable<T>>::OwnedType>;

	fn clamp(self, min: R, max: R) -> Self::Output { self.clamp_op(min, max).apply() }
}

impl<'a, T, S, R> Clamp<R> for &'a Container<T, S>
	where T: Element, R: Element, S: Storage<T>, T: Clamp<T, Output=T> + From<R>
{
	type Output = ContainerRM<T, S::Rows, S::Cols>;

	fn clamp(self, min: R, max: R) -> Self::Output { self.into_slice().clamp_op(min, max).apply() }
}

impl<'a, T, S> Norm for &'a Container<Complex<T>, S>
	where T: Float + Scalar, S: Storage<Complex<T>>
{
	type Output = ContainerRM<T, S::Rows, S::Cols>;

	fn norm(self) -> Self::Output { self.norm_op().apply() }
}

impl<'a, T, S> NormSqr for &'a Container<Complex<T>, S>
	where T: Float + Scalar, S: Storage<Complex<T>>
{
	type Output = ContainerRM<T, S::Rows, S::Cols>;

	fn norm_sqr(self) -> Self::Output { self.norm_sqr_op().apply() }
}