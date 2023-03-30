pub mod bits;
pub mod parsetree;
pub mod transduce;
pub mod parse;
pub mod parse_lazy;


use super::re::*;
use std::rc::Rc;
/**
 * partial derivative operation of a RE, given a letter l
 */
pub fn pderiv(r:&RE, l:&char) -> Vec<Rc<RE>> {
    match r {
        RE::Phi => vec![],
        RE::Eps => vec![],
        RE::Lit(m) => { 
            if l == m { 
                vec![Rc::new(RE::Eps)]
            } else { 
                vec![]
            }
        }
        RE::Seq(r1, r2) => { 
            if r1.nullable() {
                let ts =  pderiv(r1,l);
                let vs =  pderiv(r2,l);
                let mut res = vec![];
                for t in ts {
                    res.push(Rc::new(RE::Seq(t, Rc::clone(r2))));                    
                };
                for v in vs {
                    res.push(v);
                }
                nub_vec(&res)

            } else {
                let ts = pderiv(r1,l);
                let mut res = Vec::new();
                for t in ts {
                    res.push(Rc::new(RE::Seq(t,  Rc::clone(r2))));
                };
                nub_vec(&res)    
            }
        }
        RE::Choice(r1,r2) => {
            let ts = pderiv(r1,l);
            let vs = pderiv(r2,l);
            let mut res = vec![];
            for t in ts { res.push(t) };
            for v in vs { res.push(v) };
            nub_vec(&res)
        },
        RE::Star(r1) => {
            let ts = pderiv(r1, l);
            let mut res = vec![];
            for t in ts {
                res.push(Rc::new(RE::Seq(t, Rc::new(r.clone()))));    
            }
            nub_vec(&res)
        }
        
    }
}


