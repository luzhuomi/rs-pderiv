use bitvec::prelude::*;
use combine::Parser;
use crate::regex::re::*;
use crate::regex::syntax::parser::*;
use crate::regex::syntax::desugar::*;
use crate::regex::syntax::ext::*;
use crate::regex::syntax::ext::Ext::*;
use std::rc::Rc;
use std::collections::HashSet;



#[test]
fn test_parser_simp_us_addr() {
    let input = "^(.*) ([A-Za-z]{2}) ([0-9]{5})(-[0-9]{4})?$";
    let expected = Concat(vec![Carat, GrpNonMarking(Rc::new(Star(Rc::new(Dot), true))), Char(' '), 
        GrpNonMarking(Rc::new(Bound(Rc::new(Any(HashSet::from(['I', 'B', 'h', 'q', 'R', 'f', 'j', 'a', 'K', 'C', 'o', 'G', 'T', 'U', 'e', 'Q', 'g', 'p', 't', 'u', 'r', 'm', 'd', 'w', 'i', 'n', 'E', 'J', 'N', 'S', 'b', 's', 'v', 'Y', 'D', 'F', 'c', 'L', 'O', 'V', 'k', 'W', 'l', 'A', 'x', 'y', 'X', 'H', 'M', 'P']))), 2, None, true))), Char(' '), 
        GrpNonMarking(Rc::new(Bound(Rc::new(Any(HashSet::from(['4', '6', '0', '8', '7', '2', '1', '5', '3']))), 5, None, true))), 
        Opt(Rc::new(GrpNonMarking(Rc::new(Concat(vec![Char('-'), Bound(Rc::new(Any(HashSet::from(['1', '6', '0', '8', '3', '7', '4', '2', '5']))), 4, None, true)])))), true), Dollar]);
    let result = parse_ext().parse(input);
    match result {
        Err(err) => print!("{:?}", err),
        Ok((ext, _rest)) => {
            let simp = simp_ext(&ext);
            // print!("{:?}", simp) ;
            assert_eq!(expected, simp)
        } 
    }
}



#[test]
fn test_parser_simp_to_re_us_addr() {
    let input = "^(.*) ([A-Za-z]{2}) ([0-9]{5})(-[0-9]{4})?$";
    let expected = Concat(vec![Carat, GrpNonMarking(Rc::new(Star(Rc::new(Dot), true))), Char(' '), 
        GrpNonMarking(Rc::new(Bound(Rc::new(Any(HashSet::from(['I', 'B', 'h', 'q', 'R', 'f', 'j', 'a', 'K', 'C', 'o', 'G', 'T', 'U', 'e', 'Q', 'g', 'p', 't', 'u', 'r', 'm', 'd', 'w', 'i', 'n', 'E', 'J', 'N', 'S', 'b', 's', 'v', 'Y', 'D', 'F', 'c', 'L', 'O', 'V', 'k', 'W', 'l', 'A', 'x', 'y', 'X', 'H', 'M', 'P']))), 2, None, true))), Char(' '), 
        GrpNonMarking(Rc::new(Bound(Rc::new(Any(HashSet::from(['4', '6', '0', '8', '7', '2', '1', '5', '3']))), 5, None, true))), 
        Opt(Rc::new(GrpNonMarking(Rc::new(Concat(vec![Char('-'), Bound(Rc::new(Any(HashSet::from(['1', '6', '0', '8', '3', '7', '4', '2', '5']))), 4, None, true)])))), true), Dollar]);
    let result = parse_ext().parse(input);
    match result {
        Err(err) => print!("{:?}", err),
        Ok((ext, _rest)) => {
            let re = ext_to_re(&simp_ext(&ext));
            match re {
                Err(err) => print!("{:?}", err),
                Ok(re) => print!("{:?}", re)
            }
        } 
    }
}