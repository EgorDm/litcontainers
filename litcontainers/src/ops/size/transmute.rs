use crate::{Storage, Element, Slice, PtrStorage, Strided, StorageSize, SliceBase, PtrStorageMut, StorageMut, SliceMut, Dim};

pub trait Transposable<T: Element>: Storage<T>
{
	fn t(&self) -> Slice<T, Self::Cols, Self::ColStride, Self::Rows, Self::RowStride> {
		SliceBase::new(unsafe {
			PtrStorage::new(
				self.as_ptr(),
				self.size().transpose(),
				self.strides().transpose(),
			)
		}).into()
	}

	fn transmute_dims<SZ: StorageSize, ST: Strided>(&self, size: SZ, stride: ST)
		-> Slice<T, SZ::Rows, ST::RowStride, SZ::Cols, ST::ColStride>
	{
		let new_size = (size.rows() - 1) * stride.row_stride() + (size.cols() - 1) * stride.col_stride();
		assert!(new_size < self.len(), "Transmute is out of bounds!");
		SliceBase::new(unsafe { PtrStorage::new(self.as_ptr(), size.size(), stride.strides()) }).into()
	}

	fn transmute_stride_dims<ST: Strided>(&self, stride: ST)
		-> Slice<T, Self::Rows, ST::RowStride, Self::Cols, ST::ColStride>
	{ self.transmute_dims(self.size(), stride) }
}

impl<T: Element, S: Storage<T>> Transposable<T> for S {}

pub trait TransposableMut<T: Element>: StorageMut<T>
{
	fn t_mut(&mut self) -> SliceMut<T, Self::Cols, Self::ColStride, Self::Rows, Self::RowStride> {
		SliceBase::new(unsafe {
			PtrStorageMut::new(
				self.as_ptr_mut(),
				self.size().transpose(),
				self.strides().transpose(),
			)
		}).into()
	}

	fn transmute_dims_mut<SZ: StorageSize, ST: Strided>(&mut self, size: SZ, stride: ST)
		-> SliceMut<T, SZ::Rows, ST::RowStride, SZ::Cols, ST::ColStride>
	{
		let new_size = (size.rows() - 1) * stride.row_stride() + (size.cols() - 1) * stride.col_stride();
		assert!(new_size < self.len(), "Transmute is out of bounds!");
		SliceBase::new(unsafe { PtrStorageMut::new(self.as_ptr_mut(), size.size(), stride.strides()) }).into()
	}

	fn transmute_stride_dims_mut<ST: Strided>(&mut self, stride: ST)
		-> SliceMut<T, Self::Rows, ST::RowStride, Self::Cols, ST::ColStride>
	{ self.transmute_dims_mut(self.size(), stride) }
}

impl<T: Element, S: StorageMut<T>> TransposableMut<T> for S {}

pub trait TransposableOwned<'a, T: Element>: Storage<T>
{
	fn transmute_dims_inplace<SZ: StorageSize, ST: Strided>(self, size: SZ, stride: ST)
		-> Slice<'a, T, SZ::Rows, ST::RowStride, SZ::Cols, ST::ColStride>
	{
		let new_size = (size.rows() - 1) * stride.row_stride() + (size.cols() - 1) * stride.col_stride();
		assert!(new_size < self.len(), "Transmute is out of bounds!");
		SliceBase::new(unsafe { PtrStorage::new(self.as_ptr(), size.size(), stride.strides()) }).into()
	}

	fn transmute_stride_dims_inplace<ST: Strided>(self, stride: ST)
		-> Slice<'a, T, Self::Rows, ST::RowStride, Self::Cols, ST::ColStride>
	{
		let size = self.size();
		self.transmute_dims_inplace(size, stride)
	}
}

impl<'a, T: Element, R: Dim, C: Dim, RS: Dim, CS: Dim> TransposableOwned<'a, T> for Slice<'a, T, R, RS, C, CS> {}