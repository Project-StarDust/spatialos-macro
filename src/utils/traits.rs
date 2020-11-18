use proc_macro2::TokenStream;
use syn::{Ident, Type};

pub trait SchemaSerialized {
    fn generate_data_serializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
        data_name: &Ident,
    ) -> Option<TokenStream>;

    fn generate_data_deserializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
    ) -> Option<TokenStream>;

    fn generate_update_serializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
        data_name: &Ident,
    ) -> Option<TokenStream>;

    fn generate_update_deserializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
    ) -> Option<TokenStream>;

    fn generate_update_copier(
        self,
        new_data: &Ident,
        data: &Ident,
        ident: &Ident,
    ) -> Option<TokenStream>;

    fn generate_update_freeer(self, data: &Ident, ident: &Ident) -> Option<TokenStream>;

    fn get_data_type(self) -> Type;
    fn get_update_type(self) -> Type;
}
