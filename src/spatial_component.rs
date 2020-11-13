use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemStruct};

fn get_id(args: AttributeArgs) -> Option<u32> {
    if let syn::NestedMeta::Lit(lit) = &args[0] {
        if let syn::Lit::Int(lit_int) = lit {
            if let Ok(base10_id) = lit_int.base10_parse::<u32>() {
                Some(base10_id)
            } else {
                None
            }
        } else {
            None
        }
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
        ) -> Self {
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
            data: &mut Self,
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
    
    let component_data_deserialize = generate_component_data_deserialize();
    let component_data_serialize = generate_component_data_serialize();
    let component_update_deserialize = generate_component_update_deserialize();
    let component_update_serialize = generate_component_update_serialize();
    let component_update_free = generate_component_update_free();
    let component_update_copy = generate_component_update_copy();

    quote! {
        impl spatialos_sdk::Component for #struct_name {
            const ID: spatialos_sdk::sys_exports::worker::ComponentId = #id;

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

fn generate_base_struct(struct_name: &Ident) -> impl ToTokens {
    quote! {
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct #struct_name {

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


pub fn generate_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemStruct);
    let result = if let Some(id) = get_id(args) {
        let struct_name = &input.ident;

        if let syn::Fields::Named(fields) = input.fields {
            let named_fields = &fields.named;
            let base_struct = generate_base_struct(struct_name);
            let update_struct = generate_update_struct(struct_name);
            let impl_component = generate_impl_component(struct_name, id);
            for field in named_fields {
                println!("{:?}", field.attrs);
            }
            quote! {
                #base_struct

                #update_struct

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
