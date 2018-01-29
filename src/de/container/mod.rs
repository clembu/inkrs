mod meta;
use self::meta::Meta;
use super::{Container, Tree};
use std::collections::HashMap;

impl Container {
    fn set_name(&mut self, n: Option<String>) {
        self.name = n;
    }

    #[cfg(test)]
    pub fn with_flags(c: Vec<Tree>, flags: u8) -> Container {
        Container {
            c,
            flags,
            name: None,
            named_c: HashMap::new(),
        }
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
        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(untagged)]
        enum Contained {
            Content(Tree),
            Meta(Option<Meta>),
        }

        let mut c: Vec<Tree> = Vec::new();
        let mut m: Option<Meta> = None;
        while let Some(cont) = seq.next_element::<Contained>()? {
            match cont {
                Contained::Content(t) => {
                    c.push(t);
                }
                Contained::Meta(mt) => {
                    m = mt;
                    break;
                }
            }
        }

        match m {
            None => Ok(Container {
                c,
                named_c: HashMap::new(),
                flags: 0,
                name: None,
            }),
            Some(m) => Ok(Container {
                c,
                named_c: m.content,
                flags: m.flags,
                name: m.name,
            }),
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
mod test {
    use super::super::*;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn de_container() {
        assert_de_tokens(
            &Container {
                c: vec![Tree::Leaf(obj::Obj::Cmd(obj::cmd::Cmd::End))],
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

    #[test]
    fn de_tree_nested_cont() {
        let subc: Container = Container {
            name: Some(String::from("c")),
            flags: 0x03,
            c: vec![Tree::Leaf(obj::Obj::Cmd(obj::cmd::Cmd::End))],
            named_c: HashMap::new(),
        };
        let mut nc: HashMap<String, Container> = HashMap::new();
        nc.insert(String::from("c"), subc);
        assert_de_tokens(
            &Container {
                name: None,
                flags: 0,
                c: vec![Tree::Leaf(obj::Obj::Cmd(obj::cmd::Cmd::End))],
                named_c: nc,
            },
            &[
                Token::Seq { len: None },
                Token::Str("end"),
                Token::Some,
                Token::Map { len: None },
                Token::Str("c"),
                Token::Seq { len: None },
                Token::Str("end"),
                Token::Some,
                Token::Map { len: None },
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
