use std::convert::TryFrom;

use syn::Type;

use super::SchemaSerialized;

mod composite;
mod custom;
mod primitive;

#[derive(Debug)]
pub enum Primitive {
    Bool,
    F64,
    F32,
    U32,
    U64,
    I32,
    I64,
    String,
}

#[derive(Debug)]
pub enum SpatialType {
    PlainType(Box<PlainType>),
    CompositeType(Box<CompositeType>),
}

impl TryFrom<&Type> for SpatialType {
    type Error = ();

    fn try_from(value: &Type) -> Result<Self, Self::Error> {
        CompositeType::try_from(value)
            .map(Box::new)
            .map(Self::CompositeType)
            .or_else(|_| {
                PlainType::try_from(value)
                    .map(Box::new)
                    .map(Self::PlainType)
            })
    }
}

impl SchemaSerialized for SpatialType {
    fn generate_data_serializer(
        self,
        id: u32,
        ident: &proc_macro2::Ident,
        object_name: &proc_macro2::Ident,
        data_name: Option<&proc_macro2::Ident>,
        is_ref: bool,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::PlainType(ty) => {
                ty.generate_data_serializer(id, ident, object_name, data_name, is_ref)
            }
            Self::CompositeType(ty) => {
                ty.generate_data_serializer(id, ident, object_name, data_name, is_ref)
            }
        }
    }

    fn generate_data_deserializer(
        self,
        id: u32,
        object_name: &proc_macro2::Ident,
        index: Option<&proc_macro2::Ident>,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::PlainType(ty) => ty.generate_data_deserializer(id, object_name, index),
            Self::CompositeType(ty) => ty.generate_data_deserializer(id, object_name, index),
        }
    }

    fn generate_update_serializer(
        self,
        id: u32,
        ident: &proc_macro2::Ident,
        object_name: &proc_macro2::Ident,
        data_name: Option<&proc_macro2::Ident>,
        is_ref: bool,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::PlainType(ty) => {
                ty.generate_update_serializer(id, ident, object_name, data_name, is_ref)
            }
            Self::CompositeType(ty) => {
                ty.generate_update_serializer(id, ident, object_name, data_name, is_ref)
            }
        }
    }

    fn generate_update_deserializer(
        self,
        id: u32,
        object_name: &proc_macro2::Ident,
        index: Option<&proc_macro2::Ident>,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::PlainType(ty) => ty.generate_update_deserializer(id, object_name, index),
            Self::CompositeType(ty) => ty.generate_update_deserializer(id, object_name, index),
        }
    }

    fn generate_update_copier(
        self,
        new_data: &proc_macro2::Ident,
        data: &proc_macro2::Ident,
        ident: &proc_macro2::Ident,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::PlainType(ty) => ty.generate_update_copier(new_data, data, ident),
            Self::CompositeType(ty) => ty.generate_update_copier(new_data, data, ident),
        }
    }

    fn generate_update_freeer(
        self,
        data: &proc_macro2::Ident,
        ident: &proc_macro2::Ident,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::PlainType(ty) => ty.generate_update_freeer(data, ident),
            Self::CompositeType(ty) => ty.generate_update_freeer(data, ident),
        }
    }

    fn get_data_type(self) -> Type {
        match self {
            Self::PlainType(ty) => ty.get_data_type(),
            Self::CompositeType(ty) => ty.get_data_type(),
        }
    }

    fn get_update_type(self) -> Type {
        match self {
            Self::PlainType(ty) => ty.get_update_type(),
            Self::CompositeType(ty) => ty.get_update_type(),
        }
    }
}

#[derive(Debug)]
pub enum PlainType {
    Primitive(Box<Primitive>),
    SpatialType(Box<Type>),
}

impl TryFrom<&Type> for PlainType {
    type Error = ();

    fn try_from(value: &Type) -> Result<Self, Self::Error> {
        Primitive::try_from(value)
            .map(Box::new)
            .map(Self::Primitive)
            .or_else(|_| Ok(Self::SpatialType(Box::new(value.clone()))))
    }
}

impl SchemaSerialized for PlainType {
    fn generate_data_serializer(
        self,
        id: u32,
        ident: &proc_macro2::Ident,
        object_name: &proc_macro2::Ident,
        data_name: Option<&proc_macro2::Ident>,
        is_ref: bool,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::Primitive(ty) => {
                ty.generate_data_serializer(id, ident, object_name, data_name, is_ref)
            }
            Self::SpatialType(ty) => {
                ty.generate_data_serializer(id, ident, object_name, data_name, is_ref)
            }
        }
    }

    fn generate_data_deserializer(
        self,
        id: u32,
        object_name: &proc_macro2::Ident,
        index: Option<&proc_macro2::Ident>,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::Primitive(ty) => ty.generate_data_deserializer(id, object_name, index),
            Self::SpatialType(ty) => ty.generate_data_deserializer(id, object_name, index),
        }
    }

    fn generate_update_serializer(
        self,
        id: u32,
        ident: &proc_macro2::Ident,
        object_name: &proc_macro2::Ident,
        data_name: Option<&proc_macro2::Ident>,
        is_ref: bool,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::Primitive(ty) => {
                ty.generate_update_serializer(id, ident, object_name, data_name, is_ref)
            }
            Self::SpatialType(ty) => {
                ty.generate_update_serializer(id, ident, object_name, data_name, is_ref)
            }
        }
    }

    fn generate_update_deserializer(
        self,
        id: u32,
        object_name: &proc_macro2::Ident,
        index: Option<&proc_macro2::Ident>,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::Primitive(ty) => ty.generate_update_deserializer(id, object_name, index),
            Self::SpatialType(ty) => ty.generate_update_deserializer(id, object_name, index),
        }
    }

    fn generate_update_copier(
        self,
        new_data: &proc_macro2::Ident,
        data: &proc_macro2::Ident,
        ident: &proc_macro2::Ident,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::Primitive(ty) => ty.generate_update_copier(new_data, data, ident),
            Self::SpatialType(ty) => ty.generate_update_copier(new_data, data, ident),
        }
    }

    fn generate_update_freeer(
        self,
        data: &proc_macro2::Ident,
        ident: &proc_macro2::Ident,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::Primitive(ty) => ty.generate_update_freeer(data, ident),
            Self::SpatialType(ty) => ty.generate_update_freeer(data, ident),
        }
    }

    fn get_data_type(self) -> Type {
        match self {
            Self::Primitive(ty) => ty.get_data_type(),
            Self::SpatialType(ty) => ty.get_data_type(),
        }
    }

    fn get_update_type(self) -> Type {
        match self {
            Self::Primitive(ty) => ty.get_update_type(),
            Self::SpatialType(ty) => ty.get_update_type(),
        }
    }
}

#[derive(Debug)]
pub enum CompositeType {
    Vec(PlainType),
    HashMap(PlainType, PlainType),
    Option(PlainType),
}
