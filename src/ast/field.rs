use proc_macro2::TokenStream as TokenStream2;
use syn::{Field, Ident};

use super::{get_field_id, SpatialType};

#[derive(Debug)]
pub struct FieldAST {
    pub id: u32,
    pub name: Ident,
    pub ty: SpatialType,
}

impl FieldAST {
    pub fn get_update_field(&self) -> TokenStream2 {
        let name = &self.name;
        let utype = self.ty.get_update_type();
        quote! {
            pub #name: #utype
        }
    }

    pub fn get_data_field(&self) -> TokenStream2 {
        let name = &self.name;
        let dtype = self.ty.get_data_type();
        quote! {
            pub #name: #dtype
        }
    }
    pub fn get_data_deserializer(&self, object_name: &Ident) -> TokenStream2 {
        let id = self.id;
        let name = &self.name;
        let deserializer = self.ty.get_data_deserializer(object_name, id);
        quote! {
            let #name = #deserializer;
        }
    }
    pub fn get_data_serializer(&self, data: &Ident, target: &Ident) -> TokenStream2 {
        let id = self.id;
        let name = &self.name;
        let data = quote! { #data.#name };
        let serializer = self.ty.get_data_serializer(&data, target, id);
        quote! {
            #serializer;
        }
    }
    pub fn get_update_deserializer(&self, object_name: &Ident) -> TokenStream2 {
        let id = self.id;
        let name = &self.name;
        let deserializer = self.ty.get_update_deserializer(object_name, id);
        quote! {
            let #name = #deserializer;
        }
    }
    pub fn get_update_serializer(&self, data: &Ident, target: &Ident) -> TokenStream2 {
        let id = self.id;
        let name = &self.name;
        let data = quote! { #data.#name };
        let serializer = self.ty.get_update_serializer(&data, target, id);
        quote! {
            #serializer;
        }
    }
}

impl From<&Field> for FieldAST {
    fn from(field: &Field) -> Self {
        let id = get_field_id(&field.attrs).expect("Can't find the field_id");
        let name = field
            .ident
            .as_ref()
            .expect("Can't find field ident")
            .clone();
        let ty = SpatialType::from(field);
        Self { id, name, ty }
    }
}
