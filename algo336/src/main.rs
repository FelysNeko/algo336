use crate::ast::{Language, Terminal};
use crate::generation::Codegen;

mod ast;
mod builder;
mod generation;

fn main() {
    let language = Language::Union(
        Language::Terminal(Terminal::Set(vec![('a', 'b')]), 0).into(),
        Language::Terminal(Terminal::Set(vec![('b', 'c')]), 0).into(),
    );
    let (transition, accept) = language.annotated().build();

    println!("{:?}", transition);
    println!("{:?}", accept);

    for ((_, terminal), _) in transition {
        println!("{}", terminal.codegen())
    }
}
