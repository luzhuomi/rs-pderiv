

use bitvec::prelude::*;
use super::re::*;
use super::parsetree::*;


/**
 * precond: r is nullable
 */
pub fn emp_code(r:&RE) -> BitVec {
    match r {
        RE::Eps => bitvec![],
        RE::Choice(r1,r2) => {
            if nullable(r1) {
                let mut bv = emp_code(r1);
                bv.insert(0,false);
                bv
            } else { 
                if nullable(r2) {
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
            if nullable(r1) {
                let ts = pderiv_bc(r1, l);
                let vs = pderiv_bc(r2, l);
                let mut res = vec![];
                for (t,bv) in ts {
                    res.push((RE::Seq(Box::new(t), r2.clone()), bv))
                }
                for (v, mut bu) in vs {
                    let mut emp = emp_code(r1);
                    emp.append(& mut bu);                    
                    res.push((v, emp));
                }
                res
            } else {
                let ts = pderiv_bc(r1, l);
                let mut res = vec![];
                for (t,bv) in ts {
                    res.push((RE::Seq(Box::new(t), r2.clone()), bv))
                }
                res
            }
        },
        RE::Choice(r1,r2) => {
            let mut ts = pderiv_bc(r1, l);
            let mut vs = pderiv_bc(r2, l);
            ts.append(& mut vs);
            ts
        }
        _ =>    Vec::new()
    }
}