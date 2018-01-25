//! Corresponds to inkle's ChoicePoint
use tree::path::Path;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct Choice {
    #[serde(rename = "*")] goto: Path,
    #[serde(rename = "flg")] flags: u8,
}

#[cfg(test)]
mod tests_serde {
    use super::*;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn de_choice() {
        assert_de_tokens(
            &Choice {
                goto: Path::from(".0.c"),
                flags: 0x03,
            },
            &[
                Token::Map { len: Some(2) },
                Token::Str("*"),
                Token::Str(".0.c"),
                Token::Str("flg"),
                Token::U8(3),
                Token::MapEnd,
            ],
        );
    }
}
