use quote::ToTokens;
use std::convert::identity;
use syn::Ident;

pub fn generate_debug_impl(name: &Ident, fields: &syn::FieldsNamed) -> impl ToTokens {
    let name_str = name.to_string();
    let fields_name = fields
        .named
        .iter()
        .map(|f| f.ident.as_ref())
        .filter_map(identity)
        .collect::<Vec<&Ident>>();
    let fields_name_str = fields_name
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>();
    quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct(#name_str)
                    #(.field(#fields_name_str, &self.#fields_name))*
                    .finish()
            }
        }
    }
}

pub fn get_dirty_bits_count(fields: &syn::FieldsNamed) -> usize {
    (fields.named.len() / 32) + 1
}