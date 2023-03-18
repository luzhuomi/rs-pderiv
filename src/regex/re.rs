
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

