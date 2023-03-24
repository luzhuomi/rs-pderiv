
use std::collections::HashSet;
//use super::list::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::Wrapping;

macro_rules! seq{
    ($a:expr, $b:expr) => { 
        RE::Seq(Box::new($a), Box::new($b))
    }
}

macro_rules! star{
    ($a:expr) => { 
        RE::Star(Box::new($a))
    }
}


macro_rules! choice{
    ($a:expr, $b:expr) => { 
        RE::Choice(Box::new($a), Box::new($b))
    }
}

pub(crate) use seq; 
pub(crate) use star; 
pub(crate) use choice; 


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


pub fn nub_vec_fst<T:Clone+Hash+Eq, S>(v:Vec<(T,S)>)-> Vec<(T,S)> {
    let empty_seen = HashSet::new();
    let empty_res = Vec::new();
    let (seen, res) = v.into_iter().fold( (empty_seen, empty_res), | (mut seen,mut res), (t,s)|
        {
            if !seen.contains(&t) { 
                seen.insert(t.clone());
                res.push((t,s));
                (seen, res)    
            } else {
                (seen, res)
            }
        });
    res
}


// let's use the default hasher
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn hash2(x:&u64, y:&u64) -> u64 {
    let f:u64 = 256;
    (Wrapping(*x) * Wrapping(f) + Wrapping(*y)).0
}