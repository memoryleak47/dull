use crate::*;

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub enum Semi {
    Class(usize), // this usize is conceptually an `Id`!
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


