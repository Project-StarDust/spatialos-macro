pub mod r#enum;
pub mod r#struct;
pub mod field;
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

pub use r#struct::StructAST;
pub use r#enum::EnumAST;
pub use r#type::SpatialType;
use syn::{Attribute, TypePath, parse::Parse};