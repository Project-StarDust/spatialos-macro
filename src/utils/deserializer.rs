use std::convert::TryFrom;

use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, token::Comma, Field, Ident};

use crate::utils::get_ident_type_id_fields;

use super::{SchemaSerialized, SpatialType};

pub fn get_data_deserializers(
    fields: &Punctuated<Field, Comma>,
    object_name: &Ident,
) -> Vec<TokenStream> {
    get_ident_type_id_fields(fields)
        .into_iter()
        .filter_map(|(ident, ty, id)| SpatialType::try_from(ty).ok().map(|ty| (ident, ty, id)))
        .filter_map(|(ident, ty, id)| {
            ty.generate_data_deserializer(id, object_name, None)
                .map(|tokens| (ident, tokens))
        })
        .map(|(ident, tokens)| quote! { let #ident = #tokens })
        .collect()
}

pub fn get_update_deserializers(
    fields: &Punctuated<Field, Comma>,
    object_name: &Ident,
) -> Vec<TokenStream> {
    get_ident_type_id_fields(fields)
        .into_iter()
        .filter_map(|(ident, ty, id)| SpatialType::try_from(ty).ok().map(|ty| (ident, ty, id)))
        .filter_map(|(ident, ty, id)| {
            ty.generate_update_deserializer(id, object_name, None)
                .map(|tokens| (ident, tokens))
        })
        .map(|(ident, tokens)| quote! { let #ident = #tokens })
        .collect()
}
