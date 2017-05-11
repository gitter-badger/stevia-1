use bitvec::BitVec;

use ast::{P, Type};
use ast::traits::ExprTrait;
use ast::variants::{Expr, ExprKind};
use ast::iterators::{Childs, ChildsMut, IntoChilds};

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct BitVecConst{pub value: BitVec}

//=============================================================================
// ARITHMETIC EXPRESSIONS
//=============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Neg{
	pub inner: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Add{
	pub terms: Vec<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Mul{
	pub factors: Vec<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Sub{
	pub minuend: P<Expr>,
	pub subtrahend: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Div{
	pub dividend: P<Expr>,
	pub divisor: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Mod{
	pub dividend: P<Expr>,
	pub divisor: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct SignedDiv{
	pub dividend: P<Expr>,
	pub divisor: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct SignedMod{
	pub dividend: P<Expr>,
	pub divisor: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct SignedRem{
	pub dividend: P<Expr>,
	pub divisor: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

//=============================================================================
// BITWISE EXPRESSIONS
//=============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct BitNot{
	pub inner: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct BitAnd{
	pub left: P<Expr>,
	pub right: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct BitOr{
	pub left: P<Expr>,
	pub right: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct BitXor{
	pub left: P<Expr>,
	pub right: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct BitNand{
	pub left: P<Expr>,
	pub right: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct BitNor{
	pub left: P<Expr>,
	pub right: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct BitXnor{
	pub left: P<Expr>,
	pub right: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

//=============================================================================
// ORDER COMPARE EXPRESSIONS
//=============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Lt{
	pub left: P<Expr>,
	pub right: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Le{
	pub left: P<Expr>,
	pub right: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Gt{
	pub left: P<Expr>,
	pub right: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Ge{
	pub left: P<Expr>,
	pub right: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct SignedLt{
	pub left: P<Expr>,
	pub right: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct SignedLe{
	pub left: P<Expr>,
	pub right: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct SignedGt{
	pub left: P<Expr>,
	pub right: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct SignedGe{
	pub left: P<Expr>,
	pub right: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

//=============================================================================
// SHIFT EXPRESSIONS
//=============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Shl{
	pub shifted: P<Expr>,
	pub shift_amount: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Shr{
	pub shifted: P<Expr>,
	pub shift_amount: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct SignedShr{
	pub shifted: P<Expr>,
	pub shift_amount: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

//=============================================================================
// EXTEND & EXTRACT EXPRESSIONS
//=============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Concat{
	pub hi: P<Expr>,
	pub lo: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Extract{
	pub source: P<Expr>,
	pub lo_bit: P<Expr>, pub hi_bit: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Extend{
	pub source: P<Expr>,
	pub extension: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct SignedExtend{
	pub source: P<Expr>,
	pub extension: P<Expr>,
	/// The bitvec type of this expression.
	pub ty: Type
}

//=============================================================================
// ARRAY EXPRESSIONS
//=============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Read{
	pub array: P<Expr>,
	pub index: P<Expr>,
	/// The array type of this expression.
	pub ty: Type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SmtExpr)]
pub struct Write{
	pub array: P<Expr>,
	pub index: P<Expr>,
	pub new_val: P<Expr>,
	/// The array type of this expression.
	pub ty: Type
}
