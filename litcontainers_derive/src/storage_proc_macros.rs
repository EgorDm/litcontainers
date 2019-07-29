use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use super::storage_meta_info::*;
use crate::utils::*;


pub fn sized_storage_derive(input: TokenStream) -> TokenStream {
	storage_based_derive!(input, ast, name, impl_generics, ty_generics, where_clause, storage_field, _storage_type, storage);

	let StorageMetaInfo { row_dim_type, col_dim_type, .. } = storage;

	TokenStream::from(quote! {
		impl #impl_generics SizedStorage<#row_dim_type, #col_dim_type>
			for #name #ty_generics #where_clause
		{
			#[inline]
			fn row_dim(&self) -> #row_dim_type { self.#storage_field.row_dim() }

			#[inline]
			fn col_dim(&self) -> #col_dim_type { self.#storage_field.col_dim() }
		}
	})
}

pub fn storage_derive(input: TokenStream) -> TokenStream {
	storage_based_derive!(input, ast, name, impl_generics, ty_generics, where_clause, storage_field, storage_type, storage);

	let StorageMetaInfo { element_type, row_dim_type, col_dim_type, .. } = storage;
	let impl_type = quote! { Storage<#element_type, #row_dim_type, #col_dim_type> };

	TokenStream::from(quote! {
		impl #impl_generics #impl_type
			for #name #ty_generics #where_clause
		{
			type RStride = <#storage_type as #impl_type>::RStride;
			type CStride = <#storage_type as #impl_type>::CStride;

			#[inline]
			fn row_stride_dim(&self) -> Self::RStride { self.#storage_field.row_stride_dim() }

			#[inline]
			fn col_stride_dim(&self) -> Self::CStride { self.#storage_field.col_stride_dim() }

			#[inline]
			unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const #element_type { self.#storage_field.get_index_ptr_unchecked(i) }
		}
	})
}

pub fn storage_mut_derive(input: TokenStream) -> TokenStream {
	storage_based_derive!(input, ast, name, impl_generics, ty_generics, where_clause, storage_field, _storage_type, storage);

	let StorageMetaInfo { element_type, row_dim_type, col_dim_type, .. } = storage;

	TokenStream::from(quote! {
		impl #impl_generics StorageMut<#element_type, #row_dim_type, #col_dim_type>
			for #name #ty_generics #where_clause
		{
			unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut #element_type {
				self.#storage_field.get_index_mut_ptr_unchecked(i)
			}

			fn map_inplace<F: FnMut(&mut #element_type)>(&mut self, f: F) { self.#storage_field.map_inplace(f) }
		}
	})
}