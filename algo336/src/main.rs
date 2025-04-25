use crate::ast::{Language, Terminal};

mod ast;
mod builder;

fn main() {
    let language = Language::Concat(
        Language::Terminal(
            Terminal::Set(vec![
                ('a' as u32, 'z' as u32),
                ('A' as u32, 'Z' as u32),
                ('_' as u32, '_' as u32),
            ]),
            0,
        )
        .into(),
        Language::Kleene(
            Language::Terminal(
                Terminal::Set(vec![
                    ('a' as u32, 'z' as u32),
                    ('A' as u32, 'Z' as u32),
                    ('_' as u32, '_' as u32),
                    ('0' as u32, '9' as u32),
                ]),
                0,
            )
            .into(),
        ).into(),
    );

    let (transition, accept) = language.annotated().build();
    for ((s0, (s, e)), s1) in transition {
        println!("{s0}: [{s}, {e}] -> {s1}");
    }

    println!("{:?}", accept);
}
