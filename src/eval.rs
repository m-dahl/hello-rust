use crate::prop::Prop::*;
use crate::prop::StateEval::*;
use crate::prop::*;
use crate::state::*;

fn value_eval(l: &StateEval, r: &StateEval, s: &State, cmp: impl Fn(bool, bool) -> bool) -> bool {
    fn get_value<'a>(se: &'a StateEval, s: &'a State) -> std::option::Option<&'a bool> {
        match se {
            ID(id) => s.state.get(id),
            LIT(b) => Some(b),
        }
    }
    let lv = get_value(l, s);
    let rv = get_value(r, s);
    let res = { lv.and_then(|a| rv.map(|b| cmp(*a, *b))) };
    res.unwrap() // TODO, return Result type
}

pub fn eval(p: &Prop, s: &State) -> bool {
    match p {
        AND(props) => props.iter().all(|p| eval(p, s)),
        OR(props) => props.iter().any(|p| eval(p, s)),
        NOT(prop) => !eval(prop, s),
        EQ(left, right) => value_eval(left, right, s, |l, r| l == r),
        TRUE => true,
        FALSE => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        use uuid::Uuid;
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let hh = hashmap![id1 => true, id2 => true];
        let s = State { state: hh };
        let p = NOT(Box::new(OR(vec![EQ(ID(id1), LIT(false)), FALSE])));
        assert_eq!(eval(&p, &s), true);

        let p = Box::new(OR(vec![EQ(ID(id1), LIT(false)), FALSE]));
        assert_eq!(eval(&p, &s), false);
    }

}
