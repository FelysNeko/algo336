use crate::ast::Terminal;
use proc_macro2::TokenStream;
use quote::quote;

pub trait Codegen {
    fn codegen(&self) -> TokenStream;
}

impl Codegen for Terminal {
    fn codegen(&self) -> TokenStream {
        match self {
            Terminal::Set(set) => {
                let set = set.iter().map(|(s, e)| quote! {#s..#e});
                quote! {#(#set)|*}
            }
            Terminal::Pound => panic!(),
        }
    }
}
