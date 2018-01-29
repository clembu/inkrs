use de::path::Path;

#[derive(PartialEq, Debug)]
pub(crate) enum Target {
    Var(String),
    Path(Path),
}

#[derive(PartialEq, Debug)]
pub(crate) enum Divert {
    Goto(Target, bool),
    Fn(Target, bool),
    Tunnel(Target, bool),
    XFn(String, usize, bool),
}

struct DivertVisitor;
use serde::de::{self, Visitor};

use std::fmt;
impl<'de> Visitor<'de> for DivertVisitor {
    type Value = Divert;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a divert map")
    }

    fn visit_map<M>(self, mut m: M) -> Result<Divert, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        enum Type {
            Divert(String),
            Func(String),
            Tunnel(String),
            Xfunc(String),
        };

        let mut ty: Option<Type> = None;
        let mut var: bool = false;
        let mut cond: bool = false;
        let mut xargs: Option<usize> = None;
        while let Some(k) = m.next_key::<String>()? {
            match &k {
                s if s == "var" => {
                    let v: bool = m.next_value()?;
                    var = v;
                }
                s if s == "c" => {
                    let c: bool = m.next_value()?;
                    cond = c;
                }
                s if s == "->" => {
                    let trgstr: String = m.next_value()?;
                    ty = Some(Type::Divert(trgstr));
                }
                s if s == "f()" => {
                    let trgstr: String = m.next_value()?;
                    ty = Some(Type::Func(trgstr));
                }
                s if s == "->t->" => {
                    let trgstr: String = m.next_value()?;
                    ty = Some(Type::Tunnel(trgstr));
                }
                s if s == "x()" => {
                    let trgstr: String = m.next_value()?;
                    ty = Some(Type::Xfunc(trgstr));
                }
                s if s == "exArgs" => {
                    let exargs: usize = m.next_value()?;
                    xargs = Some(exargs);
                }
                _ => {
                    break;
                }
            }
        }

        use serde::de::Error;
        match ty {
            None => Err(M::Error::custom("couldn't find a divert target")),
            Some(t) => match t {
                Type::Divert(tgstr) => Ok(Divert::Goto(
                    if var {
                        Target::Var(tgstr)
                    } else {
                        Target::Path(Path::from(tgstr))
                    },
                    cond,
                )),
                Type::Func(tgstr) => Ok(Divert::Fn(
                    if var {
                        Target::Var(tgstr)
                    } else {
                        Target::Path(Path::from(tgstr))
                    },
                    cond,
                )),
                Type::Tunnel(tgstr) => Ok(Divert::Tunnel(
                    if var {
                        Target::Var(tgstr)
                    } else {
                        Target::Path(Path::from(tgstr))
                    },
                    cond,
                )),
                Type::Xfunc(tgstr) => Ok(Divert::XFn(tgstr, xargs.unwrap_or_default(), cond)),
            },
        }
    }
}

use serde::Deserialize;
impl<'de> Deserialize<'de> for Divert {
    fn deserialize<D>(d: D) -> Result<Divert, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_any(DivertVisitor)
    }
}

// TODO: more tests pls
#[cfg(test)]
mod tests_serde {
    use super::*;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn de_simple_divert() {
        assert_de_tokens(
            &Divert::Goto(Target::Path(Path::from("some.path")), false),
            &[
                Token::Map { len: Some(1) },
                Token::Str("->"),
                Token::Str("some.path"),
                Token::MapEnd,
            ],
        )
    }

    #[test]
    fn de_conditional_divert() {
        assert_de_tokens(
            &Divert::Goto(Target::Path(Path::from("some.path")), true),
            &[
                Token::Map { len: Some(2) },
                Token::Str("->"),
                Token::Str("some.path"),
                Token::Str("c"),
                Token::Bool(true),
                Token::MapEnd,
            ],
        )
    }

    #[test]
    fn de_variable_divert() {
        assert_de_tokens(
            &Divert::Goto(Target::Var("strucker".to_string()), false),
            &[
                Token::Map { len: Some(2) },
                Token::Str("->"),
                Token::Str("strucker"),
                Token::Str("var"),
                Token::Bool(true),
                Token::MapEnd,
            ],
        )
    }

    #[test]
    fn de_conditional_variable_divert() {
        assert_de_tokens(
            &Divert::Goto(Target::Var("strucker".to_string()), true),
            &[
                Token::Map { len: Some(3) },
                Token::Str("->"),
                Token::Str("strucker"),
                Token::Str("var"),
                Token::Bool(true),
                Token::Str("c"),
                Token::Bool(true),
                Token::MapEnd,
            ],
        )
    }

    #[test]
    fn de_simple_tunnel() {
        assert_de_tokens(
            &Divert::Tunnel(Target::Path(Path::from("some.path")), false),
            &[
                Token::Map { len: Some(1) },
                Token::Str("->t->"),
                Token::Str("some.path"),
                Token::MapEnd,
            ],
        )
    }

    #[test]
    fn de_simple_function() {
        assert_de_tokens(
            &Divert::Fn(Target::Path(Path::from("some.path")), false),
            &[
                Token::Map { len: Some(1) },
                Token::Str("f()"),
                Token::Str("some.path"),
                Token::MapEnd,
            ],
        )
    }

    #[test]
    fn de_simple_ext_function() {
        assert_de_tokens(
            &Divert::XFn("exfun".to_string(), 3, false),
            &[
                Token::Map { len: Some(2) },
                Token::Str("x()"),
                Token::Str("exfun"),
                Token::Str("exArgs"),
                Token::U64(3),
                Token::MapEnd,
            ],
        )
    }
}
