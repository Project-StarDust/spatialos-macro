use proc_macro::TokenStream;
use syn::{Ident, ItemEnum, Variant};

use super::get_value;

#[derive(Debug)]
pub struct VariantAST {
    pub value: u32,
    pub name: Ident,
}

impl From<&Variant> for VariantAST {
    fn from(input: &Variant) -> Self {
        let name = input.ident.clone();
        let value = get_value(&input.attrs).expect("Can't find value for variant");
        Self { name, value }
    }
}

#[derive(Debug)]
pub struct EnumAST<'a> {
    pub name: &'a Ident,
    pub variants: Vec<VariantAST>,
}

impl<'a> From<&'a ItemEnum> for EnumAST<'a> {
    fn from(input: &'a ItemEnum) -> Self {
        let name = &input.ident;
        let variants = input
            .variants
            .iter()
            .map(VariantAST::from)
            .collect::<Vec<_>>();
        Self { name, variants }
    }
}

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

        let as_ref = {
            quote! {
                impl AsRef<#name> for #name {
                    fn as_ref(&self) -> &#name {
                        self
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

            #[automatically_derived]
            #[allow(unused_qualifications)]
            #as_ref
        };
        result.into()
    }
}
