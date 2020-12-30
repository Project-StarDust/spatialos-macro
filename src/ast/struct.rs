use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Fields, Ident, ItemStruct};

use super::{field::FieldAST, get_id};

#[derive(Debug)]
pub struct StructAST<'a> {
    pub id: Option<u32>,
    pub name: &'a Ident,
    pub fields: Vec<FieldAST>,
}

impl StructAST<'_> {
    pub fn is_component(&self) -> bool {
        self.id.is_some()
    }

    fn get_update_constructor(&self) -> TokenStream2 {
        let idents = self
            .fields
            .iter()
            .map(|field| &field.name)
            .collect::<Vec<_>>();
        quote! {
            Self::Update { #(#idents,)* }
        }
    }

    fn get_data_constructor(&self) -> TokenStream2 {
        let idents = self
            .fields
            .iter()
            .map(|field| &field.name)
            .collect::<Vec<_>>();
        quote! {
            Self::Data { #(#idents,)* }
        }
    }

    fn get_data_deserializers(&self, source: &Ident) -> TokenStream2 {
        let deserializers = self
            .fields
            .iter()
            .map(|field| field.get_data_deserializer(source))
            .collect::<Vec<_>>();
        quote! {
            #(#deserializers)*
        }
    }

    fn get_data_serializers(&self, data: &Ident, target: &Ident) -> TokenStream2 {
        let serializers = self
            .fields
            .iter()
            .map(|field| field.get_data_serializer(data, target))
            .collect::<Vec<_>>();
        quote! {
            #(#serializers)*
        }
    }

    fn get_update_deserializers(&self, source: &Ident) -> TokenStream2 {
        let deserializers = self
            .fields
            .iter()
            .map(|field| field.get_update_deserializer(source))
            .collect::<Vec<_>>();
        quote! {
            #(#deserializers)*
        }
    }

    fn get_update_serializers(&self, data: &Ident, target: &Ident) -> TokenStream2 {
        let serializers = self
            .fields
            .iter()
            .map(|field| field.get_update_serializer(data, target))
            .collect::<Vec<_>>();
        quote! {
            #(#serializers)*
        }
    }

    fn get_impl_type(
        &self,
        struct_name: &Ident,
        data_struct_name: &Ident,
        update_struct_name: &Ident,
    ) -> TokenStream2 {
        let type_data_deserialize = {
            let source = format_ident!("source");
            let deserializers = self.get_data_deserializers(&source);
            let constructor = self.get_data_constructor();
            quote! {
                fn type_data_deserialize(
                    user_data: *mut core::ffi::c_void,
                    #source: &mut spatialos::schema::Object,
                ) -> Self::Data {
                    #deserializers
                    #constructor
                }
            }
        };

        let type_data_serialize = {
            let target = format_ident!("target");
            let data = format_ident!("data");
            let serializers = self.get_data_serializers(&data, &target);
            quote! {
                fn type_data_serialize(
                    user_data: *mut core::ffi::c_void,
                    #data: &mut Self::Data,
                    #target: &mut spatialos::schema::Object,
                ) {
                    #serializers
                }
            }
        };

        let type_update_deserialize = {
            let source = format_ident!("source");
            let deserializers = self.get_update_deserializers(&source);
            let constructor = self.get_update_constructor();
            quote! {
                fn type_update_deserialize(
                    user_data: *mut core::ffi::c_void,
                    #source: &mut spatialos::schema::Object,
                ) -> Self::Update {
                    #deserializers
                    #constructor
                }
            }
        };

        let type_update_serialize = {
            let target = format_ident!("target");
            let data = format_ident!("data");
            let serializers = self.get_update_serializers(&data, &target);
            quote! {
                fn type_update_serialize(
                    user_data: *mut core::ffi::c_void,
                    #data: &mut Self::Update,
                    #target: &mut spatialos::schema::Object,
                ) {
                    #serializers
                }
            }
        };

        let type_update_free = {
            let data = format_ident!("data");
            let freeers = quote! {};
            quote! {
                fn type_update_free(user_data: *mut core::ffi::c_void, #data: Self::Update) {
                    #freeers
                }
            }
        };

        let type_update_copy = {
            let data = format_ident!("data");
            let new_data = format_ident!("new_data");
            let copiers = quote! {};
            quote! {
                fn type_update_copy(user_data: *mut core::ffi::c_void, #data: &Self::Update) -> Self::Update {
                    let mut #new_data = data.clone();
                    #copiers
                    #new_data
                }
            }
        };

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

    fn get_impl_component(
        &self,
        struct_name: &Ident,
        data_struct_name: &Ident,
        update_struct_name: &Ident,
    ) -> TokenStream2 {
        let id = self.id.unwrap();
        let component_data_deserialize = {
            let fields = format_ident!("fields");
            let deserializers = self.get_data_deserializers(&fields);
            let constructor = self.get_data_constructor();
            quote! {
                fn component_data_deserialize(
                    component_id: spatialos::worker::ComponentId,
                    user_data: *mut core::ffi::c_void,
                    mut source: spatialos::schema::ComponentData
                ) -> Self::Data {
                    let mut #fields = source.get_fields();
                    #deserializers
                    #constructor
                }
            }
        };

        let component_data_serialize = {
            let fields = format_ident!("fields");
            let data = format_ident!("data");
            let serializers = self.get_data_serializers(&data, &fields);
            quote! {
                fn component_data_serialize(
                    component_id: spatialos::worker::ComponentId,
                    user_data: *mut core::ffi::c_void,
                    #data: &mut Self::Data,
                ) -> spatialos::schema::ComponentData {
                    let mut component_data = spatialos::schema::ComponentData::new();
                    let mut #fields = component_data.get_fields();
                    #serializers
                    component_data
                }
            }
        };

        let component_update_deserialize = {
            let fields = format_ident!("fields");
            let deserializers = self.get_update_deserializers(&fields);
            let constructor = self.get_update_constructor();
            quote! {
                fn component_update_deserialize(
                    component_id: spatialos::worker::ComponentId,
                    user_data: *mut core::ffi::c_void,
                    mut source: spatialos::schema::ComponentUpdate,
                ) -> Self::Update {
                    let mut #fields = source.get_fields();
                    #deserializers
                    #constructor
                }
            }
        };

        let component_update_serialize = {
            let fields = format_ident!("fields");
            let data = format_ident!("data");
            let serializers = self.get_update_serializers(&data, &fields);
            quote! {
                fn component_update_serialize(
                    component_id: spatialos::worker::ComponentId,
                    user_data: *mut core::ffi::c_void,
                    #data: &mut Self::Update,
                ) -> spatialos::schema::ComponentUpdate {
                    let mut new_update = spatialos::schema::ComponentUpdate::new();
                    let mut #fields = new_update.get_fields();
                    #serializers
                    new_update
                }
            }
        };

        let component_update_free = {
            let data = format_ident!("data");
            let freeers = quote! {};
            quote! {
                fn component_update_free(
                    component_id: spatialos::worker::ComponentId,
                    user_data: *mut core::ffi::c_void,
                    #data: Self::Update
                ) {
                    #freeers
                }
            }
        };

        let component_update_copy = {
            let data = format_ident!("data");
            let new_data = format_ident!("new_data");
            let copiers = quote! {};
            quote! {
                fn component_update_copy(
                    component_id: spatialos::worker::ComponentId,
                    user_data: *mut core::ffi::c_void,
                    #data: &Self::Update
                ) -> Self::Update {
                    let mut #new_data = #data.clone();
                    #copiers
                    #new_data
                }
            }
        };

        quote! {
            impl spatialos_sdk::Component for #struct_name {

                const ID: u32 = #id;

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
}

impl Into<TokenStream> for StructAST<'_> {
    fn into(self) -> TokenStream {
        let struct_name = &self.name;
        let data_struct_name = format_ident!("{}Data", &self.name);
        let update_struct_name = format_ident!("{}Update", &self.name);

        let data_struct = {
            let fields = self
                .fields
                .iter()
                .map(|field| field.get_data_field())
                .collect::<Vec<_>>();
            quote! {
                #[repr(C)]
                #[derive(Debug, Clone)]
                pub struct #data_struct_name {
                    #(#fields,)*
                }
            }
        };
        let update_struct = {
            let fields = self
                .fields
                .iter()
                .map(|field| field.get_update_field())
                .collect::<Vec<_>>();
            quote! {
                #[repr(C)]
                #[derive(Debug, Clone)]
                pub struct #update_struct_name {
                    #(#fields,)*
                }
            }
        };
        let implementation = {
            if self.is_component() {
                self.get_impl_component(struct_name, &data_struct_name, &update_struct_name)
            } else {
                self.get_impl_type(struct_name, &data_struct_name, &update_struct_name)
            }
        };

        let result = quote! {
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #data_struct

            #[automatically_derived]
            #[allow(unused_qualifications)]
            #update_struct

            #[automatically_derived]
            #[allow(unused_qualifications)]
            #implementation
        };
        result.into()
    }
}

impl<'a> From<&'a ItemStruct> for StructAST<'a> {
    fn from(input: &'a ItemStruct) -> Self {
        let id = get_id(&input.attrs);
        let name = &input.ident;
        if let Fields::Named(fields) = &input.fields {
            let fields = fields.named.iter().map(FieldAST::from).collect::<Vec<_>>();
            Self { id, name, fields }
        } else {
            panic!("Didn't find fields for struct");
        }
    }
}
