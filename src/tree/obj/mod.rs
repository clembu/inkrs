mod cmd;
mod glue;
mod choice;
mod funcall;
mod value;
mod divert;
mod varassign;
mod varref;

pub(crate) enum Obj {
    Glue(glue::Glue),
    Tag(String),
    Cmd(cmd::Cmd),
    Divert(divert::Divert),
    VarRef(varref::VarRef),
    VarAssign(varassign::VarAssign),
    Value(value::Value),
    Choice(choice::Choice),
    FunCall(funcall::FunCall),
}
