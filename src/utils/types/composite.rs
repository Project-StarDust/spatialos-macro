use std::convert::TryFrom;

use proc_macro2::TokenStream;
use syn::{Ident, Type};

const MAP_KEY_FIELD_ID: u32 = 1u32;
const MAP_VALUE_FIELD_ID: u32 = 2u32;

use crate::utils::{get_one_argument_type, get_two_argument_type, SchemaSerialized};

use super::{CompositeType, PlainType, Primitive};

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
        data_name: Option<&Ident>,
        _: bool,
    ) -> Option<TokenStream> {
        let data_name = data_name?;
        match self {
            Self::Vec(PlainType::Primitive(Primitive::Char)) => Some(quote! {
                #object_name.add_bytes(#id, &#data_name.#ident)
            }),
            Self::Vec(PlainType::Primitive(primitive)) => {
                let func = format_ident!("add_{}_list", primitive.get_name());
                Some(quote! {
                    #object_name.#func(#id, &#data_name.#ident)
                })
            }
            Self::Vec(PlainType::SpatialType(ty)) => {
                let value_ident = format_ident!("value");
                let serializer =
                    ty.generate_data_serializer(id, &value_ident, object_name, None, true)?;
                Some(quote! {
                    #data_name.#ident.iter_mut().for_each(|mut #value_ident| {
                        #serializer
                    })
                })
            }
            Self::Option(ty) => {
                let serializer = ty.generate_data_serializer(id, ident, object_name, None, true);
                Some(quote! {
                    if let Some(mut #ident) = #data_name.#ident.as_mut() {
                        #serializer
                    }
                })
            }
            Self::HashMap(ty1, ty2) => {
                let object_ident = format_ident!("object");
                let key_ident = format_ident!("key");
                let value_ident = format_ident!("value");
                let serializer1 = ty1.generate_data_serializer(
                    MAP_KEY_FIELD_ID,
                    &key_ident,
                    &object_ident,
                    None,
                    true,
                )?;
                let serializer2 = ty2.generate_data_serializer(
                    MAP_VALUE_FIELD_ID,
                    &value_ident,
                    &object_ident,
                    None,
                    true,
                )?;
                Some(quote! {
                    #data_name.#ident.iter_mut().for_each(|(mut #key_ident, mut #value_ident)| {
                        let mut #object_ident = #object_name.add_object(#id);
                        #serializer1;
                        #serializer2;
                    })
                })
            }
        }
    }

    fn generate_data_deserializer(
        self,
        id: u32,
        object_name: &Ident,
        _: Option<&Ident>,
    ) -> Option<TokenStream> {
        match self {
            Self::Vec(PlainType::Primitive(Primitive::Char)) => Some(quote! {
                #object_name.get_bytes(#id)
            }),
            Self::Vec(PlainType::Primitive(primitive)) => {
                let func = format_ident!("get_{}_list", primitive.get_name());
                Some(quote! {
                    #object_name.#func(#id)
                })
            }
            Self::Vec(PlainType::SpatialType(ty)) => {
                let index_ident = format_ident!("idx");
                let deserializer =
                    ty.generate_data_deserializer(id, object_name, Some(&index_ident))?;
                Some(quote! {
                    (0..#object_name.get_object_count(#id)).map(|#index_ident| {
                        #deserializer
                    }).collect()
                })
            }
            Self::Option(ty) => {
                let deserializer = ty.generate_data_deserializer(id, object_name, None)?;
                Some(quote! {
                    if #object_name.get_object_count(#id) > 0 {
                        Some(#deserializer)
                    } else {
                        None
                    }
                })
            }
            Self::HashMap(ty1, ty2) => {
                let serializer1 =
                    ty1.generate_data_deserializer(MAP_KEY_FIELD_ID, object_name, None)?;
                let serializer2 =
                    ty2.generate_data_deserializer(MAP_VALUE_FIELD_ID, object_name, None)?;
                Some(quote! {
                    (0..#object_name.get_object_count(#id)).map(|i| {
                        let mut object = #object_name.index_object(#id, i);
                        let arg1 = #serializer1;
                        let arg2 = #serializer2;
                        (arg1, arg2)
                    }).collect()
                })
            }
        }
    }

    fn generate_update_serializer(
        self,
        id: u32,
        ident: &Ident,
        object_name: &Ident,
        data_name: Option<&Ident>,
        _: bool,
    ) -> Option<TokenStream> {
        let data_name = data_name?;
        match self {
            Self::Vec(PlainType::Primitive(Primitive::Char)) => Some(quote! {
                if let Some(mut #ident) = #data_name.#ident.as_mut() {
                    #object_name.add_bytes(#id, #ident)
                }
            }),
            Self::Vec(PlainType::Primitive(primitive)) => {
                let func = format_ident!("add_{}_list", primitive.get_name());
                Some(quote! {
                    if let Some(mut #ident) = #data_name.#ident.as_mut() {
                        #object_name.#func(#id, #ident)
                    }
                })
            }
            Self::Vec(PlainType::SpatialType(ty)) => {
                let value_ident = format_ident!("value");
                let serializer =
                    ty.generate_data_serializer(id, &value_ident, object_name, None, true)?;
                Some(quote! {
                    if let Some(mut #ident) = #data_name.#ident.as_mut() {
                        #ident.iter_mut().for_each(|mut #value_ident| {
                            #serializer
                        })
                    }
                })
            }
            Self::Option(PlainType::Primitive(ty)) => {
                let serializer =
                    ty.generate_update_serializer(id, ident, object_name, None, true)?;
                Some(quote! {
                    if let Some(mut #ident) = #data_name.#ident.as_mut() {
                        #serializer
                    }
                })
            }
            Self::Option(PlainType::SpatialType(ty)) => {
                let serializer = ty.generate_data_serializer(id, ident, object_name, None, true)?;
                Some(quote! {
                    if let Some(mut #ident) = #data_name.#ident.as_mut() {
                        #serializer
                    }
                })
            }
            _ => None,
        }
    }

    fn generate_update_deserializer(
        self,
        id: u32,
        object_name: &Ident,
        _: Option<&Ident>,
    ) -> Option<TokenStream> {
        match self {
            Self::Vec(PlainType::Primitive(Primitive::Char)) => Some(quote! {
                if #object_name.get_bytes_count(#id) > 0 {
                    Some(#object_name.get_bytes(#id))
                } else {
                    None
                }
            }),
            Self::Vec(PlainType::Primitive(primitive)) => {
                let func = format_ident!("get_optional_{}_list", primitive.get_name());
                Some(quote! {
                    #object_name.#func(#id)
                })
            }
            Self::Vec(PlainType::SpatialType(ty)) => {
                let index_ident = format_ident!("idx");
                let deserializer =
                    ty.generate_data_deserializer(id, object_name, Some(&index_ident))?;
                Some(quote! {
                    if #object_name.get_object_count(#id) > 0 {
                        Some((0..#object_name.get_object_count(#id)).map(|#index_ident| {
                            #deserializer
                        }).collect())
                    } else {
                        None
                    }
                })
            }
            Self::Option(PlainType::Primitive(ty)) => {
                let deserializer = ty.generate_update_deserializer(id, object_name, None)?;
                Some(quote! {
                    if #object_name.get_object_count(#id) > 0 {
                        Some(#deserializer)
                    } else {
                        None
                    }
                })
            }
            Self::Option(PlainType::SpatialType(ty)) => {
                let deserializer = ty.generate_data_deserializer(id, object_name, None);
                Some(quote! {
                    if #object_name.get_object_count(#id) > 0 {
                        Some(#deserializer)
                    } else {
                        None
                    }
                })
            }
            Self::HashMap(ty1, ty2) => {
                let object_ident = format_ident!("object");
                let index_ident = format_ident!("idx");
                let deserializer1 =
                    ty1.generate_data_deserializer(MAP_KEY_FIELD_ID, &object_ident, None)?;
                let deserializer2 =
                    ty2.generate_data_deserializer(MAP_VALUE_FIELD_ID, &object_ident, None)?;
                Some(quote! {
                    if #object_name.get_object_count(#id) > 0 {
                        Some((0..#object_name.get_object_count(#id)).map(|#index_ident| {
                            let mut object = #object_name.index_object(#id, #index_ident);
                            let arg1 = #deserializer1;
                            let arg2 = #deserializer2;
                            (arg1, arg2)
                        }).collect())
                    } else {
                        None
                    }
                })
            }
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
                    HashMap<#inner_type1, #inner_type2>
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
