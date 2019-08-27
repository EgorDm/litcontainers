use crate::{Scalar};
use num_traits::{Float, Signed};

macro_rules! unary_op_trait (
	($Trait: ident, $method: ident, $TraitAssign: ident, $method_assign: ident) => {
		pub trait $Trait {
			type Output;

			fn $method(self) -> Self::Output;
		}

		pub trait $TraitAssign {
			fn $method_assign(&mut self);
		}
	}
);

unary_op_trait!(ASin, asin, ASinAssign, asin_assign);
unary_op_trait!(Sin, sin, SinAssign, sin_assign);
unary_op_trait!(ACos, acos, ACosAssign, acos_assign);
unary_op_trait!(Cos, cos, CosAssign, cos_assign);
unary_op_trait!(Tan, tan, TanAssign, tan_assign);
unary_op_trait!(ATan, atan, ATanAssign, atan_assign);
unary_op_trait!(Exp, exp, ExpAssign, exp_assign);
unary_op_trait!(Exp2, exp2, Exp2Assign, exp2_assign);
unary_op_trait!(Ceil, ceil, CeilAssign, ceil_assign);
unary_op_trait!(Floor, floor, FloorAssign, floor_assign);
unary_op_trait!(Round, round, RoundAssign, round_assign);
unary_op_trait!(Abs, abs, AbsAssign, abs_assign);
unary_op_trait!(Sqrt, sqrt, SqrtAssign, sqrt_assign);
unary_op_trait!(Log2, log2, Log2Assign, log2_assign);
unary_op_trait!(Log10, log10, Log10Assign, log10_assign);
unary_op_trait!(Ln, ln, LnAssign, ln_assign);
unary_op_trait!(Norm, norm, NormAssign, norm_assign);

macro_rules! impl_op_traits (
	($($GroupTrait: ident => $Trait: ident: $op_fn: ident),* $(,)*) => {$(
		impl<T: $GroupTrait> $Trait for T {
			type Output = T;

			fn $op_fn(self) -> Self::Output { $GroupTrait::$op_fn(self) }
		}
	)*}
);

impl_op_traits!(
	Float   => ASin: asin,
	Float   => Sin: sin,
	Float   => ACos: acos,
	Float   => Cos: cos,
	Float   => ATan: atan,
	Float   => Tan: tan,
	Float   => Exp: exp,
	Float   => Exp2: exp2,
	Float   => Ceil: ceil,
	Float   => Floor: floor,
	Float   => Round: round,
	Float   => Sqrt: sqrt,
	Float   => Log2: log2,
	Float   => Log10: log10,
	Float   => Ln: ln,
);

impl<T: Signed> Abs for T {
	type Output = T;

	fn abs(self) -> Self::Output { Signed::abs(&self) }
}


macro_rules! binary_op_trait (
	($Trait: ident, $method: ident, $TraitAssign: ident, $method_assign: ident) => {
		pub trait $Trait<RHS=Self> {
			type Output;

			fn $method(self, rhs: RHS) -> Self::Output;
		}
	}
);

binary_op_trait!(Pow, pow, PowAssign, pow_assign);
binary_op_trait!(Log, log, LogAssign, log_assign);
binary_op_trait!(Max, max, MaxAssign, max_assign);
binary_op_trait!(Min, min, MinAssign, min_assign);

macro_rules! impl_binary_op_traits (
	($($GroupTrait: ident => $Trait: ident: $op_fn: ident),* $(,)*) => {$(
		impl<T: $GroupTrait, A: Into<T>> $Trait<A> for T {
			type Output = T;

			fn $op_fn(self, rhs: A) -> Self::Output { $GroupTrait::$op_fn(self, rhs.into())}
		}
	)*}
);

impl_binary_op_traits!(
	Float => Log: log,
	Float => Max: max,
	Float => Min: min,
);

pub trait PowAssign<RHS = Self> {
	fn pow_assign(&mut self, rhs: RHS);
}

pub trait Clamp<R> {
	type Output;

	fn clamp(self, min: R, max: R) -> Self::Output;
}

pub trait ClampAssign<R> {
	fn clamp_assign(&mut self, min: R, max: R);
}

pub fn clamp<T: Scalar>(x: T, min: T, max: T) -> T {
	if x < min { min } else if x > max { max } else { x }
}