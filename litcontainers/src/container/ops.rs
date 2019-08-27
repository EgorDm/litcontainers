use crate::format::*;
use crate::storage::*;
use crate::container::*;
use crate::slice::*;
use crate::ops::*;
use std::ops::{Add, Sub, Mul, Div, Rem};

macro_rules! impl_arith_scalar_traits (
	(
		$($Name: ident: $op_fn: ident => $Trait: ident: $trait_fn: ident),* $(,)*
	) => {
		$(
			impl<T, S, R> $Trait<R> for Container<T, S>
				where T: Element, R: Element, S: Storage<T>, T: $Trait<T, Output=T> + From<R>
			{
				type Output = <Self as Ownable<T>>::OwnedType;

				fn $trait_fn(self, rhs: R) -> Self::Output { self.$op_fn(rhs).apply() }
			}

			impl<'a, T, S, R> $Trait<R> for &'a Container<T, S>
				where T: Element, R: Element, S: Storage<T>, T: $Trait<T, Output=T> + From<R>
			{
				type Output = VecStorageRM<T, S::Rows, S::Cols>;

				fn $trait_fn(self, rhs: R) -> Self::Output { self.into_slice().$op_fn(rhs).apply() }
			}
		)*
	}
);

impl_arith_scalar_traits!(
	AddScalar: add_scalar => Add: add,
    SubScalar: sub_scalar => Sub: sub,
	MulScalar: mul_scalar => Mul: mul,
	DivScalar: div_scalar => Div: div,
	RemScalar: rem_scalar => Rem: rem,
);

macro_rules! impl_arith_storage_traits (
	(
		$($Name: ident: $op_fn: ident => $Trait: ident: $trait_fn: ident),* $(,)*
	) => {
		$(
			impl<T, S, R> $Trait<Container<T, R>> for Container<T, S>
				where T: Element,
				      S: Storage<T>, R: Storage<T>,
				      T: $Trait<T, Output=T>
			{
				type Output = <Self as Ownable<T>>::OwnedType;

				fn $trait_fn(self, rhs: Container<T, R>) -> Self::Output { self.$op_fn(rhs).apply() }
			}

			impl<'a, T, S, R> $Trait<Container<T, R>> for &'a Container<T, S>
				where T: Element,
				      S: Storage<T>, R: Storage<T>,
				      T: $Trait<T, Output=T>
			{
				type Output = VecStorageRM<T, S::Rows, S::Cols>;

				fn $trait_fn(self, rhs: Container<T, R>) -> Self::Output { self.into_slice().$op_fn(rhs).apply() }
			}
		)*
	}
);

impl_arith_storage_traits!(
	AddStorage: add_storage => Add: add,
    SubStorage: sub_storage => Sub: sub,
	MulStorage: mul_storage => Mul: mul,
	DivStorage: div_storage => Div: div,
	RemStorage: rem_storage => Rem: rem,
);

macro_rules! impl_scientific_traits (
	(
		$($Name: ident: $op_fn: ident => $Trait: ident: $trait_fn: ident),* $(,)*
		;
		$($NameBi: ident: $op_fn_bi: ident => $TraitBi: ident: $trait_fn_bi: ident),* $(,)*

	) => {
		$(
			impl<T, S> $Trait for Container<T, S>
				where T: Element, S: Storage<T>, T: $Trait<Output=T>
			{
				type Output = <Self as Ownable<T>>::OwnedType;

				fn $trait_fn(self) -> Self::Output { self.$op_fn().apply() }
			}

			impl<'a, T, S> $Trait for &'a Container<T, S>
				where T: Element, S: Storage<T>, T: $Trait<Output=T>
			{
				type Output = VecStorageRM<T, S::Rows, S::Cols>;

				fn $trait_fn(self) -> Self::Output { self.into_slice().$op_fn().apply() }
			}
		)*
		$(
			impl<T, S, R> $TraitBi<R> for Container<T, S>
				where T: Element, R: Element, S: Storage<T>, T: $TraitBi<T, Output=T> + From<R>
			{
				type Output = <Self as Ownable<T>>::OwnedType;

				fn $trait_fn_bi(self, rhs: R) -> Self::Output { self.$op_fn_bi(rhs).apply() }
			}

			impl<'a, T, S, R> $TraitBi<R> for &'a Container<T, S>
				where T: Element, R: Element, S: Storage<T>, T: $TraitBi<T, Output=T> + From<R>
			{
				type Output = VecStorageRM<T, S::Rows, S::Cols>;

				fn $trait_fn_bi(self, rhs: R) -> Self::Output { self.into_slice().$op_fn_bi(rhs).apply() }
			}
		)*
	}
);

impl_scientific_traits!(
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
	;
	LogOp   : log_op => Log: log,
	MaxOp   : max_op => Max: max,
	MinOp   : min_op => Min: min,
);