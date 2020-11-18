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

pub fn get_field_id(attrs: &[Attribute]) -> Option<u32> {
    let attribute: syn::Lit = attrs
        .iter()
        .find(|attr| attr.path.is_ident("field_id"))
        .map(|attr| attr.parse_args())?
        .ok()?;

    if let syn::Lit::Int(lit_int) = attribute {
        lit_int.base10_parse::<u32>().ok()
    } else {
        None
    }
}

pub fn get_id(attrs: &[Attribute]) -> Option<u32> {
    let attribute: syn::Lit = attrs
        .iter()
        .find(|attr| attr.path.is_ident("id"))
        .map(|attr| attr.parse_args())?
        .ok()?;

    if let syn::Lit::Int(lit_int) = attribute {
        lit_int.base10_parse::<u32>().ok()
    } else {
        None
    }
}

pub fn get_composite_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(path) = ty {
        if path.path.segments.len() == 1 {
            let segment = &path.path.segments[0];
            if !segment.arguments.is_empty() {
                match segment.ident.to_string().as_str() {
                    "Vec" => Some(ty),
                    "HashMap" => Some(ty),
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

pub fn get_ident_type_fields(fields: &Punctuated<Field, Comma>) -> Vec<(&Ident, &Type)> {
    fields
        .iter()
        .map(|field| (field.ident.as_ref(), &field.ty))
        .filter_map(|field| {
            let ident = field.0?;
            Some((ident, field.1))
        })
        .collect()
}

pub fn get_ident_type_id_fields(fields: &Punctuated<Field, Comma>) -> Vec<(&Ident, &Type, u32)> {
    fields
        .iter()
        .map(|field| (field.ident.as_ref(), &field.ty, get_field_id(&field.attrs)))
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

pub fn get_constructor<S: AsRef<str>>(fields: &Punctuated<Field, Comma>, name: S) -> TokenStream {
    let ty = Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: name
                .as_ref()
                .split("::")
                .map(|seg| PathSegment {
                    ident: format_ident!("{}", seg),
                    arguments: syn::PathArguments::None,
                })
                .fold(Punctuated::new(), |mut acc, val| {
                    acc.push(val);
                    acc
                }),
        },
    });
    let idents = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<_>>();
    quote! {
        #ty { #(#idents,)* }
    }
}

pub fn append_to_end_segment<S: AsRef<str>>(mut ty_path: TypePath, suffix: S) -> TypePath {
    if let Some(mut last) = ty_path.path.segments.last_mut() {
        last.ident = format_ident!("{}{}", last.ident, suffix.as_ref())
    };
    ty_path
}
