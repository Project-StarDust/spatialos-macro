use std::convert::TryFrom;

use syn::Type;

use super::SchemaSerialized;

mod composite;
mod custom;
mod primitive;

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

pub enum SpatialType {
    PlainType(PlainType),
    CompositeType(CompositeType),
}

impl TryFrom<&Type> for SpatialType {
    type Error = ();

    fn try_from(value: &Type) -> Result<Self, Self::Error> {
        CompositeType::try_from(value)
            .map(Self::CompositeType)
            .or(PlainType::try_from(value).map(Self::PlainType))
    }
}

impl SchemaSerialized for SpatialType {
    fn generate_data_serializer(
        self,
        id: u32,
        ident: &proc_macro2::Ident,
        object_name: &proc_macro2::Ident,
        data_name: &proc_macro2::Ident,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::PlainType(ty) => ty.generate_data_serializer(id, ident, object_name, data_name),
            Self::CompositeType(ty) => {
                ty.generate_data_serializer(id, ident, object_name, data_name)
            }
        }
    }

    fn generate_data_deserializer(
        self,
        id: u32,
        ident: &proc_macro2::Ident,
        object_name: &proc_macro2::Ident,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::PlainType(ty) => ty.generate_data_deserializer(id, ident, object_name),
            Self::CompositeType(ty) => ty.generate_data_deserializer(id, ident, object_name),
        }
    }

    fn generate_update_serializer(
        self,
        id: u32,
        ident: &proc_macro2::Ident,
        object_name: &proc_macro2::Ident,
        data_name: &proc_macro2::Ident,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::PlainType(ty) => ty.generate_update_serializer(id, ident, object_name, data_name),
            Self::CompositeType(ty) => {
                ty.generate_update_serializer(id, ident, object_name, data_name)
            }
        }
    }

    fn generate_update_deserializer(
        self,
        id: u32,
        ident: &proc_macro2::Ident,
        object_name: &proc_macro2::Ident,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::PlainType(ty) => ty.generate_update_deserializer(id, ident, object_name),
            Self::CompositeType(ty) => ty.generate_update_deserializer(id, ident, object_name),
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

pub enum PlainType {
    Primitive(Primitive),
    SpatialType(Type),
}

impl TryFrom<&Type> for PlainType {
    type Error = ();

    fn try_from(value: &Type) -> Result<Self, Self::Error> {
        Primitive::try_from(value)
            .map(Self::Primitive)
            .or(Ok(Self::SpatialType(value.clone())))
    }
}

impl SchemaSerialized for PlainType {
    fn generate_data_serializer(
        self,
        id: u32,
        ident: &proc_macro2::Ident,
        object_name: &proc_macro2::Ident,
        data_name: &proc_macro2::Ident,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::Primitive(ty) => ty.generate_data_serializer(id, ident, object_name, data_name),
            Self::SpatialType(ty) => ty.generate_data_serializer(id, ident, object_name, data_name),
        }
    }

    fn generate_data_deserializer(
        self,
        id: u32,
        ident: &proc_macro2::Ident,
        object_name: &proc_macro2::Ident,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::Primitive(ty) => ty.generate_data_deserializer(id, ident, object_name),
            Self::SpatialType(ty) => ty.generate_data_deserializer(id, ident, object_name),
        }
    }

    fn generate_update_serializer(
        self,
        id: u32,
        ident: &proc_macro2::Ident,
        object_name: &proc_macro2::Ident,
        data_name: &proc_macro2::Ident,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::Primitive(ty) => ty.generate_update_serializer(id, ident, object_name, data_name),
            Self::SpatialType(ty) => {
                ty.generate_update_serializer(id, ident, object_name, data_name)
            }
        }
    }

    fn generate_update_deserializer(
        self,
        id: u32,
        ident: &proc_macro2::Ident,
        object_name: &proc_macro2::Ident,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::Primitive(ty) => ty.generate_update_deserializer(id, ident, object_name),
            Self::SpatialType(ty) => ty.generate_update_deserializer(id, ident, object_name),
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

#[allow(dead_code)]
pub enum CompositeType {
    Vec(PlainType),
    HashMap(PlainType, PlainType),
    Option(PlainType),
}
