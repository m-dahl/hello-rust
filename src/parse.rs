use crate::prop::Prop::*;
use crate::prop::*;
use uuid::Uuid;

named!(parse_true<&str, Prop>, map!(tag_no_case!("true"), |_| TRUE));
named!(parse_false<&str, Prop>, map!(tag_no_case!("false"), |_| FALSE));
named!(parse_id<&str, Prop>, complete!(map_res!(take_s!(36), |s| Uuid::parse_str(s).map(|id| ID(id)))));
named!(parse_not<&str, Prop>, do_parse!(ws!(tag!("!")) >> p: parse_p1 >> (NOT(Box::new(p)))));
named!(parse_parens<&str, Prop>, delimited!(ws!(tag!("(")), ws!(parse_prop), tag!(")")));

// precedence levels: || lowest, && mid, comparison greater, variables, literals, and negation highest
named!(parse_prop<&str, Prop>,
       do_parse!(
           p: parse_p1 >>
           op: many0!(tuple!(complete!(ws!(tag!("||"))), parse_p1)) >>
           (make_binary_prop(p, op)))
);

named!(parse_p1<&str, Prop>,
       do_parse!(
           p: parse_p2 >>
           op: many0!(tuple!(complete!(ws!(tag!("&&"))), parse_p2)) >>
           (make_binary_prop(p, op)))
);

named!(parse_p2<&str, Prop>,
       do_parse!(
           p: parse_p3 >>
           op: many0!(tuple!(complete!(ws!(tag!("=="))), parse_p3)) >>
           (make_binary_prop(p, op)))
);

named!(parse_p3<&str, Prop>, alt_complete!(
    parse_id | parse_parens | parse_true | parse_false | parse_not
));

fn make_binary_prop(p: Prop, op: Vec<(&str, Prop)>) -> Prop {
    op.into_iter().fold(p, |acc, val| parse_op(val, acc))
}

fn parse_op(tup: (&str, Prop), p1: Prop) -> Prop {
    let (op, p2) = tup;
    match op {
        "&&" => AND(Box::new(p1), Box::new(p2)),
        "||" => OR(Box::new(p1), Box::new(p2)),
        "==" => EQ(Box::new(p1), Box::new(p2)),
        _ => panic!("Unknown Operation"),
    }
}

pub fn parse(s: &str) -> Prop {
    parse_p1(s).unwrap().1
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

        assert_eq!(parse_id(id_str), Ok(("", ID(id))));
        assert_eq!(parse_parens("(false)"), Ok(("", FALSE)));
    }

    #[test]
    fn test_parse() {
        let id_str = "95022733-f013-301a-0ada-abc18f151006";
        let id = Uuid::parse_str(id_str).unwrap();

        assert_eq!(parse_prop("TRUE"), Ok(("", TRUE)));
        assert_eq!(parse_prop("FALSE"), Ok(("", FALSE)));

        assert_eq!(
            parse_prop("TRUE&&FALSE&&FALSE"),
            Ok((
                "",
                AND(
                    Box::new(AND(Box::new(TRUE), Box::new(FALSE))),
                    Box::new(FALSE)
                )
            ))
        );

        assert_eq!(
            parse_prop("TRUE && FALSE ||    !FALSE"),
            Ok((
                "",
                OR(
                    Box::new(AND(Box::new(TRUE), Box::new(FALSE))),
                    Box::new(NOT(Box::new(FALSE)))
                )
            ))
        );
        assert_eq!(
            parse_prop("TRUE||  FALSE"),
            Ok(("", OR(Box::new(TRUE), Box::new(FALSE))))
        );
        assert_eq!(parse_prop("! TRUE"), Ok(("", NOT(Box::new(TRUE)))));

        let s = &format!("! ({} == false)", id_str);
        assert_eq!(
            parse_prop(s),
            Ok(("", NOT(Box::new(EQ(Box::new(ID(id)), Box::new(FALSE))))))
        );

        let s = &format!("TRUE || ({} == false)", id_str);
        assert_eq!(
            parse_prop(s),
            Ok((
                "",
                OR(
                    Box::new(TRUE),
                    Box::new(EQ(Box::new(ID(id)), Box::new(FALSE)))
                )
            ))
        );

        let s = &format!("{} == false || TRUE", id_str);
        assert_eq!(
            parse_prop(s),
            Ok((
                "",
                OR(
                    Box::new(EQ(Box::new(ID(id)), Box::new(FALSE))),
                    Box::new(TRUE)
                )
            ))
        );
    }

}
