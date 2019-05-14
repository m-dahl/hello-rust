extern crate std;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum Prop {
    AND(Vec<Prop>),
    OR(Vec<Prop>),
    NOT(Box<Prop>),
    EQ(StateEval, StateEval),
    TRUE,
    FALSE,
}

#[derive(Debug, PartialEq)]
pub enum StateEval {
    ID(Uuid),
    LIT(bool),
}
