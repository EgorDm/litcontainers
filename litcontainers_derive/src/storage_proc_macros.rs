use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use super::storage_meta_info::*;
use crate::utils::*;

pub fn strided_storage_derive(input: TokenStream) -> TokenStream {
	storage_based_derive!(input, ast, name, impl_generics, ty_generics, where_clause, storage_field, storage_type, storage);

	let StorageMetaInfo { row_dim_type, col_dim_type, .. } = storage;
	let (size_field, _) = get_member_by_attr(&ast, "stride_field")
		.unwrap_or(get_member_by_name(&ast, "stride").unwrap_or((storage_field.clone(), storage_type.clone())));


	TokenStream::from(quote! {
		impl #impl_generics Strided
			for #name #ty_generics #where_clause
		{
			type RowStride = <#storage_type as Strided>::RowStride;
			type ColStride = <#storage_type as Strided>::ColStride;

			#[inline]
			fn row_stride_dim(&self) -> Self::RowStride { self.#storage_field.row_stride_dim() }

			#[inline]
			fn col_stride_dim(&self) -> Self::ColStride { self.#storage_field.col_stride_dim() }
		}
	})
}

pub fn sized_storage_derive(input: TokenStream) -> TokenStream {
	storage_based_derive!(input, ast, name, impl_generics, ty_generics, where_clause, storage_field, storage_type, storage);

	let StorageMetaInfo { row_dim_type, col_dim_type, .. } = storage;

	TokenStream::from(quote! {
		impl #impl_generics StorageSize
			for #name #ty_generics #where_clause
		{
			type Rows = <#storage_type as StorageSize>::Rows;
			type Cols = <#storage_type as StorageSize>::Cols;

			#[inline]
			fn row_dim(&self) -> Self::Rows { self.#storage_field.row_dim() }

			#[inline]
			fn col_dim(&self) -> Self::Cols { self.#storage_field.col_dim() }
		}
	})
}

pub fn storage_derive(input: TokenStream) -> TokenStream {
	storage_based_derive!(input, ast, name, impl_generics, ty_generics, where_clause, storage_field, storage_type, storage);

	let StorageMetaInfo { element_type, row_dim_type, col_dim_type, .. } = storage;

	TokenStream::from(quote! {
		impl #impl_generics Storage<#element_type>
			for #name #ty_generics #where_clause
		{
			#[inline]
			fn as_ptr(&self) -> *const T { self.#storage_field.as_ptr() }
		}

		impl #impl_generics InplaceForeach<#element_type>
			for #name #ty_generics #where_clause
		{
			fn foreach<F: FnMut(&T)>(&self, f: F) {
				self.#storage_field.foreach(f)
			}
		}

		impl #impl_generics Index<usize>
			for #name #ty_generics #where_clause
		{
			type Output = T;

			fn index(&self, index: usize) -> &Self::Output {
				self.#storage_field.index(index)
			}
		}

	})
}

pub fn storage_mut_derive(input: TokenStream) -> TokenStream {
	storage_based_derive!(input, ast, name, impl_generics, ty_generics, where_clause, storage_field, _storage_type, storage);

	let StorageMetaInfo { element_type, row_dim_type, col_dim_type, .. } = storage;

	TokenStream::from(quote! {
		impl #impl_generics StorageMut<#element_type>
			for #name #ty_generics #where_clause
		{
			#[inline]
			fn as_ptr_mut(&mut self) -> *mut #element_type { self.#storage_field.as_ptr_mut() }
		}
	})
}

pub fn ownable_derive(input: TokenStream) -> TokenStream {
	storage_based_derive!(input, ast, name, impl_generics, ty_generics, where_clause, storage_field, storage_type, storage);

	let StorageMetaInfo { element_type, .. } = storage;

	TokenStream::from(quote! {
		impl #impl_generics Ownable<#element_type>
		    for #name #ty_generics #where_clause
		{
			type OwnedType = <#storage_type as Ownable<#element_type>>::OwnedType;

			#[inline]
			fn owned(self) -> Container<T, Self::OwnedType> { self.#storage_field.owned() }

			#[inline]
			fn clone_owned(&self) -> Container<T, Self::OwnedType> { self.#storage_field.clone_owned() }
		}
	})
}