use crate::prop::Prop::*;
use crate::prop::StateEval::*;
use crate::prop::*;
use uuid::Uuid;

named!(parse_true<&str, Prop>, map!(tag_no_case!("true"), |_| TRUE));
named!(parse_false<&str, Prop>, map!(tag_no_case!("false"), |_| FALSE));

named!(parse_state_eval<&str, StateEval>, alt_complete!(parse_id | parse_lit));
named!(parse_id<&str, StateEval>, complete!(map_res!(take_s!(36), |s| Uuid::parse_str(s).map(|id| ID(id)))));
named!(parse_lit_true<&str, StateEval>, map!(tag_no_case!("true"), |_| LIT(true)));
named!(parse_lit_false<&str, StateEval>, map!(tag_no_case!("false"), |_| LIT(false)));
//named!(parse_lit<&str, StateEval>, map!(parse_to!(bool), |b| LIT(b)));
named!(parse_lit<&str, StateEval>, alt!(parse_lit_true | parse_lit_false));

named!(parse_factor<&str, Prop>, alt_complete!(
    parse_eq | parse_true | parse_false));

named!(parse_prop<&str, Prop>, alt_complete!(
    parse_and    |
    parse_or     |
    parse_not    |
    parse_factor
)
);

named!(parse_eq<&str, Prop>,
       do_parse!(s1: parse_state_eval >> ws!(tag!("==")) >> s2: parse_state_eval >> (EQ(s1, s2)))
);

named!(parse_and<&str, Prop>,
       do_parse!(p1: parse_factor >> ws!(tag!("&&")) >> p2: parse_factor >> (AND(vec![p1, p2])))
);

named!(parse_or<&str, Prop>,
       do_parse!(p1: parse_factor >> ws!(tag!("||")) >> p2: parse_factor >> (OR(vec![p1, p2])))
);

named!(parse_not<&str, Prop>,
       do_parse!(ws!(tag!("!")) >> p: parse_factor >> (NOT(Box::new(p))))
);

pub fn parse(s: &str) -> Prop {
    parse_prop(s).unwrap().1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atoms() {
        let id_str = "95022733-f013-301a-0ada-abc18f151006";
        let id = Uuid::parse_str(id_str).unwrap();

        assert_eq!(parse_true("true"), Ok(("", TRUE)));
        assert_eq!(parse_true("TRUE"), Ok(("", TRUE)));
        assert_ne!(parse_true("FALSE"), Ok(("", TRUE)));

        assert_eq!(parse_false("FALSE"), Ok(("", FALSE)));
        assert_eq!(parse_false("false"), Ok(("", FALSE)));
        assert_ne!(parse_false("TRUE"), Ok(("", FALSE)));

        assert_eq!(
            parse_and("TRUE  && FALSE"),
            Ok(("", AND(vec![TRUE, FALSE])))
        );
        assert_eq!(parse_or("TRUE||  FALSE"), Ok(("", OR(vec![TRUE, FALSE]))));

        assert_eq!(parse_id(id_str), Ok(("", ID(id))));

        assert_eq!(parse_lit("false"), Ok(("", LIT(false))));
        assert_eq!(parse_state_eval("false"), Ok(("", LIT(false))));

        let s = &format!("true ==  false");
        assert_eq!(parse_eq(s), Ok(("", EQ(LIT(true), LIT(false)))));

        let s = &format!("true ==  true && false == false");
        assert_eq!(
            parse_and(s),
            Ok((
                "",
                AND(vec![EQ(LIT(true), LIT(true)), EQ(LIT(false), LIT(false))])
            ))
        );
    }

    #[test]
    fn test_parse() {
        let id_str = "95022733-f013-301a-0ada-abc18f151006";
        let id = Uuid::parse_str(id_str).unwrap();

        assert_eq!(parse_prop("TRUE"), Ok(("", TRUE)));
        assert_eq!(parse_prop("FALSE"), Ok(("", FALSE)));

        assert_eq!(
            parse_prop("TRUE  && FALSE"),
            Ok(("", AND(vec![TRUE, FALSE])))
        );
        assert_eq!(parse_prop("TRUE||  FALSE"), Ok(("", OR(vec![TRUE, FALSE]))));
        assert_eq!(parse_prop("! TRUE"), Ok(("", NOT(Box::new(TRUE)))));

        let s = &format!("{} == false", id_str);
        assert_eq!(parse_prop(s), Ok(("", EQ(ID(id), LIT(false)))));

        let s = &format!("! {} == false", id_str);
        assert_eq!(
            parse_prop(s),
            Ok(("", NOT(Box::new(EQ(ID(id), LIT(false))))))
        );

        let s = &format!("TRUE || {} == false", id_str);
        assert_eq!(
            parse_prop(s),
            Ok(("", OR(vec![TRUE, EQ(ID(id), LIT(false))])))
        );

        let s = &format!("{} == false || TRUE", id_str);
        assert_eq!(
            parse_prop(s),
            Ok(("", OR(vec![EQ(ID(id), LIT(false)), TRUE])))
        );
    }

}
