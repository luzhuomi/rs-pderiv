
use std::collections::HashMap;
use std::collections::HashSet;
use bitvec::prelude::*;
use super::super::re::*;
use super::bits::*;
// use super::parsetree::*;

type Trans = HashMap<(RE,char), Vec<(RE, BitVec)>>;
type Finals = HashMap<RE, BitVec>;


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


fn build_fix(all_states_sofar: HashSet<RE>, curr_trans:Trans, sig:HashSet<char>) -> (HashSet<RE>, Trans) {
    // dbg!(all_states_sofar.clone().len());
    // dbg!(curr_trans.clone().len());
    let mut new_delta = all_states_sofar.iter().flat_map(
        |r| {
            let e = sig.iter().filter(|l| {
                let key = (r.clone(), **l);
                !(curr_trans.contains_key(&key))
            }).flat_map(move |l| {
                let tfs = pderiv_bc(r, l);
                // println!("{:?}{:?}",r,l);
                if tfs.len() == 0 {
                    None
                } else {
                    let tfdelta = tfs.into_iter();
                    Some((r,l,tfdelta))    
                }
            });
            e
        }).peekable();
    if new_delta.peek().is_none() { // lowerbound is 0
        (all_states_sofar, curr_trans)
    } else {
        let new_trans :Trans = curr_trans.clone() ;
        let (next_states, next_trans) = new_delta
            .fold((HashSet::new(), new_trans), |acc, t| {
                // let (states_sofar, trans) = acc;
                let (src, c, dstbvs) = t;
                dstbvs.fold(acc, |(mut states_sofar, mut trans), s| {
                    let (dst, bv) = s;
                    let _inserted = states_sofar.insert(dst.clone());
                    let key = (src.clone(), *c);
                    match trans.get_mut(&key) {
                        None => {
                            trans.insert(key, vec![(dst, bv)]);
                            ()
                        }
                        Some(dstbv1) => dstbv1.push((dst, bv))
                    };
                    (states_sofar,trans)
                })
        });
        // dbg!(next_states.clone().len());
        let mut ns = next_states;
        ns.extend(all_states_sofar);
        build_fix(ns,next_trans, sig)
    }
}

pub fn build_regex(r:&RE) -> Regex {
    let sig = r.sigma();
    let init_states = vec![r.clone()].into_iter().collect();
    let (all_states, trans) = build_fix(init_states, HashMap::new(), sig);
    let fins = all_states.iter().filter(|r|{r.nullable()}).map(|r|{
        (r.clone(), emp_code(r))
    }).collect();
    Regex{trans : trans, init : r.clone(), finals:fins}
}




pub fn parse_regex(regex:&Regex, s:&String) -> Option<BitVec> {
    fn go<'a>(rbc:Vec<(&RE,BitVec)>, trans:&Trans, finals:&Finals, s:&str) -> Vec<BitVec> {
        if s.len() == 0 {
            let mut res:Vec<BitVec> = vec![];
            rbc.iter().for_each(|(r, bc)| {
                if r.nullable() { // finals is useless.
                    let mut bc1 = (*bc).clone();
                    bc1.extend(emp_code(r));
                    res.push(bc1);
                } else { // nothing to do here.
                }
            });
            res
        } else {
            let ox = &s[0..1].chars().nth(0);
            let (x,xs) = match ox {
                None => panic!("parse_regex failed, empty string slice with len > 0"),
                Some(c) => (c,&s[1..])
            };
            let mut tbc:Vec<(&RE, BitVec)> = vec![];
            rbc.iter().for_each(|(r,bc)| {
                let key = ((*r).clone(),x.clone());
                match trans.get(&key) {
                    None => {
                    }
                    Some(tfs) => tfs.iter().for_each(|tb|{
                        let (t, bc1) = tb;
                        let mut bc2 = (*bc).clone();
                        bc2.extend(bc1);
                        tbc.push((t,bc2));
                    })
                };
            });
            go(tbc, trans, finals, xs)
        }
    }

    match regex {
        Regex { trans, init, finals } => {
            let result =  go(vec![(&init,BitVec::new())], trans, finals, &s);
            if result.len() == 0 {
                None
            } else {
                let bv: BitVec = (result.clone()[0]).clone();
                Some(bv)
            }
        }
    }

}