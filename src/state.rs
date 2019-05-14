extern crate std;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct State {
    pub state: HashMap<Uuid, bool>,
}
