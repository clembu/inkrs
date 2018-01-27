pub(crate) mod container;
pub(crate) mod obj;
pub(crate) mod path;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub(crate) enum Tree {
    Node(container::Container),
    Leaf(obj::Obj),
}

#[cfg(test)]
mod tests_serde {
    use super::*;
    use std::collections::HashMap;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn de_tree_cmd() {
        assert_de_tokens(
            &Tree::Leaf(obj::Obj::Cmd(obj::cmd::Cmd::End)),
            &[Token::Str("end")],
        );
    }

    #[test]
    fn de_tree_simple_cont() {
        assert_de_tokens(
            &Tree::Node(container::Container {
                name: None,
                flags: 0,
                named_c: HashMap::new(),
                c: vec![Tree::Leaf(obj::Obj::Cmd(obj::cmd::Cmd::End))],
            }),
            &[
                Token::Seq { len: Some(2) },
                Token::Str("end"),
                Token::None,
                Token::SeqEnd,
            ],
        )
    }

    #[test]
    fn de_tree_nested_cont() {
        let subc: container::Container = container::Container {
            name: Some(String::from("c")),
            flags: 0x03,
            c: vec![Tree::Leaf(obj::Obj::Cmd(obj::cmd::Cmd::End))],
            named_c: HashMap::new(),
        };
        let mut nc: HashMap<String, container::Container> = HashMap::new();
        nc.insert(String::from("c"), subc);
        assert_de_tokens(
            &Tree::Node(container::Container {
                name: None,
                flags: 0,
                c: vec![Tree::Leaf(obj::Obj::Cmd(obj::cmd::Cmd::End))],
                named_c: nc,
            }),
            &[
                Token::Seq { len: Some(2) },
                Token::Str("end"),
                Token::Some,
                Token::Map { len: Some(2) },
                Token::Str("c"),
                Token::Seq { len: Some(2) },
                Token::Str("end"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::Str("#f"),
                Token::U8(3),
                Token::MapEnd,
                Token::SeqEnd,
                Token::MapEnd,
                Token::SeqEnd,
            ],
        )
    }
}
