use crate::ast::{Language, Terminal};

mod ast;
mod builder;

fn main() {
    let language = Language::Union(
        Language::Terminal(Terminal::Set(vec![('a' as u32, 'c' as u32)]), 0).into(),
        Language::Terminal(Terminal::Set(vec![('b' as u32, 'd' as u32)]), 0).into(),
    );
    let (transition, accept) = language.annotated().build();
    for ((s0, (s, e)), s1) in transition {
        println!("{s0}: [{s}, {e}] -> {s1}");
    }
    
    println!("{:?}", accept);
}
