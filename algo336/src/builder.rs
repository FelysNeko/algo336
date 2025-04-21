use crate::ast::{Language, Terminal};
use std::collections::{HashMap, HashSet};

type Graph = HashMap<usize, HashSet<usize>>;
type Transition = HashMap<(usize, (u32, u32)), usize>;

impl Language {
    pub fn annotated(self) -> Self {
        Language::Concat(self.into(), Language::Terminal(Terminal::Pound, 0).into())
    }

    pub fn build(&mut self) -> (Transition, Vec<bool>) {
        let terminals = self.label();
        let follows = self.follow();
        let pound = terminals.len() - 1;

        let mut states = vec![self.first()];
        let mut transition = HashMap::new();

        let mut unmarked = vec![0];
        while let Some(s) = unmarked.pop() {
            let mut symbols = Vec::new();
            for i in &states[s] {
                match &terminals[*i] {
                    Terminal::Set(set) => {
                        symbols.extend(set);
                    }
                    Terminal::Pound => continue,
                }
            }

            let mut boundaries = Vec::with_capacity(symbols.len() * 2 + 2);
            for (start, end) in symbols {
                boundaries.push(start);
                boundaries.push(end.saturating_add(1));
            }
            boundaries.sort();
            boundaries.dedup();

            let ranges = boundaries
                .windows(2)
                .map(|x| (x[0], x[1].saturating_sub(1)))
                .collect::<Vec<_>>();

            for range in ranges {
                let mut next = HashSet::new();
                for &i in &states[s] {
                    match &terminals[i] {
                        Terminal::Set(set) => {
                            if set
                                .iter()
                                .any(|&(start, end)| start <= range.0 && range.1 <= end)
                            {
                                next.extend(follows.get(&i).unwrap_or(&HashSet::new()));
                            }
                        }
                        Terminal::Pound => continue,
                    }
                }

                if let Some(id) = states.iter().position(|x| x == &next) {
                    transition.insert((s, range), id);
                } else {
                    let id = states.len();
                    states.push(next);
                    unmarked.push(id);
                    transition.insert((s, range), id);
                }
            }
        }

        let accept = states.iter().map(|x| x.contains(&pound)).collect();
        (transition, accept)
    }

    fn label(&mut self) -> Vec<Terminal> {
        let mut terminals = Vec::new();
        let mut todo = vec![self];
        while let Some(language) = todo.pop() {
            match language {
                Language::Union(c1, c2) => {
                    todo.push(c2);
                    todo.push(c1);
                }
                Language::Concat(c1, c2) => {
                    todo.push(c2);
                    todo.push(c1);
                }
                Language::Kleene(c) => todo.push(c),
                Language::Nested(c) => todo.push(c),
                Language::Terminal(terminal, i) => {
                    *i = terminals.len();
                    terminals.push(terminal.clone());
                }
            }
        }
        if let Terminal::Pound = terminals.last().unwrap() {
            terminals
        } else {
            panic!()
        }
    }

    fn nullable(&self) -> bool {
        match self {
            Language::Union(c1, c2) => c1.nullable() || c2.nullable(),
            Language::Concat(c1, c2) => c1.nullable() && c2.nullable(),
            Language::Kleene(_) => true,
            Language::Nested(c) => c.nullable(),
            Language::Terminal(_, _) => false,
        }
    }

    fn first(&self) -> HashSet<usize> {
        match self {
            Language::Union(c1, c2) => &c1.first() | &c2.first(),
            Language::Concat(c1, c2) => {
                if c1.nullable() {
                    &c1.first() | &c2.first()
                } else {
                    c1.first()
                }
            }
            Language::Kleene(c) => c.first(),
            Language::Nested(c) => c.first(),
            Language::Terminal(_, i) => HashSet::from([*i]),
        }
    }

    fn last(&self) -> HashSet<usize> {
        match self {
            Language::Union(c1, c2) => &c1.last() | &c2.last(),
            Language::Concat(c1, c2) => {
                if c2.nullable() {
                    &c1.last() | &c2.last()
                } else {
                    c2.last()
                }
            }
            Language::Kleene(c) => c.last(),
            Language::Nested(c) => c.last(),
            Language::Terminal(_, i) => HashSet::from([*i]),
        }
    }

    fn follow(&self) -> Graph {
        let mut follows = Graph::new();
        let mut todo = vec![self];
        while let Some(language) = todo.pop() {
            match language {
                Language::Union(c1, c2) => {
                    todo.push(c1);
                    todo.push(c2);
                }
                Language::Concat(c1, c2) => {
                    for i in c1.last() {
                        follows.entry(i).or_default().extend(c2.first());
                    }
                    todo.push(c1);
                    todo.push(c2);
                }
                Language::Kleene(c) => {
                    for i in c.last() {
                        follows.entry(i).or_default().extend(c.first());
                    }
                    todo.push(c);
                }
                Language::Nested(c) => todo.push(c),
                Language::Terminal(_, i) => {
                    follows.entry(*i).or_default();
                }
            }
        }
        follows
    }
}
