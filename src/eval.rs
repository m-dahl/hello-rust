use crate::prop::Prop::*;
use crate::prop::*;
use crate::state::*;

pub fn eval(p: &Prop, s: &State) -> bool {
    match p {
        AND(p1, p2) => eval(p1, s) && eval(p2, s),
        OR(p1, p2) => eval(p1, s) || eval(p2, s),
        NOT(prop) => !eval(prop, s),
        EQ(p1, p2) => eval(p1, s) == eval(p2, s),
        ID(id) => *s.state.get(id).unwrap(),
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
        let p = NOT(Box::new(OR(Box::new(EQ(Box::new(ID(id1)), Box::new(FALSE))), Box::new(FALSE))));
        assert_eq!(eval(&p, &s), true);

        let p = Box::new(OR(Box::new(EQ(Box::new(ID(id1)), Box::new(FALSE))), Box::new(FALSE)));
        assert_eq!(eval(&p, &s), false);
    }

}
