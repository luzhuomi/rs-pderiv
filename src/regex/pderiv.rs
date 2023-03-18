pub mod bits;
pub mod parsetree;
pub mod transduce;


use super::list::*;
use super::re::*;
/**
 * partial derivative operation of a RE, given a letter l
 */
pub fn pderiv(r:&RE, l:&char) -> Vec<Box<RE>> {
    match r {
        RE::Phi => vec![],
        RE::Eps => vec![],
        RE::Lit(m) => { 
            if l == m { 
                vec![Box::new(RE::Eps)]
            } else { 
                vec![]
            }
        }
        RE::Seq(r1, r2) => { 
            if nullable(r1) {
                let ts =  pderiv(r1,l);
                let vs =  pderiv(r2,l);
                let mut res = vec![];
                for t in ts {
                    res.push(Box::new(RE::Seq(t, r2.clone())));                    
                };
                for v in vs {
                    res.push(v);
                }
                nub_vec(&res)

            } else {
                let ts = pderiv(r1,l);
                let mut res = Vec::new();
                for t in ts {
                    res.push(Box::new(RE::Seq(t, r2.clone())));
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
                res.push(Box::new(RE::Seq(t, Box::new(r.clone()))));    
            }
            nub_vec(&res)
        }
        
    }
}

/**
 * using List instead of Vec, but we are not using this.
 */
pub fn pderiv_l(r:&RE, l:&char) -> List<RE> {
    match r {
        RE::Phi => List::Nil,
        RE::Eps => List::Nil,
        RE::Lit(m) => {
            if l == m {
                List::Cons(RE::Eps, Box::new(List::Nil))
            } else {
                List::Nil
            }
        }, 
        RE::Seq(r1, r2) => {
            if nullable(r1) {
                let ts = pderiv_l(r1,l);
                let ps = ts.map( |t| { RE::Seq(Box::new(t.clone()),r2.clone()) });
                let vs = pderiv_l(r2, l);
                let res = (&ps).append(vs);
                res
            } else {
                let ts = pderiv_l(r1,l);
                let res = ts.map( |t| { RE::Seq(Box::new(t.clone()),r2.clone()) });
                res
            }
        },
        RE::Choice(r1, r2) => {
            let ts = pderiv_l(r1,l);
            let vs = pderiv_l(r2,l);
            (&ts).append(vs)
        },
        RE::Star(r1) => {
            let ts = pderiv_l(r1,l);
            let res = ts.map( |t| { RE::Seq(Box::new(t.clone()),Box::new(r.clone())) });    
            res        
        }
    }
}