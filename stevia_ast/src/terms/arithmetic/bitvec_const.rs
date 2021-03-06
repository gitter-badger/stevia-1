use crate::prelude::*;

pub mod prelude {
    pub use super::BitvecConst;
}

/// A constant bitvec term expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitvecConst {
    /// The constant bitvec value.
    pub val: Bitvec,
}

impl BitvecConst {
    /// Creates a new `BitvecConst` for the given bit width with a value of zero.
    pub fn zero(ty: BitvecTy) -> BitvecConst {
        BitvecConst::from(Bitvec::zero(ty.width()))
    }

    /// Creates a new `BitvecConst` for the given bit width with a value of one.
    pub fn one(ty: BitvecTy) -> BitvecConst {
        BitvecConst::from(Bitvec::one(ty.width()))
    }

    /// Creates a new `BitvecConst` for the given bit width with a value that has all bits set.
    pub fn all_set(ty: BitvecTy) -> BitvecConst {
        BitvecConst::from(Bitvec::all_set(ty.width()))
    }
}

impl<T> From<T> for BitvecConst
where
    T: Into<Bitvec>
{
    fn from(val: T) -> Self {
        BitvecConst{ val: val.into() }
    }
}

impl Children for BitvecConst {
    fn children(&self) -> ChildrenIter {
        ChildrenIter::none()
    }
}

impl ChildrenMut for BitvecConst {
    fn children_mut(&mut self) -> ChildrenIterMut {
        ChildrenIterMut::none()
    }
}

impl IntoChildren for BitvecConst {
    fn into_children(self) -> IntoChildrenIter {
        IntoChildrenIter::none()
    }
}

impl HasType for BitvecConst {
    fn ty(&self) -> Type {
        self.val.ty()
    }
}

impl HasKind for BitvecConst {
    fn kind(&self) -> ExprKind {
        ExprKind::BitvecConst
    }
}

impl HasArity for BitvecConst {
    fn arity(&self) -> usize {
        0
    }
}

impl From<BitvecConst> for AnyExpr {
    fn from(bitvec_const: BitvecConst) -> AnyExpr {
        AnyExpr::BitvecConst(bitvec_const)
    }
}
