use syn::{parse::Parse, Attribute, Field, Fields, Ident, ItemEnum, ItemStruct, Type, Variant};

fn get_field_id(attrs: &[Attribute]) -> Option<u32> {
    let attribute = extract_attribute::<syn::Lit>(attrs, "field_id")?;

    if let syn::Lit::Int(lit_int) = attribute {
        lit_int.base10_parse::<u32>().ok()
    } else {
        None
    }
}

fn get_value(attrs: &[Attribute]) -> Option<u32> {
    let attribute = extract_attribute::<syn::Lit>(attrs, "value")?;

    if let syn::Lit::Int(lit_int) = attribute {
        lit_int.base10_parse::<u32>().ok()
    } else {
        None
    }
}

fn get_id(attrs: &[Attribute]) -> Option<u32> {
    let attribute = extract_attribute::<syn::Lit>(attrs, "id")?;

    if let syn::Lit::Int(lit_int) = attribute {
        lit_int.base10_parse::<u32>().ok()
    } else {
        None
    }
}

fn get_spatial_type(attrs: &[Attribute]) -> Option<String> {
    let attribute = extract_attribute::<syn::Lit>(attrs, "spatial_type")?;

    if let syn::Lit::Str(lit_str) = attribute {
        Some(lit_str.value())
    } else {
        None
    }
}

#[derive(Debug)]
pub enum SpatialType {
    Bool,
    Uint32,
    Uint64,
    Int32,
    Int64,
    SInt32,
    SInt64,
    Fixed32,
    Fixed64,
    SFixed32,
    SFixed64,
    Float,
    Double,
    String,
    Bytes,
    EntityID,
    Entity,
    Map(Box<SpatialType>, Box<SpatialType>),
    List(Box<SpatialType>),
    Option(Box<SpatialType>),
    Type,
    Enum,
}

impl From<String> for SpatialType {
    fn from(data: String) -> Self {
        match data.as_str() {
            "bool" => Self::Bool,
            "float" => Self::Float,
            "bytes" => Self::Bytes,
            "int32" => Self::Int32,
            "int64" => Self::Int64,
            "string" => Self::String,
            "double" => Self::Double,
            "uint32" => Self::Uint32,
            "uint64" => Self::Uint64,
            "sint32" => Self::SInt32,
            "sint64" => Self::SInt64,
            "fixed32" => Self::Fixed32,
            "fixed64" => Self::Fixed64,
            "sfixed32" => Self::SFixed32,
            "sfixed64" => Self::SFixed64,
            "EntityId" => Self::EntityID,
            "Entity" => Self::Entity,
            "type" => Self::Type,
            "enum" => Self::Enum,
            _ => panic!("Not supported yet: {}", data.as_str()),
        }
    }
}

#[derive(Debug)]
pub struct FieldAST {
    id: u32,
    name: Ident,
    ty: Type,
    data_type: SpatialType,
}

impl From<&Field> for FieldAST {
    fn from(field: &Field) -> Self {
        let id = get_field_id(&field.attrs).expect("Can't find the field_id");
        let data_type =
            SpatialType::from(get_spatial_type(&field.attrs).expect("Can't find the spatial_type"));
        let name = field
            .ident
            .as_ref()
            .expect("Can't find field ident")
            .clone();
        let ty = field.ty.clone();
        Self {
            id,
            name,
            ty,
            data_type,
        }
    }
}

#[derive(Debug)]
pub struct StructAST<'a> {
    id: Option<u32>,
    name: &'a Ident,
    fields: Vec<FieldAST>,
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

fn extract_attribute<T: Parse>(attrs: &[Attribute], name: &str) -> Option<T> {
    attrs
        .iter()
        .find(|attr| attr.path.is_ident(name))
        .map(|attr| attr.parse_args())?
        .ok()
}

#[derive(Debug)]
pub struct VariantAST {
    pub value: u32,
    pub name: Ident,
}

impl From<&Variant> for VariantAST {
    fn from(input: &Variant) -> Self {
        let name = input.ident.clone();
        let value = get_value(&input.attrs).expect("Can't find value for variant");
        Self { name, value }
    }
}

#[derive(Debug)]
pub struct EnumAST<'a> {
    pub name: &'a Ident,
    pub variants: Vec<VariantAST>,
}

impl<'a> From<&'a ItemEnum> for EnumAST<'a> {
    fn from(input: &'a ItemEnum) -> Self {
        let name = &input.ident;
        let variants = input
            .variants
            .iter()
            .map(VariantAST::from)
            .collect::<Vec<_>>();
        Self { name, variants }
    }
}
