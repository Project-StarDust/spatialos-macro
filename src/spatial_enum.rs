use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum};

use crate::ast::EnumAST;

impl<'a> Into<TokenStream> for EnumAST<'a> {
    fn into(self) -> TokenStream {
        let name = self.name;

        let from_u32 = {
            let variants = self
                .variants
                .iter()
                .map(|variant| {
                    let value = variant.value;
                    let ident = &variant.name;
                    quote! { #value => #name :: #ident}
                })
                .collect::<Vec<_>>();
            quote! {
                impl From<u32> for #name {
                    fn from(data: u32) -> Self {
                        match data {
                            #(#variants,)*
                            _ => panic!("Invalid data")
                        }
                    }
                }
            }
        };

        let into_u32 = {
            let variants = self
                .variants
                .iter()
                .map(|variant| {
                    let value = variant.value;
                    let ident = &variant.name;
                    quote! { #name :: #ident => #value}
                })
                .collect::<Vec<_>>();
            quote! {
                impl Into<u32> for #name {
                    fn into(self) -> u32 {
                        match self {
                            #(#variants,)*
                        }
                    }
                }

                impl Into<u32> for &#name {
                    fn into(self) -> u32 {
                        match *self {
                            #(#variants,)*
                        }
                    }
                }
            }
        };

        let result = quote! {
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #from_u32

            #[automatically_derived]
            #[allow(unused_qualifications)]
            #into_u32
        };
        result.into()
    }
}

pub fn generate_enum(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);

    let ast = EnumAST::from(&input);
    ast.into()
}
