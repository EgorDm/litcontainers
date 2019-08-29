macro_rules! operation_scalar_binary_op (
	($($Name: ident $(| $NameAssign: ident)? => $Trait: ident: $op_fn: ident),* $(,)*) => {$(
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

		$(
			#[derive(new)]
			pub struct $NameAssign<'a, L, T>
				where L: InplaceMap<T> + StorageSize, T: Element
			{
				left: &'a mut L,
				right: T,
			}

			impl<'a, L, T> Operation for $NameAssign<'a, L, T>
				where L: InplaceMap<T> + StorageSize, T: $Trait<T, Output=T> + Element
			{
				type Type = T;
				type Rows = L::Rows;
				type Cols = L::Cols;
				type Result = ();

				fn apply(self) -> Self::Result {
					let r = self.right;
					self.left.map_inplace(|v| *v = (*v).$op_fn(r.clone()))
				}
			}
		)?
	)*}
);

macro_rules! operation_scalar_binary_rev_op (
	($($Name: ident $(| $NameAssign: ident)? => $Trait: ident: $op_fn: ident),* $(,)*) => {$(
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

		$(
			#[derive(new)]
			pub struct $NameAssign<'a, L, T>
				where L: InplaceMap<T> + StorageSize, T: Element
			{
				left: &'a mut L,
				right: T,
			}

			impl<'a, L, T> Operation for $NameAssign<'a, L, T>
				where L: InplaceMap<T> + StorageSize, T: $Trait<T, Output=T> + Element
			{
				type Type = T;
				type Rows = L::Rows;
				type Cols = L::Cols;
				type Result = ();

				fn apply(self) -> Self::Result {
					let r = self.right;
					self.left.map_inplace(|v| *v = r.clone().$op_fn(*v))
				}
			}
		)?
	)*}
);

macro_rules! operation_unary_op (
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

macro_rules! operation_storage_binary_op (
	($($Name: ident $(| $NameAssign: ident)? => $Trait: ident: $op_fn: ident),* $(,)*) => {$(
		#[derive(new)]
		pub struct $Name<L, R>
			where L: Operation, R: Operation
		{
			left: L,
			right: R,
		}

		impl<L, R> Operation for $Name<L, R>
			where L: Operation, L::Result: InplaceMapOrdered<L::Type>,
			      R: Operation, R::Result: IntoOrderedIterator<R::Type>,
			      L::Type: $Trait<R::Type, Output=L::Type>
		{
			type Type = L::Type;
			type Rows = L::Rows;
			type Cols = L::Cols;
			type Result = L::Result;

			fn apply(self) -> Self::Result {
				let mut l = self.left.apply();
				let r = self.right.apply();
				l.mapv_inplace_zip_ordered(r.into_ordered_iter(), |l, r| l.$op_fn(r));
				l
			}
		}

		$(
			#[derive(new)]
			pub struct $NameAssign<'a, L, R>
				where L: InplaceMapOrdered<R::Type> + StorageSize,
					  R: Operation, R::Result: IntoOrderedIterator<R::Type>
			{
				left: &'a mut L,
				right: R,
			}

			impl<'a, L, R> Operation for $NameAssign<'a, L, R>
				where L: InplaceMapOrdered<R::Type> + StorageSize,
					  R: Operation, R::Result: IntoOrderedIterator<R::Type>,
					  R::Type: $Trait<R::Type, Output=R::Type>,
			{
				type Type = R::Type;
				type Rows = L::Rows;
				type Cols = L::Cols;
				type Result = ();

				fn apply(self) -> Self::Result {
					let r = self.right.apply();
					self.left.mapv_inplace_zip_ordered(r.into_ordered_iter(), |l, r| l.$op_fn(r));
				}
			}
		)?
	)*}
);

macro_rules! operation_storage_binary_rev_op (
	($($Name: ident $(| $NameAssign: ident)? => $Trait: ident: $op_fn: ident),* $(,)*) => {$(
		#[derive(new)]
		pub struct $Name<L, R>
			where L: Operation, R: Operation
		{
			left: L,
			right: R,
		}

		impl<L, R, LS, RS> Operation for $Name<L, R>
			where L: Operation<Result=LS>, LS: IntoOrderedIterator<L::Type>,
			      R: Operation<Result=RS>, RS: InplaceMapOrdered<R::Type>,
			      L::Type: $Trait<R::Type, Output=R::Type>
		{
			type Type = R::Type;
			type Rows = R::Rows;
			type Cols = R::Cols;
			type Result = R::Result;

			fn apply(self) -> Self::Result {
				let l = self.left.apply();
				let mut r = self.right.apply();
				r.mapv_inplace_zip_ordered(l.into_ordered_iter(), |r, l| l.$op_fn(r));
				r
			}
		}

		$(
			#[derive(new)]
			pub struct $NameAssign<'a, L, R>
				where L: InplaceMapOrdered<R::Type> + StorageSize,
					  R: Operation, R::Result: IntoOrderedIterator<R::Type>
			{
				left: &'a mut L,
				right: R,
			}

			impl<'a, L, R> Operation for $NameAssign<'a, L, R>
				where L: InplaceMapOrdered<R::Type> + StorageSize,
					  R: Operation, R::Result: IntoOrderedIterator<R::Type>,
					  R::Type: $Trait<R::Type, Output=R::Type>,
			{
				type Type = R::Type;
				type Rows = L::Rows;
				type Cols = L::Cols;
				type Result = ();

				fn apply(self) -> Self::Result {
					let r = self.right.apply();
					self.left.mapv_inplace_zip_ordered(r.into_ordered_iter(), |l, r| r.$op_fn(l));
				}
			}
		)?
	)*}
);

macro_rules! operation_group_unary (
	($($Name: ident: $op_fn: ident => $Trait: ident),* $(,)*) => {
		$(
			fn $op_fn(self) -> $Name<Self::OpType>
				where <Self::OpType as Operation>::Type: $Trait<Output=<Self::OpType as Operation>::Type>
			{
				$Name::new(self.into_operation())
			}
		)*
	}
);

macro_rules! operation_group_scalar_binary (
	($($Name: ident: $op_fn: ident $(| $NameAssign: ident: $op_fn_assign: ident)? => $Trait: ident),* $(,)*) => {
		$(
			fn $op_fn<O>(self, rhs: O) -> $Name<Self::OpType>
				where <Self::OpType as Operation>::Type: $Trait<<Self::OpType as Operation>::Type, Output=<Self::OpType as Operation>::Type> + From<O>
			{
				$Name::new(self.into_operation(), rhs.into())
			}

			$(
				fn $op_fn_assign<O, T>(&mut self, rhs: O) -> $NameAssign<Self, T>
					where T: $Trait<T, Output=T> + Element + From<O>,
					      Self: InplaceMap<T> + StorageSize
				{
					$NameAssign::new(self, rhs.into())
				}
			)?
		)*
	}
);

macro_rules! operation_group_scalar_binary_rev (
	($($Name: ident: $op_fn: ident $(| $NameAssign: ident: $op_fn_assign: ident)? => $Trait: ident),* $(,)*) => {
		$(
			fn $op_fn<O>(self, lhs: O) -> $Name<Self::OpType>
				where <Self::OpType as Operation>::Type: $Trait<<Self::OpType as Operation>::Type, Output=<Self::OpType as Operation>::Type> + From<O>
			{
				$Name::new(lhs.into(), self.into_operation())
			}

			$(
				fn $op_fn_assign<O, T>(&mut self, rhs: O) -> $NameAssign<Self, T>
					where T: $Trait<T, Output=T> + Element + From<O>,
					      Self: InplaceMap<T> + StorageSize
				{
					$NameAssign::new(self, rhs.into())
				}
			)?
		)*
	}
);

macro_rules! operation_group_storage_binary (
	($($Name: ident: $op_fn: ident $(| $NameAssign: ident: $op_fn_assign: ident)? => $Trait: ident),* $(,)*) => {
		$(
			fn $op_fn<O>(self, rhs: O) -> $Name<Self::OpType, O::OpType>
				where O: IntoOperation, <O::OpType as Operation>::Result: IntoOrderedIterator<<O::OpType as Operation>::Type>,
					  <Self::OpType as Operation>::Type: $Trait<<O::OpType as Operation>::Type, Output=<Self::OpType as Operation>::Type>,
			{
				$Name::new(self.into_operation(), rhs.into_operation())
			}

			$(
				fn $op_fn_assign<O, T>(&mut self, rhs: O) -> $NameAssign<Self, O::OpType>
					where T: $Trait<T, Output=T> + Element,
					      O: IntoOperation, <O::OpType as Operation>::Result: IntoOrderedIterator<T>,
				          O::OpType: Operation<Type=T>,
					      Self: InplaceMapOrdered<T> + StorageSize
				{
					$NameAssign::new(self, rhs.into_operation())
				}
			)?
		)*
	}
);

macro_rules! operation_group_storage_binary_rev (
	($($Name: ident: $op_fn: ident => $Trait: ident),* $(,)*) => {
		$(
			fn $op_fn<O>(self, lhs: O) -> $Name<O::OpType, Self::OpType>
				where O: IntoOperation, <O::OpType as Operation>::Result: IntoOrderedIterator<<O::OpType as Operation>::Type>,
					  <O::OpType as Operation>::Type: $Trait<<Self::OpType as Operation>::Type, Output=<Self::OpType as Operation>::Type>
			{
				$Name::new(lhs.into_operation(), self.into_operation())
			}
		)*
	}
);