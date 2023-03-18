
use std::collections::HashSet;
use std::hash::Hash;
use super::list::*;

/** 
 * the RE data type
 */
#[derive(Debug)]
#[derive(Hash)]
pub enum RE {
    Phi,
    Eps,
    Lit(char),
    Seq(Box<RE>,Box<RE>),
    Choice(Box<RE>,Box<RE>),
    Star(Box<RE>)
}

/** 
 * impelementing the clone() for RE
 */
impl Clone for RE {
    fn clone(&self) -> Self {
        match self {
            RE::Phi => RE::Phi,
            RE::Eps => RE::Eps,
            RE::Lit(l) => RE::Lit(l.clone()),
            RE::Seq(r1, r2) => RE::Seq(r1.clone(), r2.clone()),
            RE::Choice(r1, r2) => RE::Choice(r1.clone(), r2.clone()),
            RE::Star(r) => RE::Star(r.clone())
        }
    }
}


/**
 * 
 */
impl PartialEq for RE {
    fn eq(&self, other:&Self) -> bool {
        match (self, other) {
            (RE::Phi, RE::Phi) => true,
            (RE::Eps, RE::Eps) => true, 
            (RE::Lit(l), RE::Lit(m)) => l == m,
            (RE::Seq(r1,r2), RE::Seq(r3,r4)) => r1 == r3 && r2 == r4,
            (RE::Choice(r1,r2), RE::Choice(r3,r4)) => r1 == r3 && r2 == r4,
            (RE::Star(r1), RE::Star(r2)) => r1 == r2,
            (_,_) => false
        }
    }
}

impl Eq for RE {
}

/**
 * test whether a RE is nullable
 */
pub fn nullable(r:&RE) -> bool {
    match r {
        RE::Phi => false,
        RE::Eps => true,
        RE::Star(_) => true,
        RE::Choice(r1,r2) => nullable(r1) || nullable(r2),
        RE::Seq(r1,r2) => nullable(r1) && nullable(r2),
        RE::Lit(_) => false
    }
}

pub fn nub_vec<T:Clone+Hash+Eq>(v:&Vec<T>)-> Vec<T> {
    let mut seen = HashSet::new();
    let mut res = Vec::new();
    for x in v.iter() {
        if !seen.contains(x) { 
            seen.insert(x);
            res.push(x.clone());
        }
    }
    res
}

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