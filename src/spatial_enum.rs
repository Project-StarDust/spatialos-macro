use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum};

use crate::ast::EnumAST;

pub fn generate_enum(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);

    let ast = EnumAST::from(&input);
    ast.into()
}
