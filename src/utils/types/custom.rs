use proc_macro2::TokenStream;
use syn::{Ident, Type};

use crate::utils::{append_to_end_segment, SchemaSerialized};

impl SchemaSerialized for Type {
    fn generate_data_serializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
        data_name: &Ident,
    ) -> Option<TokenStream> {
        Some(quote! {
            <#self as spatialos_sdk::Type>::type_data_serialize(user_data, &mut #data_name.#ident, &mut #object_name.add_object(#id))
        })
    }

    fn generate_data_deserializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
    ) -> Option<TokenStream> {
        Some(quote! {
            let #ident = <#self as spatialos_sdk::Type>::type_data_deserialize(user_data, &mut #object_name.get_object(#id))
        })
    }

    fn generate_update_serializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
        data_name: &Ident,
    ) -> Option<TokenStream> {
        Some(quote! {
            if let Some(mut #ident) = #data_name.#ident.as_mut() {
                <#self as spatialos_sdk::Type>::type_update_serialize(user_data, &mut #ident, &mut #object_name.add_object(#id));
            }
        })
    }

    fn generate_update_deserializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
    ) -> Option<TokenStream> {
        Some(quote! {
            let #ident = if #object_name.get_object_count(#id) == 1 {
                Some(<#self as spatialos_sdk::Type>::type_update_deserialize(
                    user_data,
                    &mut #object_name.get_object(#id),
                ))
            } else {
                None
            };
        })
    }

    fn generate_update_copier(
        self,
        new_data: &Ident,
        data: &Ident,
        ident: &Ident,
    ) -> Option<TokenStream> {
        Some(quote! {
            #new_data.#ident = #data.#ident.as_ref().map(|#ident| <#self as spatialos_sdk::Type>::type_update_copy(user_data, #ident));
        })
    }

    fn generate_update_freeer(self, data: &Ident, ident: &Ident) -> Option<TokenStream> {
        Some(quote! {
            if let Some(#ident) = #data.#ident {
                <#self as spatialos_sdk::Type>::type_update_free(user_data, #ident);
            }
        })
    }

    fn get_data_type(self) -> Type {
        match self {
            Type::Path(path) => Type::Path(append_to_end_segment(path, "Data")),
            _ => self,
        }
    }

    fn get_update_type(self) -> Type {
        let ty = match self {
            Type::Path(path) => Type::Path(append_to_end_segment(path, "Update")),
            _ => self,
        };
        syn::parse2::<Type>(quote! { Option<#ty> }).expect("Cannot parse update type of Custom type")
    }
}
