use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Attribute, Field, Ident, ItemStruct,
    Type,
};

fn get_id(attrs: &Vec<Attribute>) -> Option<u32> {
    let attribute: syn::Lit = attrs
        .iter()
        .find(|attr| attr.path.is_ident("id"))
        .map(|attr| attr.parse_args())?
        .ok()?;

    if let syn::Lit::Int(lit_int) = attribute {
        lit_int.base10_parse::<u32>().ok()
    } else {
        None
    }
}

fn get_field_id(attrs: &Vec<Attribute>) -> Option<u32> {
    let attribute: syn::Lit = attrs
        .iter()
        .find(|attr| attr.path.is_ident("field_id"))
        .map(|attr| attr.parse_args())?
        .ok()?;

    if let syn::Lit::Int(lit_int) = attribute {
        lit_int.base10_parse::<u32>().ok()
    } else {
        None
    }
}

fn generate_schema_deserializer(id: u32, ty: &Type, name: &Ident) -> impl ToTokens {
    if let Type::Path(ident) = ty {
        if let Some(ident) = ident.path.get_ident() {
            match ident.to_string().as_str() {
                "bool" => quote! { let #name = fields.get_bool(#id) },
                "f64" => quote! { let #name = fields.get_double(#id) },
                "f32" => quote! { let #name = fields.get_float(#id) },
                "u32" => quote! { let #name = fields.get_uint32(#id) },
                "u64" => quote! { let #name = fields.get_uint64(#id) },
                "i32" => quote! { let #name = fields.get_int32(#id) },
                "i64" => quote! { let #name = fields.get_int64(#id) },
                _ => quote! {},
            }
        } else {
            quote! {}
        }
    } else {
        quote! {}
    }
}

fn generate_schema_serializer(id: u32, ty: &Type, name: &Ident) -> impl ToTokens {
    if let Type::Path(ident) = ty {
        if let Some(ident) = ident.path.get_ident() {
            match ident.to_string().as_str() {
                "bool" => quote! { fields.add_bool(#id, data.#name) },
                "f64" => quote! { fields.add_double(#id, data.#name) },
                "f32" => quote! { fields.add_float(#id, data.#name) },
                "u32" => quote! { fields.add_uint32(#id, data.#name) },
                "u64" => quote! { fields.add_uint64(#id, data.#name) },
                "i32" => quote! { fields.add_int32(#id, data.#name) },
                "i64" => quote! { fields.add_int64(#id, data.#name) },
                _ => quote! {},
            }
        } else {
            quote! {}
        }
    } else {
        quote! {}
    }
}

fn generate_component_data_deserialize(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let idents = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<_>>();
    let types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();
    let fields_id = fields
        .iter()
        .filter_map(|field| get_field_id(&field.attrs))
        .collect::<Vec<_>>();
    let schema_deserializers = fields_id
        .into_iter()
        .zip(types.into_iter())
        .zip(idents.iter())
        .map(|((id, ty), ident)| generate_schema_deserializer(id, ty, ident))
        .collect::<Vec<_>>();
    quote! {
        fn component_data_deserialize(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            _: *mut core::ffi::c_void,
            mut source: spatialos_sdk::sys_exports::schema::ComponentData
        ) -> Self::Data {
            let mut fields = source.get_fields();
            #(#schema_deserializers;)*
            Self::Data {
                #(#idents),*
            }
        }
    }
}

fn generate_component_data_serialize(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let idents = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<_>>();
    let types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();
    let fields_id = fields
        .iter()
        .filter_map(|field| get_field_id(&field.attrs))
        .collect::<Vec<_>>();
    let schema_serializers = fields_id
        .into_iter()
        .zip(types.into_iter())
        .zip(idents.iter())
        .map(|((id, ty), ident)| generate_schema_serializer(id, ty, ident))
        .collect::<Vec<_>>();
    quote! {
        fn component_data_serialize(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            _: *mut core::ffi::c_void,
            data: &mut Self::Data,
        ) -> spatialos_sdk::sys_exports::schema::ComponentData {
            let mut component_data = spatialos_sdk::sys_exports::schema::ComponentData::new();
            let mut fields = component_data.get_fields();
            #(#schema_serializers;)*
            component_data
        }
    }
}

fn generate_component_update_deserialize(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let idents = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<_>>();
    let types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();
    let fields_id = fields
        .iter()
        .filter_map(|field| get_field_id(&field.attrs))
        .collect::<Vec<_>>();
    let schema_deserializers = fields_id
        .into_iter()
        .zip(types.into_iter())
        .zip(idents.iter())
        .map(|((id, ty), ident)| generate_schema_deserializer(id, ty, ident))
        .collect::<Vec<_>>();
    quote! {
        fn component_update_deserialize(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            _: *mut core::ffi::c_void,
            mut source: spatialos_sdk::sys_exports::schema::ComponentUpdate,
        ) -> Self::Update {
            let mut fields = source.get_fields();
             #(#schema_deserializers;)*
            Self::Update {
                #(#idents),*
            }
        }
    }
}

fn generate_component_update_serialize(fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let idents = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<_>>();
    let types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();
    let fields_id = fields
        .iter()
        .filter_map(|field| get_field_id(&field.attrs))
        .collect::<Vec<_>>();
    let schema_serializers = fields_id
        .into_iter()
        .zip(types.into_iter())
        .zip(idents.iter())
        .map(|((id, ty), ident)| generate_schema_serializer(id, ty, ident))
        .collect::<Vec<_>>();
    quote! {
        fn component_update_serialize(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            _: *mut core::ffi::c_void,
            data: &mut Self::Update,
        ) -> spatialos_sdk::sys_exports::schema::ComponentUpdate {
            let mut new_update = spatialos_sdk::sys_exports::schema::ComponentUpdate::new();
            let mut fields = new_update.get_fields();
            #(#schema_serializers;)*
            new_update
        }
    }
}

fn generate_component_update_free() -> impl ToTokens {
    quote! {
        fn component_update_free(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            _: *mut core::ffi::c_void,
            data: Self::Update
        ) {
        }
    }
}

fn generate_component_update_copy() -> impl ToTokens {
    quote! {
        fn component_update_copy(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            _: *mut core::ffi::c_void,
            data: &Self::Update
        ) -> Self::Update {
            let mut new_data = data.clone();
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
    let component_update_free = generate_component_update_free();
    let component_update_copy = generate_component_update_copy();

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
    let types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();
    quote! {
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct #data_struct_name {
            #(#idents: #types,)*
        }
    }
}

fn generate_update_struct(struct_name: &Ident, fields: &Punctuated<Field, Comma>) -> impl ToTokens {
    let update_struct_name = format_ident!("{}Update", struct_name);
    let idents = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<_>>();
    let types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();
    quote! {
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct #update_struct_name {
            #(#idents: #types,)*
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
