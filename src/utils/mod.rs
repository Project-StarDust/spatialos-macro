use std::convert::TryFrom;

use proc_macro2::TokenStream;

use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Field, GenericArgument, Ident, Path,
    PathArguments, PathSegment, Type, TypePath,
};

mod serializer;
pub use serializer::*;

mod deserializer;
pub use deserializer::*;

mod copier;
pub use copier::*;

mod freeer;
pub use freeer::*;

mod types;
pub use types::*;

mod traits;
pub use traits::*;

pub fn get_composite_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(path) = ty {
        if path.path.segments.len() == 1 {
            let segment = &path.path.segments[0];
            if !segment.arguments.is_empty() {
                match segment.ident.to_string().as_str() {
                    "Vec" => Some(ty),
                    "HashMap" => Some(ty),
                    "Option" => Some(ty),
                    _ => None,
                }
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

pub fn get_arguments(ty: &Type) -> Option<PathArguments> {
    if get_composite_type(ty).is_some() {
        match ty {
            Type::Path(path) => {
                let segment = &path.path.segments[0];
                Some(segment.arguments.clone())
            }
            _ => None,
        }
    } else {
        None
    }
}

pub fn get_one_argument_type(ty: &Type) -> Option<Type> {
    let arguments = get_arguments(ty)?;
    if let PathArguments::AngleBracketed(arguments) = arguments {
        let arg = &arguments.args[0];
        match arg {
            GenericArgument::Type(ty) => Some(ty.clone()),
            _ => None,
        }
    } else {
        None
    }
}

pub fn get_two_argument_type(ty: &Type) -> Option<(Type, Type)> {
    let arguments = get_arguments(ty)?;
    if let PathArguments::AngleBracketed(arguments) = arguments {
        let arg1 = &arguments.args[0];
        let arg2 = &arguments.args[1];
        match arg1 {
            GenericArgument::Type(ty1) => match arg2 {
                GenericArgument::Type(ty2) => Some((ty1.clone(), ty2.clone())),
                _ => None,
            },
            _ => None,
        }
    } else {
        None
    }
}

pub fn get_ident_type_fields(fields: &Punctuated<Field, Comma>) -> Vec<(&Ident, SpatialType)> {
    fields
        .iter()
        .filter_map(|field| {
            SpatialType::try_from(field)
                .ok()
                .map(|ty| (field.ident.as_ref(), ty))
        })
        .filter_map(|field| {
            let ident = field.0?;
            Some((ident, field.1))
        })
        .collect()
}

pub fn get_ident_type_id_fields(
    fields: &Punctuated<Field, Comma>,
) -> Vec<(&Ident, SpatialType, u32)> {
    fields
        .iter()
        .filter_map(|field| {
            SpatialType::try_from(field)
                .ok()
                .map(|ty| (field.ident.as_ref(), ty, get_field_id(&field.attrs)))
        })
        .filter_map(|field| {
            let ident = field.0?;
            Some((ident, field.1, field.2))
        })
        .filter_map(|field| {
            let field_id = field.2?;
            Some((field.0, field.1, field_id))
        })
        .collect()
}
