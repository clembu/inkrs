/*! This is actually not
 * the Story we're gonna
 * open to the api yet.
 *
 * This thing is just a deserialized
 * uncontextualized data structure.
 *
 * Ideally, we want to transform this
 * into something more strongly coupled
 * with its contents.
 *
 * Like, flagsets being their own types,
 * variable references actually referencing
 * variables, not their names,
 * and so on.
 */

use tree::container::Container;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Story {
    root: Container,
    #[serde(rename = "inkVersion")] v: u32,
    #[serde(rename = "listDefs")] flagsets: HashMap<String, HashMap<String, u32>>,
}

#[cfg(test)]
mod tests_serde {
    use super::*;
    use std::collections::HashMap;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn de_story_structure() {
        use tree::Tree;
        use tree::obj::*;
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
