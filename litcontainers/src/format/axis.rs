//use crate::Dim;
//use std::marker::PhantomData;
//
//pub enum AxisType {
//	Row,
//	Col,
//}
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
