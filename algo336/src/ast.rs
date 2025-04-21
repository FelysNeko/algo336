#[derive(Debug)]
pub enum Language {
    Union(Box<Language>, Box<Language>),
    Concat(Box<Language>, Box<Language>),
    Kleene(Box<Language>),
    Nested(Box<Language>),
    Terminal(Terminal, usize),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Terminal {
    Set(Vec<(u32, u32)>),
    Pound,
}
