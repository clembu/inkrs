#[cfg(test)]
macro_rules! test_unit_variant_de {
    ($enum:ident, $var:ident, $str: expr) => {
        assert_de_tokens(
            &$enum::$var,
            &[
                Token::UnitVariant{
                    name: stringify!($enum),
                    variant: $str,
                },
            ]
        )
    }
}

const INK_VERSION: u32 = 17;
const INK_MIN_SUPPORTED: u32 = 16;

mod obj;
mod container;
mod path;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct Story {
    root: Container,
    #[serde(rename = "inkVersion")]
    #[serde(deserialize_with = "de_ink_version")]
    v: u32,
    #[serde(rename = "listDefs")] flagsets: HashMap<String, HashMap<String, u32>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Container {
    c: Vec<Tree>,
    named_c: HashMap<String, Container>,
    flags: u8,
    name: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub(crate) enum Tree {
    Node(Container),
    Leaf(obj::Obj),
}

use serde::{Deserialize, Deserializer};
use serde::de::Error;
fn de_ink_version<'de, D>(d: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let v = u32::deserialize(d)?;
    if v > INK_VERSION {
        Err(D::Error::custom(
            "Story file was built for a newer version of Ink.",
        ))
    } else if v < INK_MIN_SUPPORTED {
        Err(D::Error::custom(
            "Story file was built for a much older version of Ink.",
        ))
    } else {
        Ok(v)
    }
}

#[cfg(test)]
mod tests_serde {
    use super::*;
    use std::collections::HashMap;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn de_story_structure() {
        use super::obj::*;
        let st = Story {
            v: 17,
            flagsets: HashMap::new(),
            root: Container::with_flags(
                vec![
                    Tree::Leaf(Obj::Value(value::Value::String(
                        "Hello, world.".to_string(),
                    ))),
                    Tree::Leaf(Obj::Value(value::Value::String("\n".to_string()))),
                    Tree::Leaf(Obj::Cmd(cmd::Cmd::Done)),
                ],
                0x03,
            ),
        };

        assert_de_tokens(
            &st,
            &[
                Token::Map { len: None },
                Token::Str("inkVersion"),
                Token::U32(17),
                Token::Str("root"),
                Token::Seq { len: None },
                Token::Str("^Hello, world."),
                Token::Str("\n"),
                Token::Str("done"),
                Token::Map { len: None },
                Token::Str("#f"),
                Token::U8(3),
                Token::MapEnd,
                Token::SeqEnd,
                Token::Str("listDefs"),
                Token::Map { len: None },
                Token::MapEnd,
                Token::MapEnd,
            ],
        )
    }

}
