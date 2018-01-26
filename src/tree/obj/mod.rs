mod cmd;
mod glue;
mod choice;
mod prim;
mod value;
mod divert;
mod varassign;
mod varref;
mod tag;

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
