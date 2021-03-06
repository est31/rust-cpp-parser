// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::ExprNode;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    ScopeResolution,
    PostInc,
    PostDec,
    Call,
    Parenthesis,
    Dot,
    Arrow,
    Subscript,
    PreInc,
    PreDec,
    Plus,
    Minus,
    Indirection,
    AddressOf,
    AddressOfLabel,
    Sizeof,
    New,
    NewArray,
    Delete,
    DeleteArray,
    CoAwait,
    Not,
    BitNeg,
    DotIndirection,
    ArrowIndirection,
    Mul,
    Div,
    Mod,
    Add,
    Sub,
    LShift,
    RShift,
    ThreeWayComp,
    Lt,
    Gt,
    Leq,
    Geq,
    Eq,
    Neq,
    BitAnd,
    BitXor,
    BitOr,
    And,
    Or,
    Question,
    Colon,
    Throw,
    CoYield,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    LShiftAssign,
    RShiftAssign,
    AndAssign,
    XorAssign,
    OrAssign,
    Comma,
    Cast,
}

impl Operator {
    pub fn operate(self, stack: &mut Vec<ExprNode>) {
        use Operator::*;

        match self {
            Plus | Minus | Not | BitNeg | Sizeof | PreInc | PreDec | Indirection | AddressOf => {
                let arg = stack.pop().unwrap();
                stack.push(ExprNode::UnaryOp(Box::new(UnaryOp { op: self, arg })));
            }
            _ => {
                let arg2 = stack.pop().unwrap();
                let arg1 = stack.pop().unwrap();
                stack.push(ExprNode::BinaryOp(Box::new(BinaryOp {
                    op: self,
                    arg1,
                    arg2,
                })));
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryOp {
    pub op: Operator,
    pub arg1: ExprNode,
    pub arg2: ExprNode,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryOp {
    pub op: Operator,
    pub arg: ExprNode,
}
