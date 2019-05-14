use crate::prop::*;
use crate::state::*;

fn value_eval(l: &StateEval, r: &StateEval, s: &State, cmp: impl Fn(bool, bool) -> bool) -> bool {
    use crate::prop::StateEval::*;

    fn get_value<'a>(se: &'a StateEval, s: &'a State) -> std::option::Option<&'a bool> {
        match se {
            ID(id) => s.state.get(id),
            LIT(b) => Some(b),
        }
    }
    let lv = get_value(l, s);
    let rv = get_value(r, s);
    println!("lv: {:?}, rv: {:?}", lv, rv);
    let res = { lv.and_then(|a| rv.map(|b| cmp(*a, *b))) };
    println!("res: {:?}", res);
    res.unwrap()
}

pub fn eval(p: &Prop, s: &State) -> bool {
    use Prop::*;
    match p {
        AND(props) => props.iter().all(|p| eval(p, s)),
        OR(props) => props.iter().any(|p| eval(p, s)),
        NOT(prop) => !eval(prop, s),
        EQ(left, right) => value_eval(left, right, s, |l, r| l == r),
        TRUE => true,
        FALSE => false,
        _ => false,
    }
}

#[cfg(test)]
mod test_eval {
    use super::*;

    #[test]
    fn test_eval() {
        use std::collections::HashMap;
        use uuid::Uuid;
        use Prop::*;
        use StateEval::*;

        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let hh = hashmap![id1 => true, id2 => true];
        let s = State { state: hh };
        let p = NOT(Box::new(OR(vec![EQ((ID(id1)), (LIT(false))), FALSE])));
        assert_eq!(eval(&p, &s), false);

        let p = (Box::new(OR(vec![EQ((ID(id1)), (LIT(false))), FALSE])));
        assert_eq!(eval(&p, &s), false);
    }

}
