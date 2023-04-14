
use super::ext::*;
use super::super::re::*;

// convert an external regex syntax to internal re syntax
// for now the greedy flags are ignored.
pub fn ext_to_re(e:&Ext) -> Result<RE,String> {
    match e {
        Ext::Empty => Ok(RE::Eps),
        Ext::GrpNonMarking(d) => ext_to_re(d),
        Ext::Grp(d) => ext_to_re(d),
        Ext::Or(es) => {
            let mut re = RE::Phi;
            for d in es.iter().rev().map(ext_to_re) {
                if re == RE::Phi {
                    re = d
                } else {
                    re = choice!(d,re)
                }
            }
            re
        },
        Ext::Concat(es) => {
            let mut re = RE::Eps;
            for d in es.iter().rev().map(ext_to_re) {
                if re == RE::Eps {
                    re = d 
                } else {
                    re = seq!(d,re)
                }
            }
            re
        },
        Ext::Opt(e, greedy) => { 
            let re = ext_to_re(e);
            choice!(re, RE::Eps)
        },
        Ext::Plus(e, greedy) => {
            let re = ext_to_re(e);
            let re_clone = ext_to_re(e);
            seq!(re, star!(re_clone))
        },
        Ext::Star(e, greedy) => {
            let re = ext_to_re(e);
            star!(re)
        },
        Ext::Bound(e, lb, None, greedy) => {
            let mut re = star!(ext_to_re(e));
            for i in 0..*lb {
                let re2 = ext_to_re(e);
                re = seq!(re2, re)
            }
            re
        },
        Ext::Bound(e, lb, Some(ub), greedy) => {
            let mut re = RE::Phi;
            for i in 0..*lb {
                let re2 = ext_to_re(e);
                if re == RE::Phi {
                    re = re2;
                } else {
                    re = seq!(re2, re)
                }
            }
            for j in *lb..*ub {
                let re3 = choice!(ext_to_re(e), RE::Eps);
                re = seq!(re, re3)
            }
            re
        },
        
    }
}