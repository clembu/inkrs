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
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn de_tree_cmd() {
        assert_de_tokens(
            &Tree::Leaf(obj::Obj::Cmd(obj::cmd::Cmd::End)),
            &[Token::Str("end")],
        );
    }

}
