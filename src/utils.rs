use proc_macro2::TokenStream;
use quote::ToTokens;

use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Field, Ident, Path, PathSegment, Token, Type,
    TypePath, TypePtr,
};

pub fn get_primitive(ty: &Type) -> Option<&str> {
    if let Type::Path(ident) = ty {
        let ident = ident.path.get_ident()?;
        match ident.to_string().as_str() {
            "bool" => Some("bool"),
            "f64" => Some("f64"),
            "f32" => Some("f32"),
            "u32" => Some("u32"),
            "u64" => Some("u64"),
            "i32" => Some("i32"),
            "i64" => Some("i64"),
            _ => None,
        }
    } else {
        None
    }
}

pub fn get_non_primitive(ty: &Type) -> Option<&Type> {
    if get_primitive(ty).is_none() {
        Some(ty)
    } else {
        None
    }
}

pub fn generate_primitive_serializer(
    ty: &Type,
    id: u32,
    ident: &Ident,
    object_name: &Ident,
    data_name: &Ident,
) -> Option<TokenStream> {
    let primitive = get_primitive(ty)?;
    match primitive {
        "bool" => Some(quote! { #object_name.add_bool(#id, #data_name.#ident) }),
        "f64" => Some(quote! { #object_name.add_double(#id, #data_name.#ident) }),
        "f32" => Some(quote! { #object_name.add_float(#id, #data_name.#ident) }),
        "u32" => Some(quote! { #object_name.add_uint32(#id, #data_name.#ident) }),
        "u64" => Some(quote! { #object_name.add_uint64(#id, #data_name.#ident) }),
        "i32" => Some(quote! { #object_name.add_int32(#id, #data_name.#ident) }),
        "i64" => Some(quote! { #object_name.add_int64(#id, #data_name.#ident) }),
        _ => None,
    }
}

pub fn generate_data_serializer(
    ty: &Type,
    id: u32,
    ident: &Ident,
    object_name: &Ident,
    data_name: &Ident,
) -> TokenStream {
    if let Some(primitive_serializer) =
        generate_primitive_serializer(ty, id, ident, object_name, data_name)
    {
        primitive_serializer
    } else {
        quote! {
            <#ty as spatialos_sdk::Type>::type_data_serialize(user_data, &mut #data_name.#ident, &mut #object_name.add_object(#id))
        }
    }
}

pub fn generate_update_serializer(
    ty: &Type,
    id: u32,
    ident: &Ident,
    object_name: &Ident,
    data_name: &Ident,
) -> TokenStream {
    if let Some(primitive_serializer) =
        generate_primitive_serializer(ty, id, ident, object_name, data_name)
    {
        primitive_serializer
    } else {
        quote! {
            if !#data_name.#ident.is_null() {
                let mut #ident = unsafe { Box::from_raw(data.coords) };
                <#ty as spatialos_sdk::Type>::type_update_serialize(user_data, &mut *#ident, &mut #object_name.add_object(#id));
                Box::into_raw(#ident);
            }
        }
    }
}

pub fn generate_primitive_deserializer(
    ty: &Type,
    id: u32,
    ident: &Ident,
    object_name: &Ident,
) -> Option<TokenStream> {
    let primitive = get_primitive(ty)?;
    match primitive {
        "bool" => Some(quote! { let #ident = #object_name.get_bool(#id) }),
        "f64" => Some(quote! { let #ident = #object_name.get_double(#id) }),
        "f32" => Some(quote! { let #ident = #object_name.get_float(#id) }),
        "u32" => Some(quote! { let #ident = #object_name.get_uint32(#id) }),
        "u64" => Some(quote! { let #ident = #object_name.get_uint64(#id) }),
        "i32" => Some(quote! { let #ident = #object_name.get_int32(#id) }),
        "i64" => Some(quote! { let #ident = #object_name.get_int64(#id) }),
        _ => None,
    }
}

pub fn generate_data_deserializer(
    ty: &Type,
    id: u32,
    ident: &Ident,
    object_name: &Ident,
) -> TokenStream {
    if let Some(primitive_serializer) = generate_primitive_deserializer(ty, id, ident, object_name)
    {
        primitive_serializer
    } else {
        quote! {
            let #ident = <#ty as spatialos_sdk::Type>::type_data_deserialize(user_data, &mut #object_name.get_object(#id))
        }
    }
}

pub fn generate_update_deserializer(
    ty: &Type,
    id: u32,
    ident: &Ident,
    object_name: &Ident,
) -> TokenStream {
    if let Some(primitive_serializer) = generate_primitive_deserializer(ty, id, ident, object_name)
    {
        primitive_serializer
    } else {
        quote! {
            let #ident = if #object_name.get_object_count(#id) == 1 {
                Box::into_raw(Box::new(<#ty as spatialos_sdk::Type>::type_update_deserialize(
                    user_data,
                    &mut #object_name.get_object(#id),
                )))
            } else {
                std::ptr::null_mut()
            };
        }
    }
}

pub fn get_field_id(attrs: &[Attribute]) -> Option<u32> {
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

pub fn get_ident_type_fields(fields: &Punctuated<Field, Comma>) -> Vec<(&Ident, &Type)> {
    fields
        .iter()
        .map(|field| (field.ident.as_ref(), &field.ty))
        .filter_map(|field| {
            let ident = field.0?;
            Some((ident, field.1))
        })
        .collect()
}

pub fn get_ident_type_id_fields(fields: &Punctuated<Field, Comma>) -> Vec<(&Ident, &Type, u32)> {
    fields
        .iter()
        .map(|field| (field.ident.as_ref(), &field.ty, get_field_id(&field.attrs)))
        .filter_map(|field| {
            let ident = field.0?;
            Some((ident, field.1, field.2))
        })
        .filter_map(|field| {
            let field_id = field.2?;
            Some((field.0, field.1, field_id))
        })
        .collect()
}

pub fn get_data_serializers(
    fields: &Punctuated<Field, Comma>,
    object_name: &Ident,
    data_name: &Ident,
) -> Vec<impl ToTokens> {
    get_ident_type_id_fields(fields)
        .into_iter()
        .map(|(ident, ty, id)| generate_data_serializer(ty, id, ident, object_name, data_name))
        .collect()
}

pub fn get_data_deserializers(
    fields: &Punctuated<Field, Comma>,
    object_name: &Ident,
) -> Vec<impl ToTokens> {
    get_ident_type_id_fields(fields)
        .into_iter()
        .map(|(ident, ty, id)| generate_data_deserializer(ty, id, ident, object_name))
        .collect()
}

pub fn get_update_serializers(
    fields: &Punctuated<Field, Comma>,
    object_name: &Ident,
    data_name: &Ident,
) -> Vec<impl ToTokens> {
    get_ident_type_id_fields(fields)
        .into_iter()
        .map(|(ident, ty, id)| generate_update_serializer(ty, id, ident, object_name, data_name))
        .collect()
}

pub fn get_update_deserializers(
    fields: &Punctuated<Field, Comma>,
    object_name: &Ident,
) -> Vec<impl ToTokens> {
    get_ident_type_id_fields(fields)
        .into_iter()
        .map(|(ident, ty, id)| generate_update_deserializer(ty, id, ident, object_name))
        .collect()
}

pub fn get_constructor<S: AsRef<str>>(fields: &Punctuated<Field, Comma>, name: S) -> impl ToTokens {
    let ty = Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: name
                .as_ref()
                .split("::")
                .map(|seg| PathSegment {
                    ident: format_ident!("{}", seg),
                    arguments: syn::PathArguments::None,
                })
                .fold(Punctuated::new(), |mut acc, val| {
                    acc.push(val);
                    acc
                }),
        },
    });
    let idents = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<_>>();
    quote! {
        #ty { #(#idents,)* }
    }
}

pub fn append_to_end_segment<S: AsRef<str>>(mut ty_path: TypePath, suffix: S) -> TypePath {
    if let Some(mut last) = ty_path.path.segments.last_mut() {
        last.ident = format_ident!("{}{}", last.ident, suffix.as_ref());
    };
    ty_path
}

pub fn transform_non_primitive_data(ty: Type) -> Type {
    if get_non_primitive(&ty).is_some() {
        match ty {
            Type::Path(path) => Type::Path(append_to_end_segment(path, "Data")),
            _ => ty,
        }
    } else {
        ty
    }
}

pub fn transform_non_primitive_update(ty: Type) -> Type {
    if get_non_primitive(&ty).is_some() {
        match ty {
            Type::Path(path) => Type::Ptr(TypePtr {
                star_token: syn::parse_str::<Token![*]>("*").unwrap(),
                const_token: None,
                mutability: syn::parse_str::<Token![mut]>("mut").ok(),
                elem: Box::new(Type::Path(append_to_end_segment(path, "Update"))),
            }),
            _ => ty,
        }
    } else {
        ty
    }
}

pub fn get_copier(new_data: &Ident, data: &Ident, ident: &Ident, ty: &Type) -> impl ToTokens {
    let new_ident = format_ident!("new_{}", ident);
    quote! {
        #new_data.#ident = if !#data.#ident.is_null() {
            let #ident = unsafe { Box::from_raw(data.#ident) };
            let #new_ident = <#ty as spatialos_sdk::Type>::type_update_copy(user_data, &*#ident);
            Box::into_raw(#ident);
            Box::into_raw(Box::new(#new_ident))
        } else {
            std::ptr::null_mut()
        }
    }
}

pub fn get_freeer(data: &Ident, ident: &Ident, ty: &Type) -> impl ToTokens {
    quote! {
        if !#data.#ident.is_null() {
            let #ident = unsafe { Box::from_raw(#data.#ident) };
            <#ty as spatialos_sdk::Type>::type_update_free(user_data, *#ident);
        }
    }
}

pub fn get_copiers(
    fields: &Punctuated<Field, Comma>,
    new_data: &Ident,
    data: &Ident,
) -> Vec<impl ToTokens> {
    get_ident_type_fields(fields)
        .into_iter()
        .filter_map(|f| {
            let ty = get_non_primitive(f.1)?;
            Some((f.0, ty))
        })
        .map(|f| get_copier(new_data, data, f.0, f.1))
        .collect()
}

pub fn get_freeers(fields: &Punctuated<Field, Comma>, new_data: &Ident) -> Vec<impl ToTokens> {
    get_ident_type_fields(fields)
        .into_iter()
        .filter_map(|f| {
            let ty = get_non_primitive(f.1)?;
            Some((f.0, ty))
        })
        .map(|f| get_freeer(new_data, f.0, f.1))
        .collect()
}
