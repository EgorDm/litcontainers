use syn::{Meta, NestedMeta, Ident};
use syn::export::Span;

pub type AttrTypeHolder = Vec<syn::Ident>;

pub fn get_member_by_name(ast: &syn::DeriveInput, _name: &str) -> Option<(syn::Ident, syn::Type)> {
	match ast.data {
		syn::Data::Struct(ref data_struct) => {
			match data_struct.fields {
				syn::Fields::Named(ref fields_named) => {
					for field in fields_named.named.iter() {
						match &field.ident {
							Some(ref ident) => {
								return Some((ident.clone(), field.ty.clone()))
							},
							_ => ()
						}
					}
				},
				_ => (),
			}
		},
		_ => panic!("Must be a struct"),
	}

	None
}

pub fn get_member_by_attr(ast: &syn::DeriveInput, attr_name: &str) -> Option<(syn::Ident, syn::Type)> {
	match ast.data {
		syn::Data::Struct(ref data_struct) => {
			match data_struct.fields {
				syn::Fields::Named(ref fields_named) => {
					for field in fields_named.named.iter() {
						for attr in field.attrs.iter() {
							let meta = attr.parse_meta().unwrap();
							match meta {
								Meta::Word(ref ident) if ident == attr_name => {
									return field.ident.clone().map(|v| (v, field.ty.clone()));
								},
								_ => (),
							}
						}
					}
				},
				_ => (),
			}
		},
		_ => panic!("Must be a struct"),
	}

	None
}

pub fn get_class_attr_type_holder(ast: &syn::DeriveInput, attr_name: &str) -> Option<AttrTypeHolder> {
	for a in &ast.attrs {
		let meta = a.parse_meta().unwrap();

		match meta {
			Meta::List(ref a) if a.ident == attr_name => {
				let mut ret = AttrTypeHolder::new();

				for arg in &a.nested {
					match arg {
						NestedMeta::Meta(Meta::Word(ref ident)) => ret.push(ident.clone()),
						_ => ()
					}
				}

				return Some(ret);
			},
			_ => (),
		}
	}

	None
}