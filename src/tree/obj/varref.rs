use tree::path::Path;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) enum VarRef {
    #[serde(rename = "CNT?")] Cnt(Path),
    #[serde(rename = "VAR?")] Var(String),
}

#[cfg(test)]
mod tests_serde {
    use super::*;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn de_varref() {
        assert_de_tokens(
            &VarRef::Var("this".to_string()),
            &[
                Token::NewtypeVariant {
                    name: "VarRef",
                    variant: "VAR?",
                },
                Token::Str("this"),
            ],
        );
    }

    #[test]
    fn de_cntref() {
        assert_de_tokens(
            &VarRef::Cnt(Path::from(".^.0.c")),
            &[
                Token::NewtypeVariant {
                    name: "VarRef",
                    variant: "CNT?",
                },
                Token::Str(".^.0.c"),
            ],
        )
    }
}
