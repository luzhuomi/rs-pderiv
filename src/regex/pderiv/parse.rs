
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
            3  all_states_sofar_im is the  intmap representation of all_states_sofar for quicker lookup
 */
fn build_fix(mut all_states_sofar: Vec<RE>, mut all_states_sofar_im: IntMap<()>, new_states: Vec<RE>, curr_trans:Trans, sig:HashSet<char>) -> (Vec<RE>, Trans) {
    if new_states.len() == 0 {
        (all_states_sofar, curr_trans)
    } else {
        /* 
        let new_states_clone = new_states.clone(); // todo, this clone is expensive when states are huges, e.g. 10k
        let new_delta:Vec<_> = new_states_clone.iter().flat_map(
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
                        let tfdelta = tfs;
                        Some((r,l,tfdelta))    
                    }
                });
                e
            }).collect();
        */
        let (new_states_clone, new_data_nested):(Vec<_>, Vec<_>) = new_states.into_iter().map(
            |r| {
                let e:Vec<_> = sig.iter()
                /* .filter(|l|{
                    let hash_r = calculate_hash(&r);
                    let hash_l = calculate_hash(l);
                    let key = hash2(&hash_r, &hash_l);
                    !(curr_trans.contains_key(key))
                })*/
                .flat_map(|l| {
                    let l_c = l.clone();
                    let tfs = pderiv_bc(&r,&l_c);
                    if tfs.len() == 0 {
                        None
                    } else {
                        let tfdelta = tfs;
                        Some((calculate_hash(&r),l,tfdelta))    
                    }
                }).collect();
                (r,e)
            }).unzip();
        let new_data_flatten:Vec<_>= new_data_nested.into_iter().flat_map(|x|{x.into_iter()}).collect();
        
        all_states_sofar_im.extend(new_states_clone.iter().map(|t| {(calculate_hash(t), ())}));
        // dbg!(&all_states_sofar_im.len());
        let all_states_next_im:IntMap<()> = all_states_sofar_im;
        // new_state_next, for the next iteration
        // new_states_next_im, seems no use for now.
        let (new_states_next, next_trans) = new_data_flatten.into_iter()
            .fold((Vec::new(), curr_trans), |acc, t| {
                let (new_states_sofar, mut trans) = acc;
                let (src, c, dstbvs) = t;
                let hash_src = src;
                let hash_c  = calculate_hash(&c);
                let key = hash2(&hash_src, &hash_c);
                trans.insert(key, dstbvs.iter().map(|(dst, bv)| (calculate_hash(dst), bv.clone())).collect());
                let filtered_dstbv:Vec<(RE, BitVec)> = dstbvs.into_iter().filter(|(s,_bv)|
                { 
                    let hash_s = calculate_hash(&s);
                    !all_states_next_im.contains_key(hash_s)
                }).collect();
                let states_out = filtered_dstbv.into_iter().fold(new_states_sofar, |mut states, s| {
                    let (dst, _bv) = s;
                    // states_im.insert(calculate_hash(&dst), ());
                    let _inserted = states.push(dst);
                    states
                });

                (states_out, trans)
        });
        // dbg!(&new_states_next.len());
        all_states_sofar.extend(new_states_clone);
        let all_states_next:Vec<RE> = all_states_sofar;
        build_fix(all_states_next, all_states_next_im, new_states_next, next_trans, sig)
    }
}


pub fn build_regex(r:&RE) -> Regex { // todo: GET RID OF ALL_STATES, WHICH IS ONLY NEEDED TO CONSTRUCT THE FINS
    let sig = r.sigma();
    let init_all_states = vec![].into_iter().collect();
    let init_all_states_im = vec![].into_iter().collect();
    let init_new_states = vec![r.clone()].into_iter().collect();
    let (all_states, trans) = build_fix(init_all_states, init_all_states_im, init_new_states, IntMap::new(), sig);
    dbg!(&all_states.len());
    let emp_im :IntMap<BitVec> = IntMap::new();
    let fins = all_states.iter().filter(|r|{r.nullable()}).fold(emp_im, |mut im,r|{
        let hash_r = calculate_hash(r);
        im.insert(hash_r, emp_code(r));
        im
    });
    Regex{trans : trans, init : r, finals:fins}
}

// this could be inefficient.
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
                tbc = nub_vec_fst_u64(tbc);
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

