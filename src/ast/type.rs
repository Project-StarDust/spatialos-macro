use proc_macro2::TokenStream as TokenStream2;
use regex::Regex;
use syn::{Field, Ident, Type};

const MAP_KEY_FIELD_ID: u32 = 1u32;
const MAP_VALUE_FIELD_ID: u32 = 2u32;

use super::{append_to_end_segment, get_spatial_type, unpack_one_arg, unpack_two_arg};

lazy_static! {
    static ref LST_RE: Regex = Regex::new(r"list<(.*)>").unwrap();
    static ref MAP_RE: Regex = Regex::new(r"map<(.*),(.*)>").unwrap();
    static ref OPT_RE: Regex = Regex::new(r"option<(.*)>").unwrap();
}

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

impl SpatialType {
    pub fn from_syn(ty: &Type, spatial_marker: &str) -> Self {
        match spatial_marker {
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
            "type" => Self::Type(ty.to_owned()),
            "enum" => Self::Enum(ty.to_owned()),
            _ => {
                if LST_RE.is_match(spatial_marker) {
                    let captures = LST_RE.captures(spatial_marker).unwrap();
                    let ty1 = unpack_one_arg(ty).expect("Can't find the one argument");
                    Self::List(Box::new(Self::from_syn(
                        ty1,
                        captures.get(1).map(|c| c.as_str()).unwrap(),
                    )))
                } else if OPT_RE.is_match(spatial_marker) {
                    let captures = OPT_RE.captures(spatial_marker).unwrap();
                    let ty1 = unpack_one_arg(ty).expect("Can't find the one argument");
                    Self::Option(Box::new(Self::from_syn(
                        ty1,
                        captures.get(1).map(|c| c.as_str()).unwrap(),
                    )))
                } else if MAP_RE.is_match(spatial_marker) {
                    let captures = MAP_RE.captures(spatial_marker).unwrap();
                    let (ty1, ty2) = unpack_two_arg(ty).expect("Can't find the one argument");
                    Self::Map(
                        Box::new(Self::from_syn(
                            ty1,
                            captures.get(1).map(|c| c.as_str()).unwrap(),
                        )),
                        Box::new(Self::from_syn(
                            ty2,
                            captures.get(2).map(|c| c.as_str()).unwrap(),
                        )),
                    )
                } else {
                    panic!("Not supported yet: {}", spatial_marker);
                }
            }
        }
    }
}

impl From<&Field> for SpatialType {
    fn from(field: &Field) -> Self {
        let ty = &field.ty;
        let data_type = get_spatial_type(&field.attrs).expect("Can't find the spatial_type");
        Self::from_syn(ty, data_type.as_str())
    }
}

impl SpatialType {
    pub fn get_spatial_name(&self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::Float => "float",
            Self::Bytes => "bytes",
            Self::Int32 => "int32",
            Self::Int64 => "int64",
            Self::String => "string",
            Self::Double => "double",
            Self::Uint32 => "uint32",
            Self::Uint64 => "uint64",
            Self::SInt32 => "sint32",
            Self::SInt64 => "sint64",
            Self::Fixed32 => "fixed32",
            Self::Fixed64 => "fixed64",
            Self::SFixed32 => "sfixed32",
            Self::SFixed64 => "sfixed64",
            Self::EntityID => "EntityId",
            Self::Entity => "Entity",
            Self::Type(_) => "object",
            Self::Enum(_) => "enum",
            _ => panic!("No spatial name for {:?}", self),
        }
    }

    pub fn get_rust_type(&self) -> Type {
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
            Self::Enum(ty) => ty.clone(),
            _ => panic!("Can't get rust type for {:?}", self),
        }
    }

    pub fn get_data_type(&self) -> Type {
        match self {
            Self::Type(ty) => match ty.clone() {
                Type::Path(_) => syn::parse2::<Type>(quote! { <#ty as spatialos_sdk::Type>::Data }).unwrap(),
                _ => ty.clone(),
            },
            Self::List(spatial_type) => {
                let ty = spatial_type.get_data_type();
                syn::parse2::<Type>(quote! { Vec<#ty> }).unwrap()
            }
            Self::Option(spatial_type) => {
                let ty = spatial_type.get_data_type();
                syn::parse2::<Type>(quote! { Option<#ty> }).unwrap()
            }
            Self::Map(spatial_type1, spatial_type2) => {
                let ty1 = spatial_type1.get_data_type();
                let ty2 = spatial_type2.get_data_type();
                syn::parse2::<Type>(quote! { HashMap<#ty1, #ty2> }).unwrap()
            }
            _ => self.get_rust_type(),
        }
    }

    pub fn get_optionless_update_type(&self) -> Type {
        match self {
            Self::Type(ty) => match ty.clone() {
                Type::Path(_) => syn::parse2::<Type>(quote! { <#ty as spatialos_sdk::Type>::Update }).unwrap(),
                _ => ty.clone(),
            },
            Self::List(spatial_type) => {
                let ty = spatial_type.get_optionless_update_type();
                syn::parse2::<Type>(quote! { Vec<#ty> }).unwrap()
            }
            Self::Option(spatial_type) => {
                let ty = spatial_type.get_optionless_update_type();
                syn::parse2::<Type>(quote! { Option<#ty> }).unwrap()
            }
            Self::Map(spatial_type1, spatial_type2) => {
                let ty1 = spatial_type1.get_optionless_update_type();
                let ty2 = spatial_type2.get_optionless_update_type();
                syn::parse2::<Type>(quote! { HashMap<#ty1, #ty2> }).unwrap()
            }
            _ => self.get_rust_type(),
        }
    }

    pub fn get_update_type(&self) -> Type {
        match self {
            Self::Type(_) | Self::List(_) | Self::Map(_, _) => {
                let ty = self.get_optionless_update_type();
                syn::parse2::<Type>(quote! { Option<#ty> }).unwrap()
            }
            _ => self.get_optionless_update_type(),
        }
    }

    pub fn get_indexed_data_deserializer(
        &self,
        object_name: &Ident,
        id: u32,
        index: &Ident,
    ) -> TokenStream2 {
        match self {
            Self::Type(ty) => quote! {
                <#ty as spatialos_sdk::Type>::type_data_deserialize(
                    user_data,
                    &mut #object_name.index_object(#id, #index),
                )
            },
            _ => panic!("Can't index {:?}", self),
        }
    }

    pub fn get_indexed_update_deserializer(
        &self,
        object_name: &Ident,
        id: u32,
        index: &Ident,
    ) -> TokenStream2 {
        match self {
            Self::Type(ty) => quote! {
                <#ty as spatialos_sdk::Type>::type_update_deserialize(
                    user_data,
                    &mut #object_name.index_object(#id, #index),
                )
            },
            _ => panic!("Can't index {:?}", self),
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
            Self::List(spatial_type) => match &**spatial_type {
                SpatialType::Type(_) => {
                    let index_ident = format_ident!("idx");
                    let deserializer =
                        spatial_type.get_indexed_data_deserializer(object_name, id, &index_ident);
                    quote! {
                        (0..#object_name.get_object_count(#id)).map(|#index_ident| {
                            #deserializer
                        }).collect()
                    }
                }
                SpatialType::Enum(ty) => quote! { #object_name.get_enum_list::<#ty>(#id) },
                _ => {
                    let name = spatial_type.get_spatial_name();
                    let func = format_ident!("get_{}_list", name);
                    quote! { #object_name.#func(#id) }
                }
            },
            Self::Option(spatial_type) => {
                let func = format_ident!("get_{}_count", spatial_type.get_spatial_name());
                let deserializer = spatial_type.get_data_deserializer(object_name, id);
                quote! {
                    if #object_name.#func(#id) > 0 {
                        Some(#deserializer)
                    } else {
                        None
                    }
                }
            }
            Self::Map(st1, st2) => {
                let object_ident = format_ident!("object");
                let deserializer1 = st1.get_data_deserializer(&object_ident, MAP_KEY_FIELD_ID);
                let deserializer2 = st2.get_data_deserializer(&object_ident, MAP_VALUE_FIELD_ID);
                quote! {
                    (0..#object_name.get_object_count(#id)).map(|i| {
                        let mut #object_ident = #object_name.index_object(#id, i);
                        let arg1 = #deserializer1;
                        let arg2 = #deserializer2;
                        (arg1, arg2)
                    }).collect()
                }
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
            Self::List(spatial_type) => match &**spatial_type {
                SpatialType::Type(_) => {
                    let value_ident = quote! { value };
                    let serializer = spatial_type.get_data_serializer(&value_ident, target, id);
                    quote! {
                        #data.iter_mut().for_each(|mut #value_ident| {
                            #serializer
                        })
                    }
                }
                SpatialType::Enum(ty) => quote! { #target.add_enum_list::<#ty>(#id, &#data) },
                _ => {
                    let name = spatial_type.get_spatial_name();
                    let func = format_ident!("add_{}_list", name);
                    quote! { #target.#func(#id, &#data) }
                }
            },
            Self::Option(spatial_type) => {
                let inner_ident = quote! { inner_ident };
                let serializer = spatial_type.get_data_serializer(&inner_ident, target, id);
                quote! {
                    if let Some(mut #inner_ident) = #data.as_mut() {
                        #serializer
                    }
                }
            }
            Self::Map(st1, st2) => {
                let object_ident = format_ident!("object");
                let key = quote! { key };
                let value = quote! { value };
                let serializer1 = st1.get_data_serializer(&key, &object_ident, MAP_KEY_FIELD_ID);
                let serializer2 =
                    st2.get_data_serializer(&value, &object_ident, MAP_VALUE_FIELD_ID);
                quote! {
                    #data.iter_mut().for_each(|(mut #key, mut #value)| {
                        let mut #object_ident = #target.add_object(#id);
                        #serializer1;
                        #serializer2;
                    })
                }
            }
            _ => panic!("Can't get data_serializer for {:?}", self),
        }
    }
    pub fn get_optionless_update_deserializer(&self, object_name: &Ident, id: u32) -> TokenStream2 {
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
                <#ty as spatialos_sdk::Type>::type_update_deserialize(
                    user_data,
                    &mut #object_name.get_object(#id),
                )
            },
            Self::List(spatial_type) => match &**spatial_type {
                SpatialType::Type(_) => {
                    let index_ident = format_ident!("idx");
                    let deserializer =
                        spatial_type.get_indexed_update_deserializer(object_name, id, &index_ident);
                    quote! {
                        (0..#object_name.get_object_count(#id)).map(|#index_ident| {
                            #deserializer
                        }).collect()
                    }
                }
                SpatialType::Enum(ty) => quote! { #object_name.get_enum_list::<#ty>(#id) },
                _ => {
                    let name = spatial_type.get_spatial_name();
                    let func = format_ident!("get_optional_{}_list", name);
                    quote! { #object_name.#func(#id) }
                }
            },
            Self::Option(spatial_type) => {
                let func = format_ident!("get_{}_count", spatial_type.get_spatial_name());
                let deserializer = spatial_type.get_optionless_update_deserializer(object_name, id);
                quote! {
                    if #object_name.#func(#id) > 0 {
                        Some(#deserializer)
                    } else {
                        None
                    }
                }
            }
            Self::Map(st1, st2) => {
                let object_ident = format_ident!("object");
                let deserializer1 = st1.get_optionless_update_deserializer(&object_ident, MAP_KEY_FIELD_ID);
                let deserializer2 = st2.get_optionless_update_deserializer(&object_ident, MAP_VALUE_FIELD_ID);
                quote! {
                    (0..#object_name.get_object_count(#id)).map(|i| {
                        let mut #object_ident = #object_name.index_object(#id, i);
                        let arg1 = #deserializer1;
                        let arg2 = #deserializer2;
                        (arg1, arg2)
                    }).collect()
                }
            }
            _ => panic!("Can't get update_deserializer for {:?}", self),
        }
    }

    pub fn get_update_deserializer(&self, object_name: &Ident, id: u32) -> TokenStream2 {
        match self {
            Self::List(bx) => match &**bx {
                Self::Type(_) => {
                    let deserializer = self.get_optionless_update_deserializer(object_name, id);
                    quote! {
                        if #object_name.get_object_count(#id) > 0 {
                            Some(#deserializer)
                        } else {
                            None
                        }
                    }
                }
                _ => self.get_optionless_update_deserializer(object_name, id),
            },
            Self::Type(_) | Self::Map(_, _)  => {
                let deserializer = self.get_optionless_update_deserializer(object_name, id);
                quote! {
                    if #object_name.get_object_count(#id) > 0 {
                        Some(#deserializer)
                    } else {
                        None
                    }
                }
            }
            _ => self.get_optionless_update_deserializer(object_name, id),
        }
    }

    pub fn get_optionless_update_serializer(
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
                quote! { <#ty as spatialos_sdk::Type>::type_update_serialize(user_data, &mut #data, &mut #target.add_object(#id)) }
            }
            Self::List(spatial_type) => match &**spatial_type {
                SpatialType::Type(_) => {
                    let value_ident = quote! { value };
                    let serializer =
                        spatial_type.get_optionless_update_serializer(&value_ident, target, id);
                    quote! {
                        #data.iter_mut().for_each(|mut #value_ident| {
                            #serializer
                        })
                    }
                }
                SpatialType::Enum(ty) => {
                    quote! { #target.add_enum_list::<#ty>(#id, &#data) }
                }
                _ => {
                    let name = spatial_type.get_spatial_name();
                    let func = format_ident!("add_{}_list", name);
                    quote! { #target.#func(#id, #data) }
                }
            },
            Self::Option(spatial_type) => {
                let inner_ident = quote! { inner };
                let serializer =
                    spatial_type.get_optionless_update_serializer(&inner_ident, target, id);
                quote! {
                    if let Some(mut #inner_ident) = #data.as_mut() {
                        #serializer
                    }
                }
            }
            Self::Map(st1, st2) => {
                let object_ident = format_ident!("object");
                let key = quote! { key };
                let value = quote! { value };
                let serializer1 =
                    st1.get_optionless_update_serializer(&key, &object_ident, MAP_KEY_FIELD_ID);
                let serializer2 =
                    st2.get_optionless_update_serializer(&value, &object_ident, MAP_VALUE_FIELD_ID);
                quote! {
                    #data.iter_mut().for_each(|(mut #key, mut #value)| {
                        let mut #object_ident = #target.add_object(#id);
                        #serializer1;
                        #serializer2;
                    })
                }
            }
            _ => panic!("Can't get update_serializer for {:?}", self),
        }
    }

    pub fn get_update_serializer(
        &self,
        data: &TokenStream2,
        target: &Ident,
        id: u32,
    ) -> TokenStream2 {
        match self {
            Self::Type(_) | Self::List(_) | Self::Map(_, _) => {
                let inner_ident = quote! { inner };
                let serializer = self.get_optionless_update_serializer(&inner_ident, target, id);
                quote! {
                    if let Some(mut #inner_ident) = #data.as_mut() {
                        #serializer
                    }
                }
            }
            _ => self.get_optionless_update_serializer(data, target, id),
        }
    }
}
