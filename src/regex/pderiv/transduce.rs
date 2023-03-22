
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


fn build_fix(all_states_sofar: Vec<RE>, curr_trans:Trans, sig:HashSet<char>) -> (Vec<RE>, Trans) {
    let new_delta = all_states_sofar.iter().map(
        |r| {
            let e = sig.iter().map(move |l| {
                let tfs = pderiv_bc(r, l);
                let tfdelta = tfs.into_iter().map(
                    |tbc | {
                        let (t,bv) = tbc;
                        (r.clone(),*l,t, bv)
                    }
                );
                tfdelta.collect::<Vec<_>>()
            });
            e
        }).fold(vec![], |acc, it| {
            it.fold(acc, |acc2, i|{
                i.into_iter().fold(acc2, |mut acc3, (src, c, dst, bv)| {
                    if !(curr_trans.contains_key(&(src.clone(),c))) {
                        acc3.push((src,c,dst,bv));
                    }
                    acc3
                })
            })
           
        });
    if new_delta.len() == 0 {
        (all_states_sofar, curr_trans)
    } else {
        let all_states_next = new_delta.iter().fold(all_states_sofar, |mut acc, t| {
            acc.push(t.2.clone());
            acc
        });
        let next_trans = new_delta.into_iter().fold(curr_trans, |mut acc, t| {
            let (src, c, dst, bv) = t;
            let key = (src.clone(), c.clone()); 
            match acc.get_mut(&key) {
                None => {
                    acc.insert(key, vec![(dst.clone(),bv.clone())]);
                    acc
                }
                Some(vec1) => {
                    vec1.push((dst.clone(),bv.clone()));
                    // acc.insert(key, vec1) // no need?
                    acc
                }
            }
        });
        build_fix(all_states_next,next_trans, sig)
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