use crate::{Dim};

pub enum AxisType {
	Row,
	Col,
}

pub trait AxisSelector<R, C> {
	type Result;

	fn select(r: R, c: C) -> Self::Result;
}

pub trait Axis<R: Dim, C: Dim> {
	type RetType: Dim;
	type Parallel: Axis<R, C>;

	fn axis_type() -> AxisType;

	fn parallel() -> Self::Parallel;

	fn get_axis(r: R, c: C) -> Self::RetType;

	fn get_val<V>(r: V, c: V) -> V;
}

pub type AxisRes<A, R, C> = <A as Axis<R, C>>::RetType;
pub type AxisParallelRes<A, R, C> = AxisRes<AxisParallel<A, R, C>, R, C>;
pub type AxisParallel<A, R, C> = <A as Axis<R, C>>::Parallel;

pub struct RowAxis;

impl<R: Dim, C: Dim> Axis<R, C> for RowAxis {
	type RetType = R;
	type Parallel = ColAxis;

	fn axis_type() -> AxisType { AxisType::Row }

	fn parallel() -> Self::Parallel { ColAxis }

	fn get_axis(r: R, _c: C) -> Self::RetType { r }

	fn get_val<V>(r: V, _c: V) -> V { r }
}

impl<R, C> AxisSelector<R, C> for RowAxis {
	type Result = R;

	fn select(r: R, _: C) -> Self::Result { r }
}

pub struct ColAxis;

impl<R: Dim, C: Dim> Axis<R, C> for ColAxis {
	type RetType = C;
	type Parallel = RowAxis;

	fn axis_type() -> AxisType { AxisType::Col }

	fn parallel() -> Self::Parallel { RowAxis }

	fn get_axis(_r: R, c: C) -> Self::RetType { c }

	fn get_val<V>(_r: V, c: V) -> V { c }
}

impl<R, C> AxisSelector<R, C> for ColAxis {
	type Result = C;

	fn select(_: R, c: C) -> Self::Result { c }
}

pub type Rows = RowAxis;
pub type Cols = ColAxis;