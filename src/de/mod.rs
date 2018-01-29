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

mod obj;
mod container;
mod path;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct Story {
    root: Container,
    #[serde(rename = "inkVersion")] v: u32,
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
