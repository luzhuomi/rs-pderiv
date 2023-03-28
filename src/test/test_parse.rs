use bitvec::prelude::*;
use crate::regex::re::*;
use crate::regex::pderiv::*;
use crate::regex::pderiv::bits::*;
use crate::regex::pderiv::parse::*;




#[test]
fn test_parse_aa_a() {
    use RE::*;
    let r = seq!(Lit('a'),Lit('a'));
    let regex = build_regex(&r);
    let result = regex.parse_regex(&String::from("aa"));
    match result {
        None => assert_eq!(1,2),
        Some(bv) => assert_eq!(bv,bitvec![])
       
    }
}

#[test]
fn test_parse_star_a_a() {
    use RE::*;
    let r = star!(Lit('a'));
    let regex = build_regex(&r);
    let result = regex.parse_regex(&String::from("aaa"));
    match result {
        None => assert_eq!(1,2),
        Some(bv) => {
            assert_eq!(bv,bitvec![0,0,0,1])
        }
       
    }
}


#[test]
fn test_parse_abaac1() {
    use RE::*;
    let x = choice!(Lit('a'),seq!(Lit('a'),Lit('b')));
    let y = choice!(seq!(Lit('b'), seq!(Lit('a'), Lit('a'))), Lit('a'));
    let z = choice!(seq!(Lit('a'), Lit('c')), Lit('c')); 
    let r = seq!(seq!(x,y),z);
    let regex = build_regex(&r);
    let result = regex.parse_regex(&String::from("abaac"));
    match result {
        None => assert_eq!(1,2),
        Some(bv) => {
            assert_eq!(bv,bitvec![0,0,1])
        }
       
    }
}


#[test]
fn test_parse_abaac2() {
    use RE::*;
    let x = choice!(seq!(Lit('a'),Lit('b')),Lit('a'));
    let y = choice!(seq!(Lit('b'), seq!(Lit('a'), Lit('a'))), Lit('a'));
    let z = choice!(seq!(Lit('a'), Lit('c')), Lit('c')); 
    let r = seq!(seq!(x,y),z);
    let regex = build_regex(&r);
    let result = regex.parse_regex(&String::from("abaac"));
    match result {
        None => assert_eq!(1,2),
        Some(bv) => {
            assert_eq!(bv,bitvec![0,1,0])
        }
       
    }
}

