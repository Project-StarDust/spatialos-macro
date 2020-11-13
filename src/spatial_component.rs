use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, Attribute, Ident, ItemStruct};

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

pub fn generate_component_data_deserialize() -> impl ToTokens {
    quote! {
        fn component_data_deserialize(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            _: *mut core::ffi::c_void,
            mut source: spatialos_sdk::sys_exports::schema::ComponentData
        ) -> Self::Data {
            assert_eq!(component_id, Self::ID);
            unimplemented!()
        }
    }
}

pub fn generate_component_data_serialize() -> impl ToTokens {
    quote! {
        fn component_data_serialize(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            _: *mut core::ffi::c_void,
            data: &mut Self::Data,
        ) -> spatialos_sdk::sys_exports::schema::ComponentData {
            assert_eq!(component_id, Self::ID);
            unimplemented!()
        }
    }
}

pub fn generate_component_update_deserialize() -> impl ToTokens {
    quote! {
        fn component_update_deserialize(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            _: *mut core::ffi::c_void,
            mut source: spatialos_sdk::sys_exports::schema::ComponentUpdate,
        ) -> Self::Update {
            assert_eq!(component_id, Self::ID);
            unimplemented!()
        }
    }
}

pub fn generate_component_update_serialize() -> impl ToTokens {
    quote! {
        fn component_update_serialize(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            _: *mut core::ffi::c_void,
            data: &mut Self::Update,
        ) -> spatialos_sdk::sys_exports::schema::ComponentUpdate {
            assert_eq!(component_id, Self::ID);
            unimplemented!()
        }
    }
}

pub fn generate_component_update_free() -> impl ToTokens {
    quote! {
        fn component_update_free(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            _: *mut core::ffi::c_void,
            data: Self::Update
        ) {
            assert_eq!(component_id, Self::ID);
            unimplemented!()
        }
    }
}

pub fn generate_component_update_copy() -> impl ToTokens {
    quote! {
        fn component_update_copy(
            component_id: spatialos_sdk::sys_exports::worker::ComponentId,
            _: *mut core::ffi::c_void,
            data: &Self::Update
        ) -> Self::Update {
            assert_eq!(component_id, Self::ID);
            unimplemented!()
        }
    }
}

pub fn generate_impl_component(struct_name: &Ident, id: u32) -> impl ToTokens {
    let update_struct_name = format_ident!("{}Update", struct_name);
    let data_struct_name = format_ident!("{}Data", struct_name);
    let component_data_deserialize = generate_component_data_deserialize();
    let component_data_serialize = generate_component_data_serialize();
    let component_update_deserialize = generate_component_update_deserialize();
    let component_update_serialize = generate_component_update_serialize();
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

fn generate_data_struct(struct_name: &Ident) -> impl ToTokens {
    let data_struct_name = format_ident!("{}Data", struct_name);
    quote! {
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct #data_struct_name {

        }
    }
}

fn generate_update_struct(struct_name: &Ident) -> impl ToTokens {
    let update_struct_name = format_ident!("{}Update", struct_name);
    quote! {
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct #update_struct_name {

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
            let data_struct = generate_data_struct(struct_name);
            let update_struct = generate_update_struct(struct_name);
            let impl_component = generate_impl_component(struct_name, id);
            for field in named_fields {
                println!("{:?}", field.attrs);
            }
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
