use crate::utils::get_dirty_bits_count;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::ItemStruct;
use crate::utils::generate_debug_impl;

pub fn generate_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;

    let result = if let syn::Fields::Named(fields) = input.fields {
        let named_fields = &fields.named;
        let debug_impl = generate_debug_impl(struct_name, &fields);
        let bits_count = get_dirty_bits_count(&fields);
        quote! {
            pub struct #struct_name {
                dirty_bits: [u32; #bits_count],
                #named_fields
            }

            #debug_impl

        }
    } else {
        panic!("Unable to find ID")
    };
    result.into()
}
