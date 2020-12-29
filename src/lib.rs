extern crate proc_macro;

#[macro_use]
extern crate lazy_static;

mod ast;
mod spatial_component;
mod spatial_enum;
mod spatial_type;
//mod utils;

use crate::spatial_component::generate_component;
use crate::spatial_enum::generate_enum;
use crate::spatial_type::generate_type;
use proc_macro::TokenStream;

#[macro_use]
extern crate quote;

#[proc_macro_derive(SpatialComponent, attributes(id, field_id, spatial_type))]
pub fn spatial_component(item: TokenStream) -> TokenStream {
    generate_component(item)
}

#[proc_macro_derive(SpatialType, attributes(field_id, spatial_type))]
pub fn spatial_type(item: TokenStream) -> TokenStream {
    generate_type(item)
}

#[proc_macro_derive(SpatialEnum, attributes(value))]
pub fn spatial_enum(item: TokenStream) -> TokenStream {
    generate_enum(item)
}
