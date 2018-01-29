#[derive(Debug, PartialEq)]
pub(crate) struct VarAssign {
    var: String,
    re: bool,
    gl: bool,
}
struct AssignVisitor;
use serde::de::{self, Visitor};

use std::fmt;
impl<'de> Visitor<'de> for AssignVisitor {
    type Value = VarAssign;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a varassign map")
    }

    fn visit_map<M>(self, mut m: M) -> Result<VarAssign, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        use serde::de::Error;
        let mut re = false;
        let mut gl = false;
        let mut var: Option<String> = None;
        while let Some(k) = m.next_key::<String>()? {
            match &k {
                s if s == "VAR=" => {
                    let s: String = m.next_value()?;
                    var = Some(s);
                    gl = true;
                }
                s if s == "temp=" => {
                    let s: String = m.next_value()?;
                    var = Some(s);
                    gl = false;
                }
                s if s == "re" => {
                    re = m.next_value()?;
                }
                _ => {
                    break;
                }
            }
        }
        if let Some(var) = var {
            Ok(VarAssign { var, re, gl })
        } else {
            Err(M::Error::custom("bad varassign map format"))
        }
    }
}

use serde::Deserialize;
impl<'de> Deserialize<'de> for VarAssign {
    fn deserialize<D>(d: D) -> Result<VarAssign, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_any(AssignVisitor)
    }
}

#[cfg(test)]
mod tests_serde {
    use super::*;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn de_new_glob() {
        assert_de_tokens(
            &VarAssign {
                var: "color".to_string(),
                gl: true,
                re: false,
            },
            &[
                Token::Map { len: Some(1) },
                Token::Str("VAR="),
                Token::Str("color"),
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn de_new_tmp() {
        assert_de_tokens(
            &VarAssign {
                var: "color".to_string(),
                gl: false,
                re: false,
            },
            &[
                Token::Map { len: Some(1) },
                Token::Str("temp="),
                Token::Str("color"),
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn de_re_glob() {
        assert_de_tokens(
            &VarAssign {
                var: "color".to_string(),
                gl: true,
                re: true,
            },
            &[
                Token::Map { len: Some(2) },
                Token::Str("VAR="),
                Token::Str("color"),
                Token::Str("re"),
                Token::Bool(true),
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn de_re_tmp() {
        assert_de_tokens(
            &VarAssign {
                var: "color".to_string(),
                gl: false,
                re: true,
            },
            &[
                Token::Map { len: Some(2) },
                Token::Str("temp="),
                Token::Str("color"),
                Token::Str("re"),
                Token::Bool(true),
                Token::MapEnd,
            ],
        );
    }
}
