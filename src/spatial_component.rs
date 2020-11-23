use std::convert::TryFrom;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, Field, Ident, ItemStruct};

use crate::utils::{
    get_constructor, get_copiers, get_data_deserializers, get_data_serializers, get_freeers,
    get_id, get_update_deserializers, get_update_serializers, SchemaSerialized, SpatialType,
};

fn generate_component_data_deserialize(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let deserializers = get_data_deserializers(fields, &format_ident!("fields"));
    let constructor = get_constructor(fields, "Self::Data");
    quote! {
        fn component_data_deserialize(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            user_data: *mut core::ffi::c_void,
            mut source: spatialos_sdk::sys_exports::schema::ComponentData
        ) -> Self::Data {
            let mut fields = source.get_fields();
            #(#deserializers;)*
            #constructor
        }
    }
}

fn generate_component_data_serialize(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let serializers =
        get_data_serializers(fields, &format_ident!("fields"), &format_ident!("data"));
    quote! {
        fn component_data_serialize(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            user_data: *mut core::ffi::c_void,
            data: &mut Self::Data,
        ) -> spatialos_sdk::sys_exports::schema::ComponentData {
            let mut component_data = spatialos_sdk::sys_exports::schema::ComponentData::new();
            let mut fields = component_data.get_fields();
            #(#serializers;)*
            component_data
        }
    }
}

fn generate_component_update_deserialize(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let deserializers = get_update_deserializers(fields, &format_ident!("fields"));
    let constructor = get_constructor(fields, "Self::Update");
    quote! {
        fn component_update_deserialize(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            user_data: *mut core::ffi::c_void,
            mut source: spatialos_sdk::sys_exports::schema::ComponentUpdate,
        ) -> Self::Update {
            let mut fields = source.get_fields();
            #(#deserializers;)*
            #constructor
        }
    }
}

fn generate_component_update_serialize(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let serializers =
        get_update_serializers(fields, &format_ident!("fields"), &format_ident!("data"));
    quote! {
        fn component_update_serialize(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            user_data: *mut core::ffi::c_void,
            data: &mut Self::Update,
        ) -> spatialos_sdk::sys_exports::schema::ComponentUpdate {
            let mut new_update = spatialos_sdk::sys_exports::schema::ComponentUpdate::new();
            let mut fields = new_update.get_fields();
            #(#serializers;)*
            new_update
        }
    }
}

fn generate_component_update_free(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let freeers = get_freeers(fields, &format_ident!("data"));
    quote! {
        fn component_update_free(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            user_data: *mut core::ffi::c_void,
            data: Self::Update
        ) {
            #(#freeers;)*
        }
    }
}

fn generate_component_update_copy(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let copiers = get_copiers(fields, &format_ident!("new_data"), &format_ident!("data"));
    quote! {
        fn component_update_copy(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            user_data: *mut core::ffi::c_void,
            data: &Self::Update
        ) -> Self::Update {
            let mut new_data = data.clone();
            #(#copiers;)*
            new_data
        }
    }
}

fn generate_impl_component(
    struct_name: &Ident,
    id: u32,
    fields: &Punctuated<Field, Comma>,
) -> impl ToTokens {
    let update_struct_name = format_ident!("{}Update", struct_name);
    let data_struct_name = format_ident!("{}Data", struct_name);
    let component_data_deserialize = generate_component_data_deserialize(fields);
    let component_data_serialize = generate_component_data_serialize(fields);
    let component_update_deserialize = generate_component_update_deserialize(fields);
    let component_update_serialize = generate_component_update_serialize(fields);
    let component_update_free = generate_component_update_free(fields);
    let component_update_copy = generate_component_update_copy(fields);

    quote! {
        impl spatialos_sdk::Component for #struct_name {

            const ID: spatialos_sdk::sys_exports::worker::ComponentId = #id;

            type Data = #data_struct_name;
            type Update = #update_struct_name;

            #component_data_deserialize

            #component_data_serialize

            #component_update_deserialize

            #component_update_serialize

            #component_update_free

            #component_update_copy

        }
    }
}

fn generate_data_struct(struct_name: &Ident, fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let data_struct_name = format_ident!("{}Data", struct_name);
    let idents = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<_>>();
    let types = fields
        .iter()
        .filter_map(|field| SpatialType::try_from(&field.ty).ok())
        .map(|ty| ty.get_data_type())
        .collect::<Vec<_>>();
    quote! {
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct #data_struct_name {
            #(pub #idents: #types,)*
        }
    }
}

fn generate_update_struct(struct_name: &Ident, fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let update_struct_name = format_ident!("{}Update", struct_name);
    let idents = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<_>>();
    let types = fields
        .iter()
        .filter_map(|field| SpatialType::try_from(&field.ty).ok())
        .map(|ty| ty.get_update_type())
        .collect::<Vec<_>>();
    quote! {
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct #update_struct_name {
            #(pub #idents: #types,)*
        }
    }
}

pub fn generate_component(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let attrs = &input.attrs;
    let result = if let Some(id) = get_id(attrs) {
        let struct_name = &input.ident;

        if let syn::Fields::Named(fields) = input.fields {
            let named_fields = &fields.named;
            let data_struct = generate_data_struct(struct_name, named_fields);
            let update_struct = generate_update_struct(struct_name, named_fields);
            let impl_component = generate_impl_component(struct_name, id, named_fields);
            quote! {
                #[automatically_derived]
                #[allow(unused_qualifications)]
                #data_struct

                #[automatically_derived]
                #[allow(unused_qualifications)]
                #update_struct

                #[automatically_derived]
                #[allow(unused_qualifications)]
                #impl_component
            }
        } else {
            panic!("Component should have named fields")
        }
    } else {
        panic!("Unable to find ID")
    };
    result.into()
}
