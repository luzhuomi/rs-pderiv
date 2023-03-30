use crate::regex::re::*;
use crate::regex::pderiv::*;
use std::rc::Rc;


#[test]
fn test_pderiv_a_a() {
    let r = RE::Lit('a');
    let l = 'a';
    let result = pderiv(&r, &l);
    assert_eq!(result, vec![Rc::new(RE::Eps)]); 
}


#[test]
fn test_pderiv_star_a_a() {
    let r = RE::Star(Rc::new(RE::Lit('a')));
    let l = 'a';
    let result = pderiv(&r, &l);
    assert_eq!(result, vec![Rc::new(RE::Seq(Rc::new(RE::Eps), Rc::new(RE::Star(Rc::new(RE::Lit('a'))))))]); 
}


#[test]
fn test_pderiv_star_a_star_a_a() {
    use RE::*;
    let r = Seq(Rc::new(Star(Rc::new(Lit('a')))),Rc::new(Star(Rc::new(Lit('a')))));
    let l = 'a';
    let result = pderiv(&r, &l);
    assert_eq!(result, vec![Rc::new(Seq(Rc::new(Seq(Rc::new(Eps), Rc::new(Star(Rc::new(Lit('a')))))), Rc::new(Star(Rc::new(Lit('a')))))), Rc::new(Seq(Rc::new(Eps), Rc::new(Star(Rc::new(Lit('a'))))))]); 
}