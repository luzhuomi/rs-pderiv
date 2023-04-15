
use intmap::IntMap;
use std::rc::Rc;
use std::collections::HashSet;
use bitvec::prelude::*;
use super::super::re::*;
use super::super::list::*;
use super::bits::*;
use super::parsetree::*;

// todo: implement a safe version of calculate_hash and IntMap
type Trans = IntMap<Vec<(u64, BitVec)>>;
// type RMap  = IntMap<RE>; // u64 -> RE reverse mapping
type Finals = IntMap<BitVec>;


/**
 * TODO: check whether the .clone()'s are necessary
 */
#[derive(Debug)]
pub struct Regex {
    pub trans:Trans, 
    init: RE, 
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
fn build_fix(mut cache: PDCached, mut all_states_sofar: Vec<RE>, mut all_states_sofar_im: IntMap<()>, mut new_states: Vec<RE>, mut curr_trans:Trans, sig:HashSet<char>) -> (Vec<RE>, Trans) {
    while new_states.len() > 0 {
        let (new_states_clone, new_data_nested):(Vec<_>, Vec<_>) = new_states.into_iter().map(
            |r| {
                let e:Vec<_> = sig.iter()
                .flat_map(|l| {
                    let l_c = l.clone();
                    let tfs = cache.pderiv_bc(&r,&l_c);
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
        // let all_states_next:Vec<RE> = all_states_sofar;
        all_states_sofar_im = all_states_next_im;
        new_states = new_states_next;
        curr_trans = next_trans;
        // build_fix(cache, all_states_next, all_states_next_im, new_states_next, next_trans, sig)
    }
    (all_states_sofar, curr_trans)
}


pub fn build_regex(r:&RE) -> Regex { // todo: GET RID OF ALL_STATES, WHICH IS ONLY NEEDED TO CONSTRUCT THE FINS
    let sig = r.sigma();
    let cache = PDCached::new();
    let init_all_states = vec![].into_iter().collect();
    let init_all_states_im = vec![].into_iter().collect();
    let init_new_states = vec![r.clone()].into_iter().collect();
    let (all_states, trans) = build_fix(cache, init_all_states, init_all_states_im, init_new_states, IntMap::new(), sig);
    dbg!(&all_states.len());
    let emp_im :IntMap<BitVec> = IntMap::new();
    let fins = all_states.iter().filter(|r|{r.nullable()}).fold(emp_im, |mut im,r|{
        let hash_r = calculate_hash(r);
        im.insert(hash_r, emp_code(r));
        im
    });
    Regex{trans : trans, init : r.clone(), finals:fins}
}



// One way to speed up is to keep track of the select sequence bitvec in reversed order.
// through Rc, multiple "next generations of bitvec  (w.r.t the same next state)" can share the same tail (previous generation)
// we reduce the time cloning and appending the bitvec and minimize the space requirement.


type Path<'a> = List<&'a BitVec>;

fn aggr(end:BitVec, path:Rc<List<&BitVec>>) -> BitVec {
    path.foldl(end, |mut bva, bv| {
            bva.extend(bv.iter());
            bva
    })
} 

impl  Regex {
    pub fn parse_regex(&self, s:&String) -> Option<BitVec> {
        // each rbc is the current state and the reversed bit vec path back to the start
        fn go<'a>(rbc:Vec<(u64,Rc<Path>)>, trans:&Trans, finals:&Finals, s:&str) -> Option<BitVec> {
            if s.len() == 0 {
                let mut res:Option<BitVec> = None;
                for (r,path) in rbc {
                    match finals.get(r) {
                        None => {}
                        Some(bc1) => {
                            let mut bc2 = aggr(bc1.clone(), path); // just need to clone 1
                            bc2.reverse();
                            res = Some(bc2);
                        }
                    }
                }
                res
            } else {
                let ox = &s[0..1].chars().nth(0);
                let (x,xs) = match ox {
                    None => panic!("parse_regex failed, empty string slice with len > 0"),
                    Some(c) => (c,&s[1..])
                };
                let mut tbc:Vec<(u64, Rc<List<&BitVec>>)> = vec![];
                rbc.into_iter().for_each(|(r,path)| {
                    let hash_r = r;
                    let hash_x = calculate_hash(x);
                    let key = hash2(&hash_r, &hash_x);
                    match trans.get(key) {
                        None => {
                        }
                        Some(tfs) => tfs.into_iter().for_each(|tb|{
                            let (t, bc1) = tb;
                            // let mut bc2 = bc.clone();
                            // bc2.extend(bc1);
                            let path1 = Rc::new(List::Cons(bc1, Rc::clone(&path)));
                            tbc.push((*t,path1));
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
                let result =  go(vec![(hash_init,Rc::new(List::Nil))], trans, finals, &s);
                result
            }
        }
    
    }


    pub fn parse_decode_regex(&self, s:&String) -> Option<U> {
        match self.parse_regex(s) {
            None => None, 
            Some(bv) => Some(decode(&self.init, &bv, s))
        }
    }
}
 
