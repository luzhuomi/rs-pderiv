use bitvec::prelude::*;
use combine::Parser;
use crate::regex::re::*;
use crate::regex::syntax::parser::*;




#[test]
fn test_parser_ab_or_c_star_d_plus() {
    let input = "a(b|c)*d+";
    let result = parse_ext().parse(input);
    match result {
        Err(err) => print!("{:?}", err),
        Ok((ext, rest)) => print!("{:?}", ext) 
    }
}


#[test]
fn test_parser_us_addr() {
    let input = "^(.*) ([A-Za-z]{2}) ([0-9]{5})(-[0-9]{4})?$";
    let result = parse_ext().parse(input);
    match result {
        Err(err) => print!("{:?}", err),
        Ok((ext, rest)) => print!("{:?}", ext) 
    }
}

