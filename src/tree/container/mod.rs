mod meta;
use super::Tree;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub(crate) struct Container {
    pub(super) c: Vec<Tree>,
    pub(super) named_c: HashMap<String, Container>,
    pub(super) flags: u8,
    pub(super) name: Option<String>,
}

impl Container {
    fn set_name(&mut self, n: Option<String>) {
        self.name = n;
    }
}

struct ContainerVisitor;
use serde::de::{self, Visitor};

use std::fmt;
impl<'de> Visitor<'de> for ContainerVisitor {
    type Value = Container;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a sequence of ink objects plus an optional metadata object")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Container, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut c: Vec<Tree> = Vec::new();
        while let Some(k) = seq.size_hint() {
            if k < 2 {
                break;
            }
            if let Some(e) = seq.next_element::<Tree>()? {
                c.push(e);
            }
        }

        if let Some(Some(m)) = seq.next_element::<Option<meta::Meta>>()? {
            println!("Deser: {:?}", m);
            Ok(Container {
                c,
                named_c: m.content,
                flags: m.flags,
                name: m.name,
            })
        } else {
            Ok(Container {
                c,
                named_c: HashMap::new(),
                flags: 0,
                name: None,
            })
        }
    }
}

use serde::Deserialize;
impl<'de> Deserialize<'de> for Container {
    fn deserialize<D>(d: D) -> Result<Container, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_any(ContainerVisitor)
    }
}

#[cfg(test)]
mod tests_serde {
    use super::*;
    use tree;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn de_container() {
        assert_de_tokens(
            &Container {
                c: vec![Tree::Leaf(tree::obj::Obj::Cmd(tree::obj::cmd::Cmd::End))],
                flags: 0x03,
                name: Some("c".to_string()),
                named_c: HashMap::new(),
            },
            &[
                Token::Seq { len: Some(2) },
                Token::Str("end"),
                Token::Some,
                Token::Map { len: Some(2) },
                Token::Str("#n"),
                Token::Str("c"),
                Token::Str("#f"),
                Token::U8(3),
                Token::MapEnd,
                Token::SeqEnd,
            ],
        );
    }
}
