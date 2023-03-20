use crate::regex::re::*;
use crate::regex::pderiv::*;


#[test]
fn test_pderiv_a_a() {
    let r = RE::Lit('a');
    let l = 'a';
    let result = pderiv(&r, &l);
    assert_eq!(result, vec![Box::new(RE::Eps)]); 
}