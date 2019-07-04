use syn::{Ident};
use syn::export::Span;
use crate::utils::*;

#[derive(Debug, Clone)]
pub struct StorageMetaInfo {
	pub element_type: Ident,
	pub row_dim_type: Ident,
	pub col_dim_type: Ident,
	pub row_stride_type: Ident,
	pub col_stride_type: Ident,
}

pub fn parse_storage_meta(ast: &syn::DeriveInput) -> StorageMetaInfo {
	let attr_holder = get_class_attr_type_holder(&ast, "dimension_fields")
		.unwrap_or(Vec::new());

	let extract_attr = |index, default| attr_holder.get(index).cloned()
		.unwrap_or(Ident::new(default, Span::call_site()));

	StorageMetaInfo {
		element_type: extract_attr(0, "T"),
		row_dim_type: extract_attr(1, "R"),
		col_dim_type: extract_attr(2, "C"),
		row_stride_type: extract_attr(3, "RS"),
		col_stride_type: extract_attr(4, "CS"),
	}
}

macro_rules! storage_based_derive (
	($input: ident, $ast: ident, $name: ident, $impl_generics: ident, $ty_generics: ident,
		$where_clause: ident, $storage_field: ident, $storage_type: ident, $storage: ident) => {
		let $ast: DeriveInput = parse_macro_input!($input as DeriveInput);
		let $name = &$ast.ident;
		let ($impl_generics, $ty_generics, $where_clause) = $ast.generics.split_for_impl();
		let ($storage_field, $storage_type) = get_member_by_attr(&$ast, "storage_field")
				.unwrap_or(get_member_by_name(&$ast, "storage").unwrap());
		let $storage = parse_storage_meta(&$ast);
	}
);