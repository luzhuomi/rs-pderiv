
use std::collections::HashMap;
use std::collections::HashSet;
use bitvec::prelude::*;
use super::super::re::*;
use super::bits::*;
use super::parsetree::*;

type Trans = HashMap<(RE,char), Vec<(RE, BitVec)>>;
type Finals = HashMap<RE, BitVec>;
type Init = RE;

pub struct Regex {
    trans:Trans, 
    init: RE, 
    finals: Finals 
}


fn build_fix(all_states_sofar: Vec<RE>, curr_trans:Trans, sig:HashSet<char>) -> (Vec<RE>, Trans) {
    let new_delta_it = all_states_sofar.iter().map(
        |r| {
            let e = sig.iter().map(move |l| {
                let rp = r.clone();
                let tfs = pderiv_bc(&rp, l);
                let tfdelta = tfs.iter().map(
                    |tbc | {
                        let (t,bc) = tbc;
                        (r,l.clone(),t.clone(),bc.clone())
                    }
                );
                tfdelta.collect::<Vec<_>>()
            });
            e
        });
    let new_delta :Vec<_>= new_delta_it.fold(vec![], |mut v, it| {
        for i in it {
            for j in i.iter() {
                if !curr_trans.contains_key(&(j.0.clone(),j.1)) {
                    v.push(j.clone())
                }
            }
        }
        v
    });
    if new_delta.len() == 0 {
        (all_states_sofar, curr_trans)
    } else {
        let mut all_states_next = all_states_sofar.clone();
        new_delta.iter().fold(& mut all_states_next, |acc, t| {
            acc.push(t.2.clone());
            acc
        });
        let mut next_trans = curr_trans.clone();
        new_delta.iter().fold(& mut next_trans, |acc, t| {
            let key = (t.0.clone(), t.1);
            let _ = match acc.get(&key) {
                None => acc.insert(key, vec![(t.2.clone(),t.3.clone())]),
                Some(vec1) => {
                    let mut vec2 = vec1.clone();
                    vec2.push((t.2.clone(),t.3.clone()));
                    acc.insert(key, vec2)
                }
            };
            acc
        });
        (all_states_next,next_trans)
    }
}

pub fn build_regex(r:&RE) -> Regex {
    let sig = r.sigma();
    let init_states = vec![r.clone()];
    let (all_states, trans) = build_fix(init_states, HashMap::new(), sig);
    let fins = all_states.iter().filter(|r|{r.nullable()}).map(|r|{
        (r.clone(), emp_code(r))
    }).collect();
    Regex{trans : trans, init : r.clone(), finals:fins}
}


