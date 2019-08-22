use crate::Dim;

pub enum AxisType {
	Row,
	Col,
}

pub trait Axis<R: Dim, C: Dim> {
	type RetType: Dim;
	type Parallel: Axis<R, C>;

	fn axis_type() -> AxisType;

	fn get_axis(r: R, c: C) -> Self::RetType;

	fn get_val<V>(r: V, c: V) -> V;
}

pub struct RowAxis;

impl<R: Dim, C: Dim> Axis<R, C> for RowAxis {
	type RetType = R;
	type Parallel = ColAxis;

	fn axis_type() -> AxisType { AxisType::Row }

	fn get_axis(r: R, c: C) -> Self::RetType { r }

	fn get_val<V>(r: V, c: V) -> V { r }
}

pub struct ColAxis;

impl<R: Dim, C: Dim> Axis<R, C> for ColAxis {
	type RetType = C;
	type Parallel = RowAxis;

	fn axis_type() -> AxisType { AxisType::Col }

	fn get_axis(r: R, c: C) -> Self::RetType { c }

	fn get_val<V>(r: V, c: V) -> V { c }
}


//use crate::Dim;
//use std::marker::PhantomData;
//

//
//pub trait Axis<S: Dim> {
//	type RowDim: Dim;
//	type ColDim: Dim;
//
//	fn axis() -> AxisType;
//}
//
//pub struct RowAxis<R, C> {
//	_phantoms: PhantomData<(R, C)>
//}
//
//impl<R, C> RowAxis<R, C> {
//	fn new() -> Self { Self { _phantoms: PhantomData }}
//}
//
//impl<R, C> Axis<R> for RowAxis<R, C> {
//	type RowDim = R;
//	type ColDim = C;
//
//	fn axis() -> AxisType { AxisType::Row }
//}
//
//pub struct ColDim<R, C> {
//	_phantoms: PhantomData<(R, C)>
//}
//
//impl<R, C> ColDim<R, C> {
//	fn new() -> Self { Self { _phantoms: PhantomData }}
//}
//
//impl<R, C> Axis<C> for ColDim<R, C> {
//	type RowDim = R;
//	type ColDim = C;
//
//	fn axis() -> AxisType { AxisType::Col }
//}
