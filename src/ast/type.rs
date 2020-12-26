use proc_macro2::TokenStream as TokenStream2;
use syn::{Field, Ident, Type};

use super::{append_to_end_segment, get_spatial_type};

#[derive(Debug)]
pub enum SpatialType {
    Bool,
    Uint32,
    Uint64,
    Int32,
    Int64,
    SInt32,
    SInt64,
    Fixed32,
    Fixed64,
    SFixed32,
    SFixed64,
    Float,
    Double,
    String,
    Bytes,
    EntityID,
    Entity,
    Map(Box<SpatialType>, Box<SpatialType>),
    List(Box<SpatialType>),
    Option(Box<SpatialType>),
    Type(Type),
    Enum(Type),
}

impl From<&Field> for SpatialType {
    fn from(field: &Field) -> Self {
        let ty = field.ty.clone();
        let data_type = get_spatial_type(&field.attrs).expect("Can't find the spatial_type");
        match data_type.as_str() {
            "bool" => Self::Bool,
            "float" => Self::Float,
            "bytes" => Self::Bytes,
            "int32" => Self::Int32,
            "int64" => Self::Int64,
            "string" => Self::String,
            "double" => Self::Double,
            "uint32" => Self::Uint32,
            "uint64" => Self::Uint64,
            "sint32" => Self::SInt32,
            "sint64" => Self::SInt64,
            "fixed32" => Self::Fixed32,
            "fixed64" => Self::Fixed64,
            "sfixed32" => Self::SFixed32,
            "sfixed64" => Self::SFixed64,
            "EntityId" => Self::EntityID,
            "Entity" => Self::Entity,
            "type" => Self::Type(ty),
            "enum" => Self::Enum(ty),
            _ => panic!("Not supported yet: {}", data_type.as_str()),
        }
    }
}

impl SpatialType {
    pub fn get_data_type(&self) -> Type {
        match self {
            Self::Bool => syn::parse_str::<Type>("bool").unwrap(),
            Self::Uint32 => syn::parse_str::<Type>("u32").unwrap(),
            Self::Uint64 => syn::parse_str::<Type>("u64").unwrap(),
            Self::Int32 => syn::parse_str::<Type>("i32").unwrap(),
            Self::Int64 => syn::parse_str::<Type>("i64").unwrap(),
            Self::SInt32 => syn::parse_str::<Type>("i32").unwrap(),
            Self::SInt64 => syn::parse_str::<Type>("i64").unwrap(),
            Self::Fixed32 => syn::parse_str::<Type>("u32").unwrap(),
            Self::Fixed64 => syn::parse_str::<Type>("u64").unwrap(),
            Self::SFixed32 => syn::parse_str::<Type>("i32").unwrap(),
            Self::SFixed64 => syn::parse_str::<Type>("i64").unwrap(),
            Self::Float => syn::parse_str::<Type>("f32").unwrap(),
            Self::Double => syn::parse_str::<Type>("f64").unwrap(),
            Self::String => syn::parse_str::<Type>("String").unwrap(),
            Self::Bytes => syn::parse_str::<Type>("Vec<u8>").unwrap(),
            Self::Type(ty) => match ty.clone() {
                Type::Path(path) => Type::Path(append_to_end_segment(path, "Data")),
                _ => ty.clone(),
            },
            Self::Enum(ty) => ty.clone(),
            _ => panic!("Can't get data type for {:?}", self),
        }
    }

    pub fn get_update_type(&self) -> Type {
        match self {
            Self::Bool => syn::parse_str::<Type>("bool").unwrap(),
            Self::Uint32 => syn::parse_str::<Type>("u32").unwrap(),
            Self::Uint64 => syn::parse_str::<Type>("u64").unwrap(),
            Self::Int32 => syn::parse_str::<Type>("i32").unwrap(),
            Self::Int64 => syn::parse_str::<Type>("i64").unwrap(),
            Self::SInt32 => syn::parse_str::<Type>("i32").unwrap(),
            Self::SInt64 => syn::parse_str::<Type>("i64").unwrap(),
            Self::Fixed32 => syn::parse_str::<Type>("u32").unwrap(),
            Self::Fixed64 => syn::parse_str::<Type>("u64").unwrap(),
            Self::SFixed32 => syn::parse_str::<Type>("i32").unwrap(),
            Self::SFixed64 => syn::parse_str::<Type>("i64").unwrap(),
            Self::Float => syn::parse_str::<Type>("f32").unwrap(),
            Self::Double => syn::parse_str::<Type>("f64").unwrap(),
            Self::String => syn::parse_str::<Type>("String").unwrap(),
            Self::Bytes => syn::parse_str::<Type>("Vec<u8>").unwrap(),
            Self::Type(ty) => {
                let ty = match ty.clone() {
                    Type::Path(path) => Type::Path(append_to_end_segment(path, "Update")),
                    _ => ty.clone(),
                };
                syn::parse2::<Type>(quote! { Option<#ty> }).unwrap()
            }
            Self::Enum(ty) => ty.clone(),
            _ => panic!("Can't get data type for {:?}", self),
        }
    }

    pub fn get_data_deserializer(&self, object_name: &Ident, id: u32) -> TokenStream2 {
        match self {
            Self::Bool => quote! { #object_name.get_bool(#id) },
            Self::Double => quote! { #object_name.get_double(#id) },
            Self::Float => quote! { #object_name.get_float(#id) },
            Self::Uint32 => quote! { #object_name.get_uint32(#id) },
            Self::Uint64 => quote! { #object_name.get_uint64(#id) },
            Self::Int32 => quote! { #object_name.get_int32(#id) },
            Self::Int64 => quote! { #object_name.get_int64(#id) },
            Self::String => quote! { #object_name.get_string(#id) },
            Self::Bytes => quote! { #object_name.get_bytes(#id) },
            Self::Enum(ty) => quote! { #object_name.get_enum::<#ty>(#id) },
            Self::Type(ty) => {
                quote! { <#ty as spatialos_sdk::Type>::type_data_deserialize(user_data, &mut #object_name.get_object(#id)) }
            }
            _ => panic!("Can't get data_deserializer for {:?}", self),
        }
    }

    pub fn get_data_serializer(
        &self,
        data: &TokenStream2,
        target: &Ident,
        id: u32,
    ) -> TokenStream2 {
        match self {
            Self::Bool => quote! { #target.add_bool(#id, #data) },
            Self::Double => quote! { #target.add_double(#id, #data) },
            Self::Float => quote! { #target.add_float(#id, #data) },
            Self::Uint32 => quote! { #target.add_uint32(#id, #data) },
            Self::Uint64 => quote! { #target.add_uint64(#id, #data) },
            Self::Int32 => quote! { #target.add_int32(#id, #data) },
            Self::Int64 => quote! { #target.add_int64(#id, #data) },
            Self::String => quote! { #target.add_string(#id, &#data) },
            Self::Bytes => quote! { #target.add_bytes(#id, &#data) },
            Self::Enum(_) => quote! { #target.add_enum(#id, &#data) },
            Self::Type(ty) => {
                quote! { <#ty as spatialos_sdk::Type>::type_data_serialize(user_data, &mut #data, &mut #target.add_object(#id))  }
            }
            _ => panic!("Can't get data_serializer for {:?}", self),
        }
    }

    pub fn get_update_deserializer(&self, object_name: &Ident, id: u32) -> TokenStream2 {
        match self {
            Self::Bool => quote! { #object_name.get_bool(#id) },
            Self::Double => quote! { #object_name.get_double(#id) },
            Self::Float => quote! { #object_name.get_float(#id) },
            Self::Uint32 => quote! { #object_name.get_uint32(#id) },
            Self::Uint64 => quote! { #object_name.get_uint64(#id) },
            Self::Int32 => quote! { #object_name.get_int32(#id) },
            Self::Int64 => quote! { #object_name.get_int64(#id) },
            Self::String => quote! { #object_name.get_string(#id) },
            Self::Bytes => quote! { #object_name.get_bytes(#id) },
            Self::Enum(ty) => quote! { #object_name.get_enum::<#ty>(#id) },
            Self::Type(ty) => quote! {
                if #object_name.get_object_count(#id) == 1 {
                    Some(<#ty as spatialos_sdk::Type>::type_update_deserialize(
                        user_data,
                        &mut #object_name.get_object(#id),
                    ))
                } else {
                    None
                };
            },
            _ => panic!("Can't get update_deserializer for {:?}", self),
        }
    }

    pub fn get_update_serializer(
        &self,
        data: &TokenStream2,
        target: &Ident,
        id: u32,
    ) -> TokenStream2 {
        match self {
            Self::Bool => quote! { #target.add_bool(#id, #data) },
            Self::Double => quote! { #target.add_double(#id, #data) },
            Self::Float => quote! { #target.add_float(#id, #data) },
            Self::Uint32 => quote! { #target.add_uint32(#id, #data) },
            Self::Uint64 => quote! { #target.add_uint64(#id, #data) },
            Self::Int32 => quote! { #target.add_int32(#id, #data) },
            Self::Int64 => quote! { #target.add_int64(#id, #data) },
            Self::String => quote! { #target.add_string(#id, &#data) },
            Self::Bytes => quote! { #target.add_bytes(#id, &#data) },
            Self::Enum(_) => quote! { #target.add_enum(#id, &#data) },
            Self::Type(ty) => quote! {
                if let Some(mut ty) = #data.as_mut() {
                    <#ty as spatialos_sdk::Type>::type_update_serialize(user_data, &mut ty, &mut #target.add_object(#id));
                }
            },
            _ => panic!("Can't get update_serializer for {:?}", self),
        }
    }
}
