use num_traits::{Float, Signed};
pub use num_traits::pow::Pow;
use num_complex::Complex;

macro_rules! unary_op_trait (
	($($Trait: ident: $trait_fn: ident $(=> $TraitAssign: ident: $trait_assign_fn: ident)?),* $(,)*) => {$(
		pub trait $Trait {
			type Output;

			fn $trait_fn(self) -> Self::Output;
		}

		$(
			pub trait $TraitAssign {
				fn $trait_assign_fn(&mut self);
			}
		)?
	)*}
);

unary_op_trait!(
	ASin    : asin   => ASinAssign  : asin_assign,
	Sin     : sin    => SinAssign   : sin_assign,
	ACos    : acos   => ACosAssign  : acos_assign,
	Cos     : cos    => CosAssign   : cos_assign,
	Tan     : tan    => TanAssign   : tan_assign,
	ATan    : atan   => ATanAssign  : atan_assign,
	Exp     : exp    => ExpAssign   : exp_assign,
	Exp2    : exp2   => Exp2Assign  : exp2_assign,
	Ceil    : ceil   => CeilAssign  : ceil_assign,
	Floor   : floor  => FloorAssign : floor_assign,
	Round   : round  => RoundAssign : round_assign,
	Abs     : abs    => AbsAssign   : abs_assign,
	Sqrt    : sqrt   => SqrtAssign  : sqrt_assign,
	Log2    : log2   => Log2Assign  : log2_assign,
	Log10   : log10  => Log10Assign : log10_assign,
	Ln      : ln     => LnAssign    : ln_assign,
	Norm    : norm   => NormAssign  : norm_assign,
	NormSqr : norm_sqr => NormSqrAssign  : norm_sqr_assign,
);

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
	($($Trait: ident: $trait_fn: ident => $TraitAssign: ident: $trait_assign_fn: ident),* $(,)*) => {$(
		pub trait $Trait<RHS=Self> {
			type Output;

			fn $trait_fn(self, rhs: RHS) -> Self::Output;
		}
	)*}
);

binary_op_trait!(
	Log: log => LogAssign: log_assign,
	Max: max => MaxAssign: max_assign,
	Min: min => MinAssign: min_assign,
);

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

pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
	if x < min { min } else if x > max { max } else { x }
}

impl<T: PartialOrd, A: Into<T>> Clamp<A> for T {
	type Output = T;

	fn clamp(self, min: A, max: A) -> Self::Output { clamp(self, min.into(), max.into())}
}

impl<T: Float> Norm for Complex<T> {
	type Output = T;

	fn norm(self) -> Self::Output { Complex::norm(&self) }
}

impl<T: Float> NormSqr for Complex<T> {
	type Output = T;

	fn norm_sqr(self) -> Self::Output { Complex::norm_sqr(&self) }
}