use bitvec::prelude::*;
use combine::Parser;
use crate::regex::re::*;
use crate::regex::syntax::parser::*;
use crate::regex::syntax::ext::*;
use crate::regex::syntax::ext::Ext::*;
use std::rc::Rc;
use std::collections::HashSet;




#[test]
fn test_parser_ab_or_c_star_d_plus() {
    let input = "a(b|c)*d+";
    let result = parse_ext().parse(input);
    match result {
        Err(err) => print!("{:?}", err),
        Ok((ext, _rest)) => print!("{:?}", ext) 
    }
}


#[test]
fn test_parser_us_addr() {
    let input = "^(.*) ([A-Za-z]{2}) ([0-9]{5})(-[0-9]{4})?$";
    let expected = Or(vec![Concat(vec![Carat, Grp(Rc::new(Or(vec![Concat(vec![Star(Rc::new(Dot), true)])]))), 
            Char(' '), 
            Grp(Rc::new(Or(vec![Concat(vec![Bound(Rc::new(Any(HashSet::from(['g', 'i', 'U', 'X', 's', 'G', 'o', 'l', 'T', 'x', 'W', 'p', 'w', 'j', 'm', 'v', 'B', 'D', 'E', 'H', 'L', 'q', 'N', 'y', 'd', 'F', 'a', 'O', 'n', 'P', 'f', 'k', 'c', 't', 'M', 'C', 'e', 'u', 'K', 'I', 'R', 'J', 'V', 'h', 'r', 'A', 'S', 'Q', 'Y', 'b']))), 2, None, true)])]))), 
            Char(' '), 
            Grp(Rc::new(Or(vec![Concat(vec![Bound(Rc::new(Any(HashSet::from(['2', '6', '0', '4', '1', '5', '3', '7', '8']))), 5, None, true)])]))), 
            Opt(Rc::new(Grp(Rc::new(Or(vec![Concat(vec![Char('-'), Bound(Rc::new(Any(HashSet::from(['0', '5', '2', '4', '7', '3', '6', '1', '8']))), 4, None, true)])])))), true), Dollar])]);
    let result = parse_ext().parse(input);
    match result {
        Err(err) => print!("{:?}", err),
        Ok((ext, _rest)) => assert_eq!(expected, ext) 
    }
}



