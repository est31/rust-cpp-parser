// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::{Statement, StatementParser};
use crate::lexer::lexer::{Lexer, LocToken, Token};
use crate::lexer::preprocessor::context::PreprocContext;
use crate::parser::attributes::Attributes;
use crate::parser::declarations::{TypeDeclarator, TypeDeclaratorParser};

#[derive(Clone, Debug, PartialEq)]
pub struct Try {
    pub attributes: Option<Attributes>,
    pub body: Box<Statement>,
    pub clause: Option<TypeDeclarator>,
    pub handler: Box<Statement>,
}

pub struct TryStmtParser<'a, 'b, PC: PreprocContext> {
    lexer: &'b mut Lexer<'a, PC>,
}

impl<'a, 'b, PC: PreprocContext> TryStmtParser<'a, 'b, PC> {
    pub(super) fn new(lexer: &'b mut Lexer<'a, PC>) -> Self {
        Self { lexer }
    }

    pub(super) fn parse(self, attributes: Option<Attributes>) -> (Option<LocToken>, Option<Try>) {
        let sp = StatementParser::new(self.lexer);
        let (tok, body) = sp.parse(None);

        let body = if let Some(body) = body {
            body
        } else {
            unreachable!("Invalid token in try: {:?}", tok);
        };

        let tok = tok.unwrap_or_else(|| self.lexer.next_useful());
        if tok.tok != Token::Catch {
            unreachable!("Catch expected after body in try statement");
        }

        let tok = self.lexer.next_useful();
        if tok.tok != Token::LeftParen {
            unreachable!("Invalid token in catch clause: {:?}", tok);
        }

        let tok = self.lexer.next_useful();
        let (tok, clause) = if tok.tok == Token::Ellipsis {
            (None, None)
        } else {
            let tp = TypeDeclaratorParser::new(self.lexer);
            let (tok, typ) = tp.parse(Some(tok), None);

            if typ.is_some() {
                (tok, typ)
            } else {
                unreachable!("Invalid token in catch clause: {:?}", tok);
            }
        };

        let tok = tok.unwrap_or_else(|| self.lexer.next_useful());
        if tok.tok != Token::RightParen {
            unreachable!("Invalid token in catch clause: {:?}", tok);
        }

        let sp = StatementParser::new(self.lexer);
        let (tok, handler) = sp.parse(None);

        let handler = if let Some(handler) = handler {
            handler
        } else {
            unreachable!("Invalid token in try handler: {:?}", tok);
        };

        (
            tok,
            Some(Try {
                attributes,
                body: Box::new(body),
                clause,
                handler: Box::new(handler),
            }),
        )
    }
}
