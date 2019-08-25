use crate::format::*;
use crate::storage::*;
use crate::container::*;
use crate::ops::*;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign};

macro_rules! impl_arith_traits (
	(
		$($Name: ident: $op_fn: ident => $Trait: ident: $trait_fn: ident),* $(,)*
	) => {
		$(
			impl<T, S, R> $Trait<R> for Container<T, S>
				where T: Element, S: Storage<T>, T: $Trait<T, Output=T> + From<R>
			{
				type Output = <Self as Ownable<T>>::OwnedType;

				fn $trait_fn(self, rhs: R) -> Self::Output { self.$op_fn(rhs).apply() }
			}
		)*
	}
);

impl_arith_traits!(
	AddScalar: add_scalar => Add: add,
    SubScalar: sub_scalar => Sub: sub,
	MulScalar: mul_scalar => Mul: mul,
	DivScalar: div_scalar => Div: div,
	RemScalar: rem_scalar => Rem: rem,
);