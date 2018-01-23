#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Tag {
    #[serde(rename = "#")] text: String,
}

#[cfg(test)]
mod tests_serde {
    use super::*;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn de_tag() {
        assert_de_tokens(
            &Tag {
                text: "blue".to_string(),
            },
            &[
                Token::Map { len: Some(1) },
                Token::Str("#"),
                Token::Str("blue"),
                Token::MapEnd,
            ],
        );
    }
}
