use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

use crate::ast::StructAST;

pub fn generate_type(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let ast = StructAST::from(&input);

    if ast.is_component() {
        panic!("Custom types should not have an ID");
    }

    ast.into()
}
