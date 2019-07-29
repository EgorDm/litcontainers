use crate::format::*;
use crate::storage::*;
use super::slice::*;
use crate::container::Container;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

macro_rules! impl_binary_dual_op (
	($OpTrait: ident, $op_fn: ident, $OpAssignTrait: ident, $op_assign_fn: ident) => {
		// Add conatiner
		impl<'a, T, R, C, S, TR, RR, CR, SR> $OpTrait<&Container<TR, RR, CR, SR>> for SliceBase<'a, T, R, C, S>
			where
				T: Scalar + $OpAssignTrait<TR>, R: Dim, C: Dim, S: Storage<T, R, C>,
				TR: Scalar, RR: Dim, CR: Dim, SR: StorageMut<TR, RR, CR>
		{
			type Output = Container<T, R, C, S::OwnedType>;

			fn $op_fn(self, rhs: &Container<TR, RR, CR, SR>) -> Self::Output {
				assert!(self.equal_size(rhs), "Rhs must have the same size!");
				let mut ret = self.owned();
				for (o, s) in ret.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
				ret
			}
		}

		impl<'a, T, R, C, S, TR, RR, CR, SR> $OpTrait<&Container<TR, RR, CR, SR>> for &SliceBase<'a, T, R, C, S>
			where
				T: Scalar + $OpAssignTrait<TR>, R: Dim, C: Dim, S: Storage<T, R, C>,
				TR: Scalar, RR: Dim, CR: Dim, SR: StorageMut<TR, RR, CR>
		{
			type Output = Container<T, R, C, S::OwnedType>;

			fn $op_fn(self, rhs: &Container<TR, RR, CR, SR>) -> Self::Output {
				assert!(self.equal_size(rhs), "Rhs must have the same size!");
				let mut ret = self.clone_owned();
				for (o, s) in ret.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
				ret
			}
		}

		// Add slice
		impl<'a, 'b, T, R, C, S, TR, RR, CR, SR> $OpTrait<&SliceBase<'a, TR, RR, CR, SR>> for SliceBase<'b, T, R, C, S>
			where
				T: Scalar + $OpAssignTrait<TR>, R: Dim, C: Dim, S: Storage<T, R, C>,
				TR: Scalar, RR: Dim, CR: Dim, SR: Storage<TR, RR, CR>
		{
			type Output = Container<T, R, C, S::OwnedType>;

			fn $op_fn(self, rhs: &SliceBase<'a, TR, RR, CR, SR>) -> Self::Output {
				assert!(self.equal_size(rhs), "Rhs must have the same size!");
				let mut ret = self.owned();
				for (o, s) in ret.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
				ret
			}
		}

		impl<'a, 'b, T, R, C, S, TR, RR, CR, SR> $OpTrait<&SliceBase<'a, TR, RR, CR, SR>> for &SliceBase<'b, T, R, C, S>
			where
				T: Scalar + $OpAssignTrait<TR>, R: Dim, C: Dim, S: Storage<T, R, C>,
				TR: Scalar, RR: Dim, CR: Dim, SR: Storage<TR, RR, CR>
		{
			type Output = Container<T, R, C, S::OwnedType>;

			fn $op_fn(self, rhs: &SliceBase<'a, TR, RR, CR, SR>) -> Self::Output {
				assert!(self.equal_size(rhs), "Rhs must have the same size!");
				let mut ret = self.clone_owned();
				for (o, s) in ret.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
				ret
			}
		}

		// Add scalar
		impl<'a, T, R, C, S, TR> $OpTrait<TR> for SliceBase<'a, T, R, C, S>
			where
				T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>,
				TR: Scalar, T: $OpAssignTrait<TR>
		{
			type Output = Container<T, R, C, S::OwnedType>;

			fn $op_fn(self, rhs: TR) -> Self::Output {
				let mut ret = self.owned();
				ret.map_inplace(move |v| v.$op_assign_fn(rhs));
				ret
			}
		}

		impl<'a, T, R, C, S, TR> $OpTrait<TR> for &SliceBase<'a, T, R, C, S>
			where
				T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>,
				TR: Scalar, T: $OpAssignTrait<TR>
		{
			type Output = Container<T, R, C, S::OwnedType>;

			fn $op_fn(self, rhs: TR) -> Self::Output {
				let mut ret = self.clone_owned();
				ret.map_inplace(move |v| v.$op_assign_fn(rhs));
				ret
			}
		}

		// Add assign
		impl<'a, T, R, C, S, TR, RR, CR, SR> $OpAssignTrait<&Container<TR, RR, CR, SR>> for SliceBase<'a, T, R, C, S>
			where
				T: Scalar + $OpAssignTrait<TR>, R: Dim, C: Dim, S: StorageMut<T, R, C>,
				TR: Scalar, RR: Dim, CR: Dim, SR: StorageMut<TR, RR, CR>
		{
			fn $op_assign_fn(&mut self, rhs: &Container<TR, RR, CR, SR>) {
				for (o, s) in self.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
			}
		}

		impl<'a, T, R, C, S, TR, RR, CR, SR> $OpAssignTrait<&SliceBase<'a, TR, RR, CR, SR>> for SliceBase<'a, T, R, C, S>
			where
				T: Scalar + $OpAssignTrait<TR>, R: Dim, C: Dim, S: StorageMut<T, R, C>,
				TR: Scalar, RR: Dim, CR: Dim, SR: Storage<TR, RR, CR>
		{
			fn $op_assign_fn(&mut self, rhs: &SliceBase<TR, RR, CR, SR>) {
				for (o, s) in self.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
			}
		}

		impl<'a, T, R, C, S, TR> $OpAssignTrait<TR> for SliceBase<'a, T, R, C, S>
			where
				T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>,
				TR: Scalar, T: $OpAssignTrait<TR>
		{
			fn $op_assign_fn(&mut self, rhs: TR) {
				self.map_inplace(|v| v.$op_assign_fn(rhs));
			}
		}
	}
);

impl_binary_dual_op!(Add, add, AddAssign, add_assign);
impl_binary_dual_op!(Sub, sub, SubAssign, sub_assign);
impl_binary_dual_op!(Mul, mul, MulAssign, mul_assign);
impl_binary_dual_op!(Div, div, DivAssign, div_assign);

impl<'a, T, R, C, S> Neg for &SliceBase<'a, T, R, C, S>
	where
		T: Scalar + Neg<Output=T>, R: Dim, C: Dim, S: Storage<T, R, C>
{
	type Output = Container<T, R, C, S::OwnedType>;

	fn neg(self) -> Self::Output {
		let mut ret = self.clone_owned();
		ret.mapv_inplace(|v| v.neg());
		ret
	}
}