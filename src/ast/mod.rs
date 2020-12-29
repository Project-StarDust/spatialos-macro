pub mod r#enum;
pub mod field;
pub mod r#struct;
pub mod r#type;

fn get_field_id(attrs: &[Attribute]) -> Option<u32> {
    let attribute = extract_attribute::<syn::Lit>(attrs, "field_id")?;

    if let syn::Lit::Int(lit_int) = attribute {
        lit_int.base10_parse::<u32>().ok()
    } else {
        None
    }
}

fn get_value(attrs: &[Attribute]) -> Option<u32> {
    let attribute = extract_attribute::<syn::Lit>(attrs, "value")?;

    if let syn::Lit::Int(lit_int) = attribute {
        lit_int.base10_parse::<u32>().ok()
    } else {
        None
    }
}

fn get_id(attrs: &[Attribute]) -> Option<u32> {
    let attribute = extract_attribute::<syn::Lit>(attrs, "id")?;

    if let syn::Lit::Int(lit_int) = attribute {
        lit_int.base10_parse::<u32>().ok()
    } else {
        None
    }
}

fn get_spatial_type(attrs: &[Attribute]) -> Option<String> {
    let attribute = extract_attribute::<syn::Lit>(attrs, "spatial_type")?;

    if let syn::Lit::Str(lit_str) = attribute {
        Some(lit_str.value())
    } else {
        None
    }
}

pub fn append_to_end_segment<S: AsRef<str>>(mut ty_path: TypePath, suffix: S) -> TypePath {
    if let Some(mut last) = ty_path.path.segments.last_mut() {
        last.ident = format_ident!("{}{}", last.ident, suffix.as_ref())
    };
    ty_path
}

fn extract_attribute<T: Parse>(attrs: &[Attribute], name: &str) -> Option<T> {
    attrs
        .iter()
        .find(|attr| attr.path.is_ident(name))
        .map(|attr| attr.parse_args())?
        .ok()
}

pub fn unpack_one_arg(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Path(path) => {
            let last = path.path.segments.last()?;
            match &last.arguments {
                PathArguments::AngleBracketed(args) => {
                    args.args.iter().next().and_then(|opt| match opt {
                        GenericArgument::Type(ty) => Some(ty),
                        _ => None,
                    })
                }
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn unpack_two_arg(ty: &Type) -> Option<(&Type, &Type)> {
    match ty {
        Type::Path(path) => {
            let last = path.path.segments.last()?;
            match &last.arguments {
                PathArguments::AngleBracketed(args) => {
                    let mut iter = args.args.iter();
                    let arg1 = iter.next().and_then(|opt| match opt {
                        GenericArgument::Type(ty) => Some(ty),
                        _ => None,
                    })?;
                    let arg2 = iter.next().and_then(|opt| match opt {
                        GenericArgument::Type(ty) => Some(ty),
                        _ => None,
                    })?;
                    Some((arg1, arg2))
                }
                _ => None,
            }
        }
        _ => None,
    } 
}

pub use r#enum::EnumAST;
pub use r#struct::StructAST;
pub use r#type::SpatialType;
use syn::{Attribute, GenericArgument, PathArguments, Type, TypePath, parse::Parse};
