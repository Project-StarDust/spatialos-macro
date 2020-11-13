use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, Field, Ident, ItemStruct};

use crate::utils::{
    get_constructor, get_copiers, get_data_deserializers, get_data_serializers, get_freeers,
    get_update_deserializers, get_update_serializers, transform_non_primitive_data,
    transform_non_primitive_update,
};

fn generate_type_data_deserialize(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let deserializers = get_data_deserializers(fields, &format_ident!("source"));
    let constructor = get_constructor(fields, "Self::Data");
    quote! {
        fn type_data_deserialize(
            user_data: *mut core::ffi::c_void,
            source: &mut spatialos_sdk::sys_exports::schema::Object,
        ) -> Self::Data {
            #(#deserializers;)*
            #constructor
        }
    }
}

fn generate_type_data_serialize(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let serializers =
        get_data_serializers(fields, &format_ident!("target"), &format_ident!("data"));
    quote! {
        fn type_data_serialize(
            user_data: *mut core::ffi::c_void,
            data: &mut Self::Data,
            target: &mut spatialos_sdk::sys_exports::schema::Object,
        ) {
            #(#serializers;)*
        }
    }
}

fn generate_type_update_deserialize(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let deserializers = get_update_deserializers(fields, &format_ident!("source"));
    let constructor = get_constructor(fields, "Self::Update");
    quote! {
        fn type_update_deserialize(
            user_data: *mut core::ffi::c_void,
            source: &mut spatialos_sdk::sys_exports::schema::Object,
        ) -> Self::Update {
            #(#deserializers;)*
            #constructor
        }
    }
}

fn generate_type_update_serialize(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let serializers =
        get_update_serializers(fields, &format_ident!("target"), &format_ident!("data"));
    quote! {
        fn type_update_serialize(
            user_data: *mut core::ffi::c_void,
            data: &mut Self::Update,
            target: &mut spatialos_sdk::sys_exports::schema::Object,
        ) {
            #(#serializers;)*
        }
    }
}

fn generate_type_update_free(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let freeers = get_freeers(fields, &format_ident!("data"));
    quote! {
        fn type_update_free(user_data: *mut core::ffi::c_void, data: Self::Update) {
            #(#freeers;)*
        }
    }
}

fn generate_type_update_copy(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let copiers = get_copiers(fields, &format_ident!("new_data"), &format_ident!("data"));
    quote! {
        fn type_update_copy(user_data: *mut core::ffi::c_void, data: &Self::Update) -> Self::Update {
            let new_data = data.clone();
            #(#copiers;)*
            new_data
        }
    }
}

fn generate_impl_type(struct_name: &Ident, fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let update_struct_name = format_ident!("{}Update", struct_name);
    let data_struct_name = format_ident!("{}Data", struct_name);
    let type_data_deserialize = generate_type_data_deserialize(fields);
    let type_data_serialize = generate_type_data_serialize(fields);
    let type_update_deserialize = generate_type_update_deserialize(fields);
    let type_update_serialize = generate_type_update_serialize(fields);
    let type_update_free = generate_type_update_free(fields);
    let type_update_copy = generate_type_update_copy(fields);

    quote! {
        impl spatialos_sdk::Type for #struct_name {

            type Data = #data_struct_name;
            type Update = #update_struct_name;

            #type_data_deserialize

            #type_data_serialize

            #type_update_deserialize

            #type_update_serialize

            #type_update_free

            #type_update_copy

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
        .map(|field| field.ty.clone())
        .map(transform_non_primitive_data)
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
        .map(|field| field.ty.clone())
        .map(transform_non_primitive_update)
        .collect::<Vec<_>>();
    quote! {
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct #update_struct_name {
            #(pub #idents: #types,)*
        }
    }
}

pub fn generate_type(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;

    let result = if let syn::Fields::Named(fields) = input.fields {
        let named_fields = &fields.named;
        let data_struct = generate_data_struct(struct_name, named_fields);
        let update_struct = generate_update_struct(struct_name, named_fields);
        let impl_type = generate_impl_type(struct_name, named_fields);
        quote! {
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #data_struct

            #[automatically_derived]
            #[allow(unused_qualifications)]
            #update_struct

            #[automatically_derived]
            #[allow(unused_qualifications)]
            #impl_type
        }
    } else {
        panic!("Component should have named fields")
    };
    result.into()
}
