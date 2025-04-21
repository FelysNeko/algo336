use crate::ast::{Language, Terminal};

mod ast;
mod builder;

fn main() {
    let language = Language::Union(
        Language::Terminal(Terminal::Set(vec![('a' as u32, 'a' as u32)]), 0).into(),
        Language::Terminal(Terminal::Set(vec![('b' as u32, 'b' as u32)]), 0).into(),
    );
    let (transition, accept) = language.annotated().build();

    println!("{:?}", transition);
    println!("{:?}", accept);
}
