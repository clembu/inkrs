#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) enum Glue {
    #[serde(rename = "G<")] Left,
    #[serde(rename = "G>")] Right,
    #[serde(rename = "<>")] Bi,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn glue_deser() {
        test_unit_variant_de!(Glue, Bi, "<>");
        test_unit_variant_de!(Glue, Left, "G<");
        test_unit_variant_de!(Glue, Right, "G>");
    }
}
