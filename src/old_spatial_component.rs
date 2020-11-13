use crate::utils::generate_debug_impl;
use crate::utils::get_dirty_bits_count;
use proc_macro::TokenStream;
use quote::ToTokens;
use std::convert::identity;
use syn::parse_macro_input;
use syn::AttributeArgs;
use syn::Ident;
use syn::ItemStruct;

const VALIDATE_INDEX_ERROR: &str = "Unless you are using custom component replication code, this is most likely caused by a code generation bug. Please contact nebulis support if you encounter this issue.";

fn generate_constructor(fields: &syn::FieldsNamed) -> impl ToTokens {
    let parameters = &fields.named;
    let body = fields
        .named
        .iter()
        .map(|f| f.ident.as_ref())
        .filter_map(identity)
        .fold(quote! {}, |acc, val| quote! { #val, #acc });
    let bits = (0..get_dirty_bits_count(fields))
        .map(|_| 0u32)
        .collect::<Vec<u32>>();
    quote! {
        pub fn new(#parameters) -> Self {
            Self {
                dirty_bits: [#(#bits),*],
                #body
            }
        }
    }
}

fn generate_getter(field: &syn::Field) -> impl ToTokens {
    if let Some(ident) = &field.ident {
        let ty = &field.ty;
        let getter = format_ident!("get_{}", ident);
        quote! {
            pub fn #getter(&self) -> &#ty {
                &self.#ident
            }
        }
    } else {
        quote! {}
    }
}

fn generate_setter(field: &syn::Field, field_index: usize) -> impl ToTokens {
    if let Some(ident) = &field.ident {
        let ty = &field.ty;
        let getter = format_ident!("set_{}", ident);
        quote! {
            pub fn #getter(&mut self, data: #ty) -> Result<(), &'static str> {
                self.mark_data_dirty(#field_index)?;
                self.#ident = data;
                Ok(())
            }
        }
    } else {
        quote! {}
    }
}

fn generate_getter_setter(field: &syn::Field, field_index: usize) -> impl ToTokens {
    let getter = generate_getter(&field);
    let setter = generate_setter(&field, field_index);
    quote! {
        #getter

        #setter
    }
}

fn generate_is_data_dirty(fields: &syn::FieldsNamed) -> impl ToTokens {
    let bits = (0..get_dirty_bits_count(fields))
        .into_iter()
        .collect::<Vec<usize>>();
    quote! {
        fn is_data_dirty(&self) -> bool {
            let mut data_dirty = false;
            #(data_dirty |= (self.dirty_bits[#bits] != 0x0);)*
            data_dirty
        }
    }
}

fn generate_mark_data_dirty() -> impl ToTokens {
    quote! {
        fn mark_data_dirty(&mut self, index: usize) -> Result<(), &'static str> {
            let index = self.validate_index(index)?;
            let dirty_bits_byte_index = index >> 5;
            self.dirty_bits[dirty_bits_byte_index] |= (0x1 << (index & 31usize)) as u32;
            Ok(())
        }
    }
}

fn generate_validate_index(fields: &syn::FieldsNamed) -> impl ToTokens {
    let bits_count = ((get_fields_number(fields) as isize) - 1).min(0) as usize;
    let error = format!(
        "\"index\" argument out of range. Valid range is [0, {}]. {}",
        bits_count, VALIDATE_INDEX_ERROR
    );
    quote! {
        fn validate_index(&self, index: usize) -> Result<usize, &'static str> {
            if index < 0usize || index > #bits_count {
                Err(#error)
            } else {
                Ok(index)
            }
        }
    }
}

fn get_fields_number(fields: &syn::FieldsNamed) -> usize {
    fields.named.len()
}

fn generate_base_impl(name: &Ident, fields: &syn::FieldsNamed, id: u32) -> impl ToTokens {
    let constructor = generate_constructor(fields);
    let validator = generate_validate_index(fields);
    let is_data_dirty = generate_is_data_dirty(fields);
    let marker = generate_mark_data_dirty();
    let mut gets_sets = vec![];
    for (field_index, field) in (&fields.named).iter().enumerate() {
        gets_sets.push(generate_getter_setter(&field, field_index));
    }
    quote! {
        impl #name {
            const ID: u32 = #id;

            #constructor
            #validator
            #is_data_dirty
            #marker

            #(#gets_sets)*
        }
    }
}

fn get_id(args: AttributeArgs) -> Option<u32> {
    if let syn::NestedMeta::Lit(lit) = &args[0] {
        if let syn::Lit::Int(lit_int) = lit {
            if let Ok(base10_id) = lit_int.base10_parse::<u32>() {
                Some(base10_id)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

pub fn generate_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemStruct);
    let result = if let Some(id) = get_id(args) {
        let struct_name = &input.ident;

        if let syn::Fields::Named(fields) = input.fields {
            let named_fields = &fields.named;
            let base_impl = generate_base_impl(struct_name, &fields, id);
            let debug_impl = generate_debug_impl(struct_name, &fields);
            let bits_count = get_dirty_bits_count(&fields);
            quote! {
                pub struct #struct_name {
                    dirty_bits: [u32; #bits_count],
                    #named_fields
                }

                #base_impl

                #debug_impl

            }
        } else {
            panic!("Component should have named fields")
        }
    } else {
        panic!("Unable to find ID")
    };
    result.into()
}
