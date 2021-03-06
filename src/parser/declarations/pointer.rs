// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::lexer::preprocessor::context::PreprocContext;
use crate::lexer::{Lexer, LocToken, Token};
use crate::parser::attributes::{Attributes, AttributesParser};
use bitflags::bitflags;

use super::super::r#type::CVQualifier;
use super::decl::{NoPtrDeclaratorParser, TypeDeclarator};
use super::specifier::Specifier;
use crate::parser::r#type::{BaseType, Type};

bitflags! {
    pub struct MSModifier: u8 {
        const RESTRICT = 0b1;
        const UPTR = 0b10;
        const SPTR = 0b100;
        const UNALIGNED = 0b1000;
    }
}

impl MSModifier {
    pub(crate) fn from_tok(&mut self, tok: &Token) -> bool {
        match tok {
            Token::MSRestrict => {
                *self |= MSModifier::RESTRICT;
                true
            }
            Token::MSUptr => {
                *self |= MSModifier::UPTR;
                true
            }
            Token::MSSptr => {
                *self |= MSModifier::SPTR;
                true
            }
            Token::MSUnaligned | Token::MS1Unaligned => {
                *self |= MSModifier::UNALIGNED;
                true
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PtrKind {
    Pointer,
    Reference,
    RValue,
}

impl PtrKind {
    pub(crate) fn from_tok(tok: &Token) -> Option<Self> {
        match tok {
            Token::Star => Some(PtrKind::Pointer),
            Token::And => Some(PtrKind::Reference),
            Token::AndAnd => Some(PtrKind::RValue),
            _ => None,
        }
    }

    pub(crate) fn is_ptr(tok: &Token) -> bool {
        match tok {
            Token::Star | Token::And | Token::AndAnd => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pointer {
    pub kind: PtrKind,
    pub attributes: Option<Attributes>,
    pub cv: CVQualifier,
    pub ms: MSModifier,
}

pub type Pointers = Vec<Pointer>;

pub struct PointerDeclaratorParser<'a, 'b, PC: PreprocContext> {
    lexer: &'b mut Lexer<'a, PC>,
}

impl<'a, 'b, PC: PreprocContext> PointerDeclaratorParser<'a, 'b, PC> {
    pub(crate) fn new(lexer: &'b mut Lexer<'a, PC>) -> Self {
        Self { lexer }
    }

    pub(crate) fn parse(
        self,
        tok: Option<LocToken>,
        hint: Option<PtrKind>,
    ) -> (Option<LocToken>, Option<Pointers>) {
        let tok = tok.unwrap_or_else(|| self.lexer.next_useful());
        let mut ptrs = Vec::new();
        let mut kind = if let Some(hint) = hint {
            hint
        } else {
            let kind = PtrKind::from_tok(&tok.tok);
            if let Some(kind) = kind {
                kind
            } else {
                return (Some(tok), None);
            }
        };

        let tok = loop {
            let ap = AttributesParser::new(self.lexer);
            let (tok, attributes) = ap.parse(None);
            let mut tok = tok.unwrap_or_else(|| self.lexer.next_useful());

            let mut cv = CVQualifier::empty();
            let mut ms = MSModifier::empty();

            while cv.from_tok(&tok.tok) || ms.from_tok(&tok.tok) {
                tok = self.lexer.next_useful();
            }

            ptrs.push(Pointer {
                kind,
                attributes,
                cv,
                ms,
            });

            kind = if let Some(kind) = PtrKind::from_tok(&tok.tok) {
                kind
            } else {
                break tok;
            };
        };

        (Some(tok), Some(ptrs))
    }
}

pub struct ParenPointerDeclaratorParser<'a, 'b, PC: PreprocContext> {
    lexer: &'b mut Lexer<'a, PC>,
}

impl<'a, 'b, PC: PreprocContext> ParenPointerDeclaratorParser<'a, 'b, PC> {
    pub(super) fn new(lexer: &'b mut Lexer<'a, PC>) -> Self {
        Self { lexer }
    }

    pub(super) fn parse(
        self,
        tok: Option<LocToken>,
    ) -> (Option<LocToken>, (Option<TypeDeclarator>, bool)) {
        let tok = tok.unwrap_or_else(|| self.lexer.next_useful());
        if tok.tok != Token::LeftParen {
            return (Some(tok), (None, false));
        }

        // The previous token was a parenthesis
        // so we can have some params (function type, e.g. int * (int, int)))
        // or a function/array pointer
        let pdp = PointerDeclaratorParser::new(self.lexer);
        let (tok, pointers) = pdp.parse(None, None);

        if pointers.is_some() {
            let npp = NoPtrDeclaratorParser::new(self.lexer);
            let typ = Type {
                base: BaseType::None,
                cv: CVQualifier::empty(),
                pointers,
            };
            let (tok, decl) = npp.parse(tok, typ, Specifier::empty(), false);

            let tok = tok.unwrap_or_else(|| self.lexer.next_useful());
            if tok.tok != Token::RightParen {
                unreachable!("Invalid token in function pointer: {:?}", tok);
            }

            (None, (decl, false))
        } else {
            // we've function params
            (tok, (None, true))
        }
    }
}
