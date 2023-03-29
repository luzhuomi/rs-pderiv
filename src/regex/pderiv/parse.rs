
use intmap::IntMap;
use std::collections::HashSet;
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
pub struct Regex <'a> {
    pub trans:Trans, 
    init: &'a RE, 
    finals: Finals 
}

/**
 * for debugging purpose
 */
pub fn cnt(regex:&Regex)-> usize {
    regex.trans.len()
}

/**
 * pre cond 1  allStatesSoFar \insect newStates = \emptyset
            2  dom(dom(currTrans)) \in allStatesFoFar
 */
fn build_fix(mut all_states_sofar: HashSet<RE>, new_states: HashSet<RE>, curr_trans:Trans, sig:HashSet<char>) -> (HashSet<RE>, Trans) {
    if new_states.len() == 0 {
        (all_states_sofar, curr_trans)
    } else {
        let new_states_clone = new_states.clone();
        let new_delta = new_states_clone.iter().flat_map(
            |r| {
                let e = sig.iter().filter(|l|{
                    let hash_r = calculate_hash(r);
                    let hash_l = calculate_hash(l);
                    let key = hash2(&hash_r, &hash_l);
                    !(curr_trans.contains_key(key))
                }).flat_map(move |l| {
                    let tfs = pderiv_bc(r,l);
                    if tfs.len() == 0 {
                        None
                    } else {
                        let tfdelta = tfs.into_iter();
                        Some((r,l,tfdelta))    
                    }
                });
                e
            });
        all_states_sofar.extend(new_states);
        let all_states_next:HashSet<RE> = all_states_sofar;
        let new_trans :Trans = curr_trans.clone();
        let (new_states_next, next_trans) = new_delta
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
                trans.insert(key, dstbvs.into_iter().map(|(dst, bv)| (calculate_hash(&dst), bv)).collect());
                (states_out, trans)
        });
        build_fix(all_states_next, new_states_next, next_trans, sig)
    }
}


pub fn build_regex(r:&RE) -> Regex {
    let sig = r.sigma();
    let init_all_states = vec![].into_iter().collect();
    let init_new_states = vec![r.clone()].into_iter().collect();
    let (all_states, trans) = build_fix(init_all_states, init_new_states, IntMap::new(), sig);
    let emp_im :IntMap<BitVec> = IntMap::new();
    let fins = all_states.iter().filter(|r|{r.nullable()}).fold(emp_im, |mut im,r|{
        let hash_r = calculate_hash(r);
        im.insert(hash_r, emp_code(r));
        im
    });
    Regex{trans : trans, init : r, finals:fins}
}

/* 
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
                    let tfdelta = tfs.into_iter();
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
                trans.insert(key, dstbvs.into_iter().map(|(dst, bv)| (calculate_hash(&dst), bv)).collect());
                (states_out, trans)
        });
        next_states.extend(all_states_sofar); // todo check why all_states_sofar can't be use as the init of next_states fold.
        build_fix(next_states,next_trans, sig)
    }
}

pub fn build_regex(r:&RE) -> Regex {
    let sig = r.sigma();
    let init_states = vec![r.clone()].into_iter().collect();
    let (all_states, trans) = build_fix(init_states, IntMap::new(), sig);
    let emp_im :IntMap<BitVec> = IntMap::new();
    let fins = all_states.iter().filter(|r|{r.nullable()}).fold(emp_im, |mut im,r|{
        let hash_r = calculate_hash(r);
        im.insert(hash_r, emp_code(r));
        im
    });
    Regex{trans : trans, init : r, finals:fins}
}
*/


impl <'a> Regex<'a> {
    pub fn parse_regex(&self, s:&'a String) -> Option<BitVec> {
        fn go<'a>(rbc:Vec<(u64,BitVec)>, trans:&Trans, finals:&Finals, s:&str) -> Vec<BitVec> {
            if s.len() == 0 {
                let mut res:Vec<BitVec> = vec![];
                rbc.into_iter().for_each(|(r, bc)| {
                    match finals.get(r) {
                        None => {}
                        Some(bc1) => {
                            let mut bc2 = bc.clone();
                            bc2.extend(bc1);
                            res.push(bc2);
                        }
                    }
                });
                res
            } else {
                let ox = &s[0..1].chars().nth(0);
                let (x,xs) = match ox {
                    None => panic!("parse_regex failed, empty string slice with len > 0"),
                    Some(c) => (c,&s[1..])
                };
                let mut tbc:Vec<(u64, BitVec)> = vec![];
                rbc.into_iter().for_each(|(r,bc)| {
                    let hash_r = r;
                    let hash_x = calculate_hash(x);
                    let key = hash2(&hash_r, &hash_x);
                    match trans.get(key) {
                        None => {
                        }
                        Some(tfs) => tfs.into_iter().for_each(|tb|{
                            let (t, bc1) = tb;
                            let mut bc2 = bc.clone();
                            bc2.extend(bc1);
                            tbc.push((*t,bc2));
                        })
                    };
                });
                tbc = nub_vec_fst(tbc);
                go(tbc, trans, finals, xs)
            }
        }
    
        match self {
            Regex { trans, init, finals } => {
                let hash_init = calculate_hash(init);
                let result =  go(vec![(hash_init,BitVec::new())], trans, finals, &s);
                if result.len() == 0 {
                    None
                } else {
                    let bv: BitVec = (result.clone()[0]).clone();
                    Some(bv)
                }
            }
        }
    
    }
}

