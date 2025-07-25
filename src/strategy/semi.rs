use crate::*;

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub enum Semi {
    Class(usize),
    L(GeneralLang),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Discr {
    L(<GeneralLang as Language>::Discriminant),
    Class,
}

impl Language for Semi {
    type Discriminant = Discr;

    #[inline(always)]
    fn discriminant(&self) -> Self::Discriminant {
        match self {
            Semi::L(n) => Discr::L(n.discriminant()),
            Semi::Class(_) => Discr::Class,
        }
    }

    fn matches(&self, _other: &Self) -> bool {
        panic!("Should never call this")
    }

    fn children(&self) -> &[Id] {
        match self {
            Semi::L(n) => n.children(),
            Semi::Class(_) => &[],
        }
    }

    fn children_mut(&mut self) -> &mut [Id] {
        match self {
            Semi::L(n) => n.children_mut(),
            Semi::Class(_) => &mut [],
        }
    }
}

pub fn add_semi(semi: RecExpr<Semi>, eg: &mut EGraph<GeneralLang, ()>) -> Id {
    // maps "semi indices" to "e-graph indices".
    let mut ids: Vec<Id> = Vec::new();

    for i in 0..semi.len() {
        let new_id = match &semi[i.into()] {
            Semi::L(GeneralLang::Constant(s)) => eg.add(GeneralLang::Constant(*s)),
            Semi::L(GeneralLang::App(l)) => {
                let b = l.iter().map(|x| {
                    let y: usize = (*x).into();
                    ids[y]
                }).collect::<Box<[Id]>>();
                eg.add(GeneralLang::App(b))
            },
            Semi::Class(c) => (*c).into(),
        };
        ids.push(new_id);
    }
    *ids.last().unwrap()
}

