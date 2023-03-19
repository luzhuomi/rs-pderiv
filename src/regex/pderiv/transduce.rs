
use std::collections::HashMap;
use std::collections::HashSet;
use bitvec::prelude::*;
use super::super::re::*;
use super::bits::*;
// use super::parsetree::*;

type Trans = HashMap<(RE,char), Vec<(RE, BitVec)>>;
type Finals = HashMap<RE, BitVec>;


/**
 * TODO: check whether the .clones() are necessary
 */
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




pub fn parse_regex(regex:Regex, s:String) -> Option<BitVec> {
    fn go(rbc:Vec<(&RE,BitVec)>, trans:&Trans, finals:Finals, s:&str) -> Vec<BitVec> {
        if s.len() == 0 {
            let mut res = vec![];
            rbc.iter().for_each(|x| {
                let (r, bc) = x;
                if r.nullable() { // finals is useless.
                    let mut bc1 = bc.clone();
                    bc1.extend(emp_code(r));
                    res.push(bc.clone());
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
            let mut tbc = vec![];
            rbc.iter().for_each(|rb| {
                let (r, bc) = rb.clone();
                let key = (r.clone(),x.clone());
                match trans.get(&key) {
                    None => {}
                    Some(tfs) => tfs.iter().for_each(|tb|{
                        let (t, bc1) = tb;
                        let mut bc2 = bc.clone();
                        bc2.extend(bc1);
                        tbc.push((t,bc2.clone()));
                    })
                };
            });
            go(tbc, trans, finals, xs)
        }
    }

    match regex {
        Regex { trans, init, finals } => {
            let result =  go(vec![(&init,BitVec::new())], &trans, finals, &s);
            if result.len() == 0 {
                None
            } else {
                Some(result[0].clone())
            }
        }
    }

}