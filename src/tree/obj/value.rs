use tree::path::Path;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub(crate) enum FlagSet {
    Empty(Vec<String>),
    Flags(HashMap<(String, String), i64>),
}

impl FlagSet {
    pub(crate) fn from_origins(org: Vec<String>) -> FlagSet {
        FlagSet::Empty(org)
    }
    pub(crate) fn from_items(list: HashMap<String, i64>) -> FlagSet {
        FlagSet::Flags(
            list.iter()
                .map(|(s, i)| {
                    let mut s = s.splitn(2, ".");
                    (
                        (s.next().unwrap().to_string(), s.next().unwrap().to_string()),
                        *i,
                    )
                })
                .collect::<HashMap<(String, String), i64>>(),
        )
    }
}

#[derive(PartialEq, Debug)]
pub(crate) enum Value {
    Void,
    Int(i64),
    Float(f64),
    String(String),
    DivertTarget(Path),
    VarPtr(String, Option<usize>),
    FlagSet(FlagSet),
}

struct ValueVisitor;
use serde::de::{self, Visitor};
use serde::ser;

use std::fmt;
impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("either an int, a float, a string begining with `^`, or heck")
    }

    fn visit_map<M>(self, mut m: M) -> Result<Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        use serde::de::Error;
        let mut r: Result<Value, M::Error> = Err(M::Error::custom("unimplemented"));
        let mut var: Option<String> = None;
        if let Some(k) = m.next_key::<String>()? {
            match &k {
                s if s == "^->" => {
                    r = Ok(Value::DivertTarget(Path::from(m.next_value::<String>()?)))
                }
                s if s == "^var" => {
                    let v: String = m.next_value()?;
                    if let Some(kci) = m.next_key::<String>()? {
                        if &kci == "ci" {
                            r = Ok(Value::VarPtr(v, Some(m.next_value()?)));
                        }
                    } else {
                        r = Ok(Value::VarPtr(v, None));
                    }
                }
                s if s == "ci" => {
                    let ci: usize = m.next_value()?;
                    if let Some(kvar) = m.next_key::<String>()? {
                        if &kvar == "^var" {
                            r = Ok(Value::VarPtr(m.next_value()?, Some(ci)));
                        }
                    }
                }
                s if s == "list" => {
                    let list: HashMap<String, i64> = m.next_value()?;
                    if let Some(kor) = m.next_key::<String>()? {
                        if &kor == "origins" {
                            r = Ok(Value::FlagSet(FlagSet::from_origins(m.next_value()?)));
                        }
                    } else {
                        r = Ok(Value::FlagSet(FlagSet::from_items(list)));
                    }
                }
                s if s == "origins" => {
                    r = Ok(Value::FlagSet(FlagSet::from_origins(m.next_value()?)));
                }
                _ => (),
            }
        }
        r
    }

    fn visit_str<E>(self, v: &str) -> Result<Value, E>
    where
        E: de::Error,
    {
        match v {
            "void" => Ok(Value::Void),
            "\n" => Ok(Value::String("\n".to_string())),
            s if s.starts_with("^") => Ok(Value::String(s[1..].to_string())),
            _ => Err(E::custom(format!("This string does not represent a value"))),
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Value, E>
    where
        E: de::Error,
    {
        use std::i64;
        if v <= i64::MAX as u64 {
            Ok(Value::Int(v as i64))
        } else {
            Err(E::custom(format!("u64 out of range: {}", v)))
        }
    }

    fn visit_i64<E>(self, v: i64) -> Result<Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Int(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Float(v))
    }
}

use serde::Deserialize;
impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(d: D) -> Result<Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_any(ValueVisitor)
    }
}

#[cfg(test)]
mod tests_serde {
    use super::*;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn de_void() {
        assert_de_tokens(&Value::Void, &[Token::Str("void")]);
    }

    #[test]
    fn de_numbers() {
        assert_de_tokens(&Value::Int(3), &[Token::I64(3)]);

        assert_de_tokens(&Value::Float(2.4), &[Token::F64(2.4)]);
    }

    #[test]
    fn de_strings() {
        assert_de_tokens(&Value::String("wow".to_string()), &[Token::Str("^wow")]);
        assert_de_tokens(&Value::String("^wow".to_string()), &[Token::Str("^^wow")]);
        assert_de_tokens(&Value::String("\n".to_string()), &[Token::Str("\n")]);
    }

    #[test]

    fn de_diverts() {
        assert_de_tokens(
            &Value::DivertTarget(Path::from("some.path")),
            &[
                Token::Map { len: Some(1) },
                Token::Str("^->"),
                Token::Str("some.path"),
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn de_variables() {
        assert_de_tokens(
            &Value::VarPtr("foo".to_string(), None),
            &[
                Token::Map { len: Some(1) },
                Token::Str("^var"),
                Token::Str("foo"),
                Token::MapEnd,
            ],
        );

        assert_de_tokens(
            &Value::VarPtr("foo".to_string(), Some(2)),
            &[
                Token::Map { len: Some(2) },
                Token::Str("^var"),
                Token::Str("foo"),
                Token::Str("ci"),
                Token::U64(2),
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn de_list_empty() {
        assert_de_tokens(
            &Value::FlagSet(FlagSet::from_origins(vec![
                "Doctors".to_string(),
                "Nurses".to_string(),
            ])),
            &[
                Token::Map { len: Some(2) },
                Token::Str("list"),
                Token::Map { len: Some(0) },
                Token::MapEnd,
                Token::Str("origins"),
                Token::Seq { len: Some(2) },
                Token::Str("Doctors"),
                Token::Str("Nurses"),
                Token::SeqEnd,
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn de_list_flags() {
        assert_de_tokens(
            &Value::FlagSet(FlagSet::from_items(
                vec![
                    ("Doctors.Henry".to_string(), 1),
                    ("Doctors.Jenkins".to_string(), 2),
                ].into_iter()
                    .collect::<HashMap<String, i64>>(),
            )),
            &[
                Token::Map { len: Some(1) },
                Token::Str("list"),
                Token::Map { len: Some(2) },
                Token::Str("Doctors.Henry"),
                Token::U64(1),
                Token::Str("Doctors.Jenkins"),
                Token::U64(2),
                Token::MapEnd,
                Token::MapEnd,
            ],
        )
    }
}
