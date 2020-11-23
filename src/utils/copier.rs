use std::convert::TryFrom;

use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, token::Comma, Field, Ident};

use super::{get_ident_type_fields, SchemaSerialized, SpatialType};

pub fn get_copiers(
    fields: &Punctuated<Field, Comma>,
    new_data: &Ident,
    data: &Ident,
) -> Vec<TokenStream> {
    get_ident_type_fields(fields)
        .into_iter()
        .filter_map(|(ident, ty)| SpatialType::try_from(ty).ok().map(|ty| (ident, ty)))
        .filter_map(|(ident, ty)| ty.generate_update_copier(new_data, data, ident))
        .collect()
}
