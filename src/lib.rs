extern crate proc_macro;

mod spatial_component;
mod spatial_enum;
mod spatial_type;
mod utils;

use crate::spatial_enum::generate_enum;
use crate::spatial_component::generate_component;
use crate::spatial_type::generate_type;
use proc_macro::TokenStream;

#[macro_use]
extern crate quote;

#[proc_macro_attribute]
pub fn spatial_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_component(attr, item)
}

#[proc_macro_attribute]
pub fn spatial_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_type(attr, item)
}

#[proc_macro_attribute]
pub fn spatial_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_enum(attr, item)
}
