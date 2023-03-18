
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



impl RE {
    /**
     * test whether a RE is nullable
     */
    pub fn nullable(&self) -> bool {
        match self {
            RE::Phi => false,
            RE::Eps => true,
            RE::Star(_) => true,
            RE::Choice(r1,r2) => r1.nullable() || r2.nullable(),
            RE::Seq(r1,r2) => r1.nullable() && r2.nullable(),
            RE::Lit(_) => false
        }
    }

    /** 
     * return the set of characters use in a regex
     */
    pub fn sigma(&self) -> HashSet<char> {
        match self {
            RE::Phi => HashSet::new(),
            RE::Eps => HashSet::new(),
            RE::Lit(c) => { 
                HashSet::from([*c])
            }
            RE::Star(r) => r.sigma(),
            RE::Choice(r1, r2) => { 
                let v1 = r1.sigma();
                let v2 = r2.sigma();
                let v3: HashSet<_> = v1.union(&v2).map( | c | { *c }).collect();
                v3
            },
            RE::Seq(r1, r2) => {
                let v1 = r1.sigma();
                let v2 = r2.sigma();
                let v3: HashSet<_> = v1.union(&v2).map( | c | { *c }).collect();
                v3
            }
        }
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

