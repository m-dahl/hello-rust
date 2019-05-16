#[macro_use]
mod utils;

#[macro_use]
extern crate nom;

mod eval;
mod parse;
mod prop;
mod state;
mod test;

use eval::eval;
use parse::parse;

fn main() {
    let id1 = uuid::Uuid::new_v4();
    let id2 = uuid::Uuid::new_v4();
    let hh = hashmap![id1 => true, id2 => true];
    let s = state::State { state: hh };
    println!("state: {:?}", s);

    let prop = format!(
        "{} == true && {} == false",
        id1.to_hyphenated().to_string(),
        id2.to_hyphenated().to_string()
    );
    let p = parse(&prop);
    println!("evaluating: {:?} => result: {}", p, eval(&p, &s));
}
