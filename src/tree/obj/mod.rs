pub(crate) mod cmd;
pub(crate) mod glue;
pub(crate) mod choice;
pub(crate) mod prim;
pub(crate) mod value;
pub(crate) mod divert;
pub(crate) mod varassign;
pub(crate) mod varref;
pub(crate) mod tag;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub(crate) enum Obj {
    Glue(glue::Glue),
    Tag(tag::Tag),
    Cmd(cmd::Cmd),
    Divert(divert::Divert),
    VarRef(varref::VarRef),
    VarAssign(varassign::VarAssign),
    Value(value::Value),
    Choice(choice::Choice),
    Primitive(prim::Prim),
}
