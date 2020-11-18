use std::convert::TryFrom;

use proc_macro2::TokenStream;
use syn::{Ident, Type};

use crate::utils::SchemaSerialized;

use super::Primitive;

impl TryFrom<&Type> for Primitive {
    type Error = ();

    fn try_from(value: &Type) -> Result<Self, Self::Error> {
        if let Type::Path(ident) = value {
            let ident = ident.path.get_ident().ok_or(())?;
            match ident.to_string().as_str() {
                "bool" => Ok(Primitive::Bool),
                "f64" => Ok(Primitive::F64),
                "f32" => Ok(Primitive::F32),
                "u32" => Ok(Primitive::U32),
                "u64" => Ok(Primitive::U64),
                "i32" => Ok(Primitive::I32),
                "i64" => Ok(Primitive::I64),
                "String" => Ok(Primitive::String),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

impl SchemaSerialized for Primitive {
    fn generate_data_serializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
        data_name: &Ident,
    ) -> Option<TokenStream> {
        match self {
            Primitive::Bool => Some(quote! { #object_name.add_bool(#id, #data_name.#ident) }),
            Primitive::F64 => Some(quote! { #object_name.add_double(#id, #data_name.#ident) }),
            Primitive::F32 => Some(quote! { #object_name.add_float(#id, #data_name.#ident) }),
            Primitive::U32 => Some(quote! { #object_name.add_uint32(#id, #data_name.#ident) }),
            Primitive::U64 => Some(quote! { #object_name.add_uint64(#id, #data_name.#ident) }),
            Primitive::I32 => Some(quote! { #object_name.add_int32(#id, #data_name.#ident) }),
            Primitive::I64 => Some(quote! { #object_name.add_int64(#id, #data_name.#ident) }),
            Primitive::String => Some(quote! { #object_name.add_string(#id, &#data_name.#ident) }),
        }
    }

    fn generate_data_deserializer(
        self,
        id: u32,
        _: &Ident,
        object_name: &Ident,
    ) -> Option<TokenStream> {
        match self {
            Primitive::Bool => Some(quote! { #object_name.get_bool(#id) }),
            Primitive::F64 => Some(quote! { #object_name.get_double(#id) }),
            Primitive::F32 => Some(quote! { #object_name.get_float(#id) }),
            Primitive::U32 => Some(quote! { #object_name.get_uint32(#id) }),
            Primitive::U64 => Some(quote! { #object_name.get_uint64(#id) }),
            Primitive::I32 => Some(quote! { #object_name.get_int32(#id) }),
            Primitive::I64 => Some(quote! { #object_name.get_int64(#id) }),
            Primitive::String => Some(quote! { #object_name.get_string(#id) }),
        }
    }

    fn generate_update_serializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
        data_name: &Ident,
    ) -> Option<TokenStream> {
        self.generate_data_serializer(id, ident, object_name, data_name)
    }

    fn generate_update_deserializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
    ) -> Option<TokenStream> {
        self.generate_data_deserializer(id, ident, object_name)
    }

    fn generate_update_copier(self, _: &Ident, _: &Ident, _: &Ident) -> Option<TokenStream> {
        None
    }

    fn generate_update_freeer(self, _: &Ident, _: &Ident) -> Option<TokenStream> {
        None
    }

    fn get_data_type(self) -> Type {
        syn::parse_str::<Type>(self.get_type()).expect("Cannot create data type for Primitive")
    }

    fn get_update_type(self) -> Type {
        syn::parse_str::<Type>(self.get_type()).expect("Cannot create update type for Primitive")
    }
}

impl Primitive {
    pub fn get_name(&self) -> &str {
        match self {
            Primitive::Bool => "bool",
            Primitive::F64 => "double",
            Primitive::F32 => "float",
            Primitive::U32 => "uint32",
            Primitive::U64 => "uint64",
            Primitive::I32 => "int32",
            Primitive::I64 => "int64",
            Primitive::String => "string",
        }
    }

    pub fn get_type(&self) -> &str {
        match self {
            Primitive::Bool => "bool",
            Primitive::F64 => "f64",
            Primitive::F32 => "f32",
            Primitive::U32 => "u32",
            Primitive::U64 => "u64",
            Primitive::I32 => "i32",
            Primitive::I64 => "i64",
            Primitive::String => "String",
        }
    }
}
