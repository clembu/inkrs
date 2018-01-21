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
