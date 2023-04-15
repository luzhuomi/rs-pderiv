
use intmap::IntMap;
use std::collections::HashSet;
use std::rc::Rc;
use bitvec::prelude::*;
use super::super::re::*;
use super::bits::*;
// use super::parsetree::*;

type Trans = IntMap<Vec<(u64, BitVec)>>;
// type RMap  = IntMap<RE>; // u64 -> RE reverse mapping
type Finals = IntMap<BitVec>;


/**
 * TODO: check whether the .clone()'s are necessary
 */
#[derive(Debug)]
pub struct Regex {
    pub trans:Trans, 
    init: u64, 
    finals: Finals 
}

/**
 * for debugging purpose
 */
pub fn cnt(regex:&Regex)-> usize {
    regex.trans.len()
}


fn build_fix(all_states_sofar: HashSet<RE>,  curr_trans:Trans, sig:HashSet<char>) -> (HashSet<RE>, Trans) {
    let mut new_delta = all_states_sofar.iter().flat_map(
        |r| {
            let e = sig.iter().filter(|l| {
                let hash_r = calculate_hash(r);
                let hash_l = calculate_hash(l);
                let key = hash2(&hash_r, &hash_l);
                !(curr_trans.contains_key(key))
            }).flat_map(move |l| {
                let tfs = pderiv_bc(r, l);
                if tfs.len() == 0 {
                    None
                } else {
                    let tfdelta = tfs.into_iter().map(|(r,bv)|{
                        (r, bv)
                    });
                    Some((r,l,tfdelta))    
                }
            });
            e
        }).peekable();
    if new_delta.peek().is_none() { 
        (all_states_sofar, curr_trans)
    } else {
        let new_trans :Trans = curr_trans.clone();
        let (mut next_states, next_trans) = new_delta
            .fold((HashSet::new(), new_trans), |acc, t| {
                let (states_sofar, mut trans) = acc;
                let (src, c, dstbvs) = t;
                let hash_src = calculate_hash(src);
                let hash_c  = calculate_hash(c);
                let key = hash2(&hash_src, &hash_c);
                let states_out = dstbvs.clone().fold(states_sofar, |mut states, s| {
                    let (dst, _bv) = s;
                    let _inserted = states.insert(dst);
                    states
                });
                let dstbvs_clone = dstbvs.clone();
                let val = dstbvs_clone.map(move |(dst, bv)| 
                {
                    let hash_dst = calculate_hash(&dst);
                    let k = hash_dst.clone();
                    (k, bv)
                }).collect();
                trans.insert(key, val);
                (states_out, trans)
        });
        next_states.extend(all_states_sofar); // todo check why all_states_sofar can't be use as the init of next_states fold.
        build_fix(next_states,next_trans, sig)
    }
}

pub fn build_regex<'a> (r:&'a RE) -> Regex {
    let sig = r.sigma();
    let init_states = vec![r.clone()].into_iter().collect();
    let (all_states, trans) = build_fix(init_states, IntMap::new(), sig);
    let emp_im :IntMap<BitVec> = IntMap::new();
    let fins = all_states.iter().filter(|r|{r.nullable()}).fold(emp_im, |mut im,r|{
        let hash_r = calculate_hash(r);
        im.insert(hash_r, emp_code(r));
        im
    });

    Regex{trans, init : calculate_hash(r), finals:fins}
}

use std::vec::IntoIter;


/* 
pub enum CoerceIter<'a, T, I, F, G> 
    where 
    I : Iterator<Item=T>,
    F : FnMut(T) -> T, 
    G : FnMut(T) -> Map<IntoIter<T>, F>
{
    FromIter(Iter<'a,T>),
    FromFlatMap(FlatMap<I, Map<IntoIter<T>, F>, G>)
} 

impl <'a, T, I, F, G>  Iterator for CoerceIter<'a, T, I, F, G> 
    where 
    I : Iterator<Item=T>,
    F : FnMut(T) -> T, 
    G : FnMut(T) -> Map<IntoIter<T>, F>
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        None // todo
    }
}


type MyIter<'a> = CoerceIter<'a, (&'a u64, BitVec), dyn Iterator<Item=(&'a u64, BitVec)>, dyn FnMut((&'a u64, BitVec)) -> (&'a u64, BitVec), dyn FnMut((&'a u64, BitVec)) -> Map<IntoIter<(&'a u64, BitVec)>,dyn  FnMut((&'a u64, BitVec)) -> (&'a u64, BitVec)>>;

*/


/*
pub enum CoerceIter<'a>
{
    FromIter(Iter<'a, (&'a u64, BitVec)>),
    FromFlatMap(FlatMap< dyn Iterator<Item=(&'a u64, BitVec)>, Map< IntoIter<(&'a u64, BitVec)>,  dyn FnMut((&'a u64, BitVec)) -> (&'a u64, BitVec)>, dyn FnMut((&'a u64, BitVec)) -> Map<IntoIter<(&'a u64, BitVec)>, dyn FnMut((&'a u64, BitVec)) -> (&'a u64, BitVec)>>)
} 
*/


pub enum CoerceIterator<'a>
{
    FromIntoIter(IntoIter<(u64, BitVec)>),
    FromIterator(Rc<dyn Iterator<Item=(u64, BitVec)> + 'a>)
} 

impl <'a>  Iterator for CoerceIterator<'a>
{
    type Item = (u64, BitVec);
    fn next(&mut self) -> Option<(u64, BitVec)> {
        match self {
            CoerceIterator::FromIntoIter(iit) => iit.next(),
            CoerceIterator::FromIterator(rcit) => {
                match Rc::get_mut(rcit) {
                    None => None,
                    Some(it) => it.next()
                }
            }
        }
    }
}


impl <'a> Clone for CoerceIterator<'a>
{
    fn clone(&self) -> Self {
        match self {
            CoerceIterator::FromIntoIter(iter) => CoerceIterator::FromIntoIter(iter.clone()),
            CoerceIterator::FromIterator(box_iter) => CoerceIterator::FromIterator(box_iter.clone())
        }
    }
}


impl Regex {
    pub fn parse_regex<'a>(&'a self, s:&String) -> Option<BitVec>{
        let init = &self.init;
        let empty_bv = BitVec::new();
        let rbc_vec = vec![(init.clone(),empty_bv)];
        let init_rbc = rbc_vec.into_iter();
        let init_iter = CoerceIterator::FromIntoIter(init_rbc);
        let mut result =  self.go(init_iter,  &s);
        match result.next() {
            None => None,
            Some(bv) => {
                let r = bv.clone();
                Some(r) // Some(bv)
            }
        }
    
    }

    pub fn go<'a>(&'a self, rbc:CoerceIterator<'a>, s:&'a str) -> impl Iterator<Item=BitVec> +'a {
        let mut mrbc = rbc;
        let mut ms = s;
        
        while ms.len() >0 {
            let ox = ms[0..1].chars().nth(0);
            let (x,xs):(char, &str) = match ox {
                None => panic!("parse_regex failed, empty string slice with len > 0"),
                Some(c) => (c,&ms[1..])
            };
            let tbc = mrbc.clone().flat_map(move |(r,bc)| {
                let hash_r = r;
                let hash_x = calculate_hash(&x);
                let key = hash2(&hash_r, &hash_x);
                let g = move |x:(u64, BitVec)| -> (u64, BitVec) {
                    let (t, bc1) = x;
                    let mut bc2 = bc.clone();
                    bc2.extend(bc1);
                    (t,bc2)
                };
                match self.trans.get(key) {
                    None => {
                        let empty:Vec<(u64, BitVec)> = vec![];
                        empty.into_iter().map(g)
                    },
                    Some(tfs) => { 
                        tfs.clone().into_iter().map(g)
                    }
                }
            });
            let c = tbc.into_iter();
            mrbc = CoerceIterator::FromIterator(Rc::new(c)); // fixme: how to ndeduplicate this?
            ms = xs;
        }
        let f = |(r, bc):(u64, BitVec)| -> Option<BitVec> {
            let mut bc2 = bc.clone();
            match self.finals.get(r) {
                None => None,
                Some(bc1) => {
                    bc2.extend(bc1);
                    Some(bc2)
                }
            }
        };
        mrbc.flat_map(f)
    } 
}



