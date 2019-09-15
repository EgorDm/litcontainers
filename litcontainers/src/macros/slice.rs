#[macro_export]
macro_rules! split_into {
	($storage: expr, $chunk_count: expr; RowAxis) => {
		{
			let mut storage = $storage.into_slice();
			assert!(storage.rows() >= $chunk_count, "Storage dimensions must be larger than the chunk count.");

			let storage_size = Size::new(D!(storage.rows()), storage.col_dim());
			let storage_stride = storage.strides();
			let mut storage = storage.transmute_dims_inplace(storage_size, storage_stride);

			let mut ret = Vec::new();
			let mut chunk_size = D!(storage.rows() / $chunk_count);
			for _ in 0..$chunk_count - 1 {
				let (l, r) = storage.split_at_row(chunk_size);
				ret.push(l);
				storage = r;
			}
			ret.push(storage);
			ret
		}
	};
	($storage: expr, $chunk_count: expr; ColAxis) => {
		{
			let mut storage = $storage.into_slice();
			assert!(storage.cols() >= $chunk_count, "Storage dimensions must be larger than the chunk count.");

			let storage_size = Size::new(storage.row_dim(), D!(storage.cols()));
			let storage_stride = storage.strides();
			let mut storage = storage.transmute_dims_inplace(storage_size, storage_stride);

			let mut ret = Vec::new();
			let mut chunk_size = D!(storage.cols() / $chunk_count);
			for _ in 0..$chunk_count - 1 {
				let (l, r) = storage.split_at_col(chunk_size);
				ret.push(l);
				storage = r;
			}
			ret.push(storage);
			ret
		}
	}
}