#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) enum Cmd {
    #[serde(rename = "ev")] EvalStart,
    #[serde(rename = "out")] EvalOutput,
    #[serde(rename = "/ev")] EvalEnd,
    #[serde(rename = "du")] Duplicate,
    #[serde(rename = "pop")] PopEvaluatedValue,
    #[serde(rename = "~ret")] PopFunction,
    #[serde(rename = "->->")] PopTunnel,
    #[serde(rename = "str")] BeginString,
    #[serde(rename = "/str")] EndString,
    #[serde(rename = "nop")] NoOp,
    #[serde(rename = "choiceCnt")] ChoiceCount,
    #[serde(rename = "turns")] TurnsSince,
    #[serde(rename = "readc")] ReadCount,
    #[serde(rename = "rnd")] Random,
    #[serde(rename = "srnd")] SeedRandom,
    #[serde(rename = "visit")] VisitIndex,
    #[serde(rename = "seq")] SequenceShuffleIndex,
    #[serde(rename = "thread")] StartThread,
    #[serde(rename = "done")] Done,
    #[serde(rename = "end")] End,
    #[serde(rename = "listInt")] ListFromInt,
    #[serde(rename = "range")] ListRange,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens, Token};
    #[test]
    fn cmd_deser() {
        test_unit_variant_de!(Cmd, EvalStart, "ev");
        test_unit_variant_de!(Cmd, EvalOutput, "out");
        test_unit_variant_de!(Cmd, EvalEnd, "/ev");
        test_unit_variant_de!(Cmd, Duplicate, "du");
        test_unit_variant_de!(Cmd, PopEvaluatedValue, "pop");
        test_unit_variant_de!(Cmd, PopFunction, "~ret");
        test_unit_variant_de!(Cmd, PopTunnel, "->->");
        test_unit_variant_de!(Cmd, BeginString, "str");
        test_unit_variant_de!(Cmd, EndString, "/str");
        test_unit_variant_de!(Cmd, NoOp, "nop");
        test_unit_variant_de!(Cmd, ChoiceCount, "choiceCnt");
        test_unit_variant_de!(Cmd, TurnsSince, "turns");
        test_unit_variant_de!(Cmd, ReadCount, "readc");
        test_unit_variant_de!(Cmd, Random, "rnd");
        test_unit_variant_de!(Cmd, SeedRandom, "srnd");
        test_unit_variant_de!(Cmd, VisitIndex, "visit");
        test_unit_variant_de!(Cmd, SequenceShuffleIndex, "seq");
        test_unit_variant_de!(Cmd, StartThread, "thread");
        test_unit_variant_de!(Cmd, Done, "done");
        test_unit_variant_de!(Cmd, End, "end");
        test_unit_variant_de!(Cmd, ListFromInt, "listInt");
        test_unit_variant_de!(Cmd, ListRange, "range");
    }
}
