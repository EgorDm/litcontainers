use crate::format::*;
use crate::storage::*;
use crate::container::*;
use crate::slice::*;
use crate::ops::*;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign};
use crate::IntoOrderedIterator;

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