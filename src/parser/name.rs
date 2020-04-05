use crate::lexer::preprocessor::context::PreprocContext;
use crate::lexer::{Lexer, LocToken, Token};
use crate::parser::expression::{Parameters, ParametersParser};

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub(crate) val: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Template {
    val: String,
    params: Parameters,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Name {
    Identifier(Identifier),
    Template(Template),
}

#[macro_export]
macro_rules! mk_id {
    ( $( $name:expr ),* ) => {
        Qualified {
            names: vec![
                $(
                    Name::Identifier(Identifier { val: $name.to_string()}),
                )*
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Qualified {
    pub(crate) names: Vec<Name>,
}

pub struct QualifiedParser<'a, 'b, PC: PreprocContext> {
    lexer: &'b mut Lexer<'a, PC>,
}

impl<'a, 'b, PC: PreprocContext> QualifiedParser<'a, 'b, PC> {
    pub(super) fn new(lexer: &'b mut Lexer<'a, PC>) -> Self {
        Self { lexer }
    }

    pub(super) fn parse(
        self,
        tok: Option<LocToken<'a>>,
    ) -> (Option<LocToken<'a>>, Option<Qualified>) {
        let mut tok = tok.unwrap_or_else(|| self.lexer.next_useful());
        let mut names = Vec::new();

        loop {
            match tok.tok {
                Token::ColonColon => {}
                Token::Lower => {
                    let name = if let Some(Name::Identifier(id)) = names.pop() {
                        id.val
                    } else {
                        unreachable!("Cannot have two templates args");
                    };

                    let pp = ParametersParser::new(self.lexer, Token::Greater);
                    let (_, params) = pp.parse(None);

                    names.push(Name::Template(Template {
                        val: name,
                        params: params.unwrap(),
                    }));
                }
                Token::Identifier(id) => {
                    names.push(Name::Identifier(Identifier {
                        val: id.to_string(),
                    }));
                }
                _ => {
                    return (Some(tok), Some(Qualified { names }));
                }
            }
            tok = self.lexer.next_useful();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexer::preprocessor::context::DefaultContext;
    use crate::parser::ast::*;

    #[test]
    fn test_name_one() {
        let mut l = Lexer::<DefaultContext>::new(b"abc");
        let p = QualifiedParser::new(&mut l);
        let (_, q) = p.parse(None);

        assert_eq!(q.unwrap(), mk_id!("abc"));
    }

    #[test]
    fn test_name_two() {
        let mut l = Lexer::<DefaultContext>::new(b"abc::defg");
        let p = QualifiedParser::new(&mut l);
        let (_, q) = p.parse(None);

        assert_eq!(q.unwrap(), mk_id!("abc", "defg"));
    }

    #[test]
    fn test_name_three() {
        let mut l = Lexer::<DefaultContext>::new(b"abc::defg::hijkl");
        let p = QualifiedParser::new(&mut l);
        let (_, q) = p.parse(None);

        assert_eq!(q.unwrap(), mk_id!("abc", "defg", "hijkl"));
    }

    #[test]
    fn test_name_template_zero() {
        let mut l = Lexer::<DefaultContext>::new(b"A<>");
        let p = QualifiedParser::new(&mut l);
        let (_, q) = p.parse(None);

        assert_eq!(
            q.unwrap(),
            Qualified {
                names: vec![Name::Template(Template {
                    val: "A".to_string(),
                    params: vec![],
                }),],
            }
        );
    }

    #[test]
    fn test_name_template_one() {
        let mut l = Lexer::<DefaultContext>::new(b"A<B>");
        let p = QualifiedParser::new(&mut l);
        let (_, q) = p.parse(None);

        assert_eq!(
            q.unwrap(),
            Qualified {
                names: vec![Name::Template(Template {
                    val: "A".to_string(),
                    params: vec![Some(Node::Qualified(Box::new(mk_id!("B")))),],
                }),],
            }
        );
    }

    #[test]
    fn test_name_complex() {
        let mut l = Lexer::<DefaultContext>::new(b"A::B<C::D, E::F, G>::H<I>");
        let p = QualifiedParser::new(&mut l);
        let (_, q) = p.parse(None);

        assert_eq!(
            q.unwrap(),
            Qualified {
                names: vec![
                    Name::Identifier(Identifier {
                        val: "A".to_string(),
                    }),
                    Name::Template(Template {
                        val: "B".to_string(),
                        params: vec![
                            Some(Node::Qualified(Box::new(mk_id!("C", "D")))),
                            Some(Node::Qualified(Box::new(mk_id!("E", "F")))),
                            Some(Node::Qualified(Box::new(mk_id!("G")))),
                        ]
                    }),
                    Name::Template(Template {
                        val: "H".to_string(),
                        params: vec![Some(Node::Qualified(Box::new(mk_id!("I")))),]
                    })
                ]
            }
        );
    }
}
