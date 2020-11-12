use syn::ItemEnum;
use proc_macro::TokenStream;
use syn::parse_macro_input;

pub fn generate_enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);

    let enum_name = &input.ident;
    let variants = &input.variants;

    let result = quote! {
        pub enum #enum_name {
            #variants
        }
    };
    result.into()
}
