use crate::regex::re::*;
use crate::regex::pderiv::*;


#[test]
fn test_pderiv_a_a() {
    let r = RE::Lit('a');
    let l = 'a';
    let result = pderiv(&r, &l);
    assert_eq!(result, vec![Box::new(RE::Eps)]); 
}


#[test]
fn test_pderiv_star_a_a() {
    let r = RE::Star(Box::new(RE::Lit('a')));
    let l = 'a';
    let result = pderiv(&r, &l);
    assert_eq!(result, vec![Box::new(RE::Seq(Box::new(RE::Eps), Box::new(RE::Star(Box::new(RE::Lit('a'))))))]); 
}


#[test]
fn test_pderiv_star_a_star_a_a() {
    use RE::*;
    let r = Seq(Box::new(Star(Box::new(Lit('a')))),Box::new(Star(Box::new(Lit('a')))));
    let l = 'a';
    let result = pderiv(&r, &l);
    assert_eq!(result, vec![Box::new(Seq(Box::new(Seq(Box::new(Eps), Box::new(Star(Box::new(Lit('a')))))), Box::new(Star(Box::new(Lit('a')))))), Box::new(Seq(Box::new(Eps), Box::new(Star(Box::new(Lit('a'))))))]); 
}