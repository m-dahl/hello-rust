extern crate std;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum Prop {
    AND(Box<Prop>,Box<Prop>),
    OR(Box<Prop>,Box<Prop>),
    NOT(Box<Prop>),
    EQ(Box<Prop>, Box<Prop>),
    ID(Uuid),
    TRUE,
    FALSE,
}
