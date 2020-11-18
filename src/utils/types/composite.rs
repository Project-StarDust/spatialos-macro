use std::convert::TryFrom;

use proc_macro2::TokenStream;
use syn::{Ident, Type};

use crate::utils::{get_one_argument_type, get_two_argument_type, SchemaSerialized};

use super::{CompositeType, PlainType};

impl TryFrom<&Type> for CompositeType {
    type Error = ();

    fn try_from(value: &Type) -> Result<Self, Self::Error> {
        if let Type::Path(type_path) = value {
            let segment = type_path.path.segments.last().ok_or(())?;
            match segment.ident.to_string().as_str() {
                "Vec" => {
                    let arg = get_one_argument_type(value).ok_or(())?;
                    let plain_type = PlainType::try_from(&arg)?;
                    Ok(CompositeType::Vec(plain_type))
                }
                "HashMap" => {
                    let (arg1, arg2) = get_two_argument_type(value).ok_or(())?;
                    let plain_type1 = PlainType::try_from(&arg1)?;
                    let plain_type2 = PlainType::try_from(&arg2)?;
                    Ok(CompositeType::HashMap(plain_type1, plain_type2))
                }
                "Option" => {
                    let arg = get_one_argument_type(value).ok_or(())?;
                    let plain_type = PlainType::try_from(&arg)?;
                    Ok(CompositeType::Option(plain_type))
                }
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

impl SchemaSerialized for CompositeType {
    fn generate_data_serializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
        data_name: &Ident,
    ) -> Option<TokenStream> {
        match self {
            Self::Vec(PlainType::Primitive(primitive)) => {
                let func = format_ident!("add_{}_list", primitive.get_name());
                Some(quote! {
                    #object_name.#func(#id, &#data_name.#ident)
                })
            }
            _ => None,
        }
    }

    fn generate_data_deserializer(
        self,
        id: u32,
        _: &Ident,
        object_name: &Ident,
    ) -> Option<TokenStream> {
        match self {
            Self::Vec(PlainType::Primitive(primitive)) => {
                let func = format_ident!("get_{}_list", primitive.get_name());
                Some(quote! {
                    #object_name.#func(#id)
                })
            }
            Self::HashMap(p1, p2) => {
                /*let serializer1 =
                    generate_primitive_deserializer_enum(p1, schema::MAP_KEY_FIELD_ID, object_name);
                let serializer2 =
                    generate_primitive_deserializer_enum(p2, schema::MAP_VALUE_FIELD_ID, object_name);
                Some(quote! {
                    let #ident = (0..#object_name.get_object_count(#id)).map(|i| {
                        let object = #object_name.index_object(#id, i);
                        let arg1 = #serializer1;
                        let arg2 = #serializer2;
                        (arg1, arg2)
                    }).collect()
                })*/
                None
            }
            _ => None,
        }
    }

    fn generate_update_serializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
        data_name: &Ident,
    ) -> Option<TokenStream> {
        let tokens = match self {
            Self::Vec(PlainType::Primitive(primitive)) => {
                let func = format_ident!("add_{}_list", primitive.get_name());
                Some(quote! {
                    #object_name.#func(#id, #ident)
                })
            }
            _ => None,
        }?;
        Some(quote! {
            if let Some(#ident) = &#data_name.#ident {
                #tokens;
            }
        })
    }

    fn generate_update_deserializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
    ) -> Option<TokenStream> {
        match self {
            Self::Vec(PlainType::Primitive(primitive)) => {
                let func = format_ident!("get_optional_{}_list", primitive.get_name());
                Some(quote! {
                    #object_name.#func(#id)
                })
            }
            _ => None,
        }
    }

    fn generate_update_copier(
        self,
        new_data: &Ident,
        data: &Ident,
        ident: &Ident,
    ) -> Option<TokenStream> {
        Some(quote! {
            #new_data.#ident = #data.#ident.clone();
        })
    }

    fn generate_update_freeer(self, _: &Ident, _: &Ident) -> Option<TokenStream> {
        None
    }

    fn get_data_type(self) -> Type {
        let tokens = match self {
            Self::Vec(ty) => {
                let inner_type = ty.get_data_type();
                quote! {
                    Vec<#inner_type>
                }
            }
            Self::HashMap(ty1, ty2) => {
                let inner_type1 = ty1.get_data_type();
                let inner_type2 = ty2.get_data_type();
                quote! {
                    HashMap<#inner_type1, #inner_type2>>
                }
            }
            Self::Option(ty) => {
                let inner_type = ty.get_data_type();
                quote! {
                    Option<#inner_type>
                }
            }
        };
        syn::parse2::<Type>(tokens).expect("Cannot parse data type for Composite")
    }

    fn get_update_type(self) -> Type {
        let tokens = match self {
            Self::Vec(ty) => {
                let inner_type = ty.get_data_type();
                quote! {
                    Option<Vec<#inner_type>>
                }
            }
            Self::HashMap(ty1, ty2) => {
                let inner_type1 = ty1.get_data_type();
                let inner_type2 = ty2.get_data_type();
                quote! {
                    Option<HashMap<#inner_type1, #inner_type2>>
                }
            }
            Self::Option(ty) => {
                let inner_type = ty.get_data_type();
                quote! {
                    Option<#inner_type>
                }
            }
        };
        syn::parse2::<Type>(tokens).expect("Cannot parse update type for Composite")
    }
}
