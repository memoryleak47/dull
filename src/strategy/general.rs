use crate::*;

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub struct GeneralLang {
    pub f: Symbol,
    pub children: Box<[Id]>,
}

impl Language for GeneralLang {
    type Discriminant = (Symbol, usize);

    #[inline(always)]
    fn discriminant(&self) -> Self::Discriminant {
        (self.f, self.children.len())
    }

    fn matches(&self, other: &Self) -> bool {
        self.f == other.f
        && self.children.len() == other.children.len()
    }

    fn children(&self) -> &[Id] {
        &*self.children
    }

    fn children_mut(&mut self) -> &mut [Id] {
        &mut *self.children
    }
}
