#[macro_use]
mod utils;

mod prop;
mod state;
mod eval;


use eval::eval;

fn main() {
    use prop::Prop::*;
    use prop::StateEval::*;

    let id1 = uuid::Uuid::new_v4();
    let id2 = uuid::Uuid::new_v4();
    let hh = hashmap![id1 => true, id2 => true];
    let s = state::State { state: hh };
    println!("state: {:?}", s);

    let p = NOT(Box::new(OR(vec![EQ((ID(id1)), (LIT(false))), FALSE])));
    println!("Proposition: {:?}", p);

    println!("result: {}", eval(&p, &s));

    let my_uuid: Result<uuid::Uuid, &'static str> = Ok(uuid::Uuid::new_v4()); // Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A9");
    match my_uuid {
        Ok(uuid) => println!("{}", uuid.to_hyphenated()),
        Err(e) => println!("ERROR {}", e),
    }
}

#[cfg(test)]
 mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(true,true);
    }

}
