use std::convert::TryFrom;

use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, token::Comma, Field, Ident};

use crate::utils::get_ident_type_fields;

use super::{SchemaSerialized, SpatialType};

pub fn get_freeers(fields: &Punctuated<Field, Comma>, data: &Ident) -> Vec<TokenStream> {
    get_ident_type_fields(fields)
        .into_iter()
        .filter_map(|(ident, ty)| SpatialType::try_from(ty).ok().map(|ty| (ident, ty)))
        .filter_map(|(ident, ty)| ty.generate_update_freeer(data, ident))
        .collect()
}
