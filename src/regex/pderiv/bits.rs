

use bitvec::prelude::*;
use intmap::IntMap;
use super::super::re::*;
use super::parsetree::*;
use std::rc::Rc;

/**
 * precond: r is nullable
 */
pub fn emp_code(r:&RE) -> BitVec {
    match r {
        RE::Eps => bitvec![],
        RE::Choice(r1,r2) => {
            if r1.nullable() {
                let mut bv = emp_code(r1);
                bv.insert(0,false);
                bv
            } else { 
                if r2.nullable() {
                    let mut bv = emp_code(r2);
                    bv.insert(0,true);
                    bv
                } else {
                    panic!("emp_code failed, both alteratives of the given regex are not nullable.")
                }
            }
        }
        RE::Seq(r1, r2) => {
            let mut bv1 = emp_code(r1);
            let mut bv2 = emp_code(r2);
            bv1.append(& mut bv2);
            bv1
        }, 
        RE::Star(_r1) => {
            bitvec![1]
        },
        _ => panic!("emp_code failed, the given regex is not nullable.")
    }
}

pub fn pderiv_bc(r:&RE, l:&char) -> Vec<(RE,BitVec)> {
    match r {
        RE::Phi => vec![], 
        RE::Eps => vec![], 
        RE::Lit(m) => {
            if l == m {
                vec![(RE::Eps, bitvec![])]
            } else {
                vec![]
            }
        },
        RE::Seq(r1, r2) => {
            if r1.nullable() {
                let ts = pderiv_bc(r1, l);
                let vs = pderiv_bc(r2, l);
                let mut res = vec![];
                for (t,bv) in ts {
                    res.push((RE::Seq(Rc::new(t), Rc::clone(r2)), bv))
                }
                for (v, mut bu) in vs {
                    let mut emp = emp_code(r1);
                    emp.append(& mut bu);                    
                    res.push((v, emp));
                }
                nub_vec_fst(res)
            } else {
                let ts = pderiv_bc(r1, l);
                let mut res = vec![];
                for (t,bv) in ts {
                    res.push((RE::Seq(Rc::new(t), Rc::clone(r2)), bv))
                }
                nub_vec_fst(res)
            }
        },
        RE::Choice(r1,r2) => {
            let ts = pderiv_bc(r1, l);
            let vs = pderiv_bc(r2, l);
            let mut res = vec![];
            for (t,bv) in ts{
                let mut bv1 = bv;
                bv1.insert(0, false);
                res.push((t, bv1))
            }
            for (v, bu) in vs {
                let mut bu1 = bu;
                bu1.insert(0, true);
                res.push((v, bu1));
            }
            nub_vec_fst(res)
        },
        RE::Star(r1) => {
            let ts = pderiv_bc(r1,l);
            let mut res = vec![];
            for (t, bv) in ts {
                let mut bv1 = bv;
                bv1.insert(0, false);
                res.push((RE::Seq(Rc::new(t),Rc::new(r.clone())), bv1))
            }
            nub_vec_fst(res)
        }
    }
}



pub struct PDCached {
    pub cached: IntMap<Vec<(RE,BitVec)>>
}

impl PDCached {
    pub fn new() -> Self {
        PDCached { 
            cached:IntMap::new()
        }
    }
    pub fn pderiv_bc(&mut self, r:&RE, l:&char) -> Vec<(RE,BitVec)> {
        let hash_r = calculate_hash(r);
        let hash_l = calculate_hash(l);
        let key = hash2(&hash_r, &hash_l);
        match self.cached.get(key) {
            Some(v) => v.clone(),
            None => {
                match r {
                    RE::Phi => { 
                        let v = vec![];
                        self.cached.insert(key, v.clone());
                        v
                    } 
                    RE::Eps => {
                        let v = vec![];
                        self.cached.insert(key, v.clone());
                        v
                    } 
                    RE::Lit(m) => {
                        if l == m {
                            let v = vec![(RE::Eps, bitvec![])];
                            self.cached.insert(key, v.clone());
                            v
                        } else {
                            let v = vec![];
                            self.cached.insert(key, v.clone());
                            v
                        }
                    },
                    RE::Seq(r1, r2) => {
                        if r1.nullable() {
                            let ts = self.pderiv_bc(r1, l);
                            let vs = self.pderiv_bc(r2, l);
                            let res:Vec<(RE,BitVec)> = ts.into_iter().map(|(t,bv)|{
                                (RE::Seq(Rc::new(t), Rc::clone(r2)), bv)
                            }
                            ).chain(
                                vs.into_iter().map(|(v,mut bu)|{
                                    let mut emp = emp_code(r1);
                                    emp.append(& mut bu);     
                                    (v,emp)
                                })
                            ).collect();
                            let nubbed = nub_vec_fst(res);
                            self.cached.insert(key, nubbed.clone());
                            nubbed
                        } else {
                            let ts = self.pderiv_bc(r1, l);
                            let res : Vec<(RE, BitVec)> = ts.into_iter().map(|(t,bv)| {
                                (RE::Seq(Rc::new(t), Rc::clone(r2)), bv)
                            }).collect();
                            let nubbed = nub_vec_fst(res);
                            self.cached.insert(key, nubbed.clone());
                            nubbed
                        }
                    },
                    RE::Choice(r1,r2) => {
                        let ts = self.pderiv_bc(r1, l);
                        let vs = self.pderiv_bc(r2, l);
                        let res: Vec<(RE, BitVec)>  = ts.into_iter().map(|(t,bv)|{
                            let mut bv1 = bv;
                            bv1.insert(0, false);
                            (t,bv1)
                        }).chain(vs.into_iter().map(|(v, bu)| {
                            let mut bu1 = bu;
                            bu1.insert(0, true);
                            (v,bu1)
                        })).collect();
                        let nubbed = nub_vec_fst(res);
                        self.cached.insert(key, nubbed.clone());
                        nubbed
                    },
                    RE::Star(r1) => {
                        let ts = self.pderiv_bc(r1,l);
                        let res:Vec<(RE,BitVec)> = ts.into_iter().map( |(t,bv)|{
                            let mut bv1 = bv;
                            bv1.insert(0, false);
                            (RE::Seq(Rc::new(t),Rc::new(r.clone())),bv1)
                        }).collect();
                        let nubbed = nub_vec_fst(res);
                        self.cached.insert(key, nubbed.clone());
                        nubbed
                    }
                }
            }
        }
    }
}



pub fn decode_p<'a>(r:&RE, bs:&'a BitSlice, s:&'a str) -> (U, &'a BitSlice, &'a str) {

    match r {
        RE::Eps => {
            (U::NilU, bs, s)
        },
        RE::Lit(a) => {
            (U::LitU(a.clone()), bs, &s[1..])
        },
        RE::Choice(r1,r2) => {
            if bs[0] == false { // it's 0
                let (u1, bs1, s1) = decode_p(r1, &bs[1..], s);
                (U::LeftU(Box::new(u1)), bs1, s1)
            } else {
                let (u2, bs2, s2) = decode_p(r2, &bs[1..], s);
                (U::RightU(Box::new(u2)), bs2, s2)
            }
        },
        RE::Seq(r1, r2) => {
            let (u1, bs1, s1) = decode_p(r1, bs, s);
            let (u2, bs2, s2) = decode_p(r2, bs1, s1);
            (U::PairU(Box::new(u1),Box::new(u2)), bs2, s2)
        },
        RE::Star(r1) => {
            if bs[0] == true { // it's 1
                (U::ListU(vec![]), &bs[1..], s)
            } else {
                let (u1, bs1, s1) = decode_p(r1, bs, s);
                let (u2, bs2, s2) = decode_p(r, bs1, s1);
                match u2 {
                    U::ListU(mut us) => {
                        us.insert(0,u1);
                        (U::ListU(us), bs2, s2)        
                    }
                    _ => panic!("decode_p failed. A non-list parse tree is returned by a Star RE.")
                }
            }
        },
        RE::Phi => {
            panic!("decode_p failed. A Phi RE is encountered.")
        }
    }
}

pub fn decode(r:&RE, bs:&BitVec, s:&String) -> U {
    match decode_p(r, bs.as_bitslice(), s) {
        (u, bs1, s1) => {
            if bs1.len() == 0 {
                if s1.len() == 0 {
                    u
                } else {
                    panic!("decode failed. A non empty string remaind is returned.")
                }
            } else {
                panic!("decode failed. A non empty bit remainder is returned.")
            }
        }
    }
}