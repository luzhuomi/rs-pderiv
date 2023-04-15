
use std::collections::HashSet;
use std::rc::Rc;

use super::ext::*;
use super::super::re::*;


// simplify ext expression
pub fn simp_ext(e:&Ext) -> Ext {
    match e {
        Ext::Empty => Ext::Empty,
        Ext::GrpNonMarking(d) => Ext::GrpNonMarking(Rc::new(simp_ext(d))),
        Ext::Grp(d) => Ext::GrpNonMarking(Rc::new(simp_ext(d))),
        Ext::Or(es) => {
            if es.len() == 0 {
                Ext::Empty
            } else if es.len() == 1 {
                simp_ext(&(es[0]).clone())
            } else {
                let ds = es.iter().map(|d|{simp_ext(d)}).collect();
                Ext::Or(ds)
            }
        },
        Ext::Concat(es) => {
            if es.len() == 0 {
                e.clone()
            } else if es.len() == 1 {
                simp_ext(&(es[0]).clone())
            } else {
                let ds = es.iter().map(|d|{simp_ext(d)}).collect();
                Ext::Concat(ds)
            }
        },
        Ext::Opt(e, greedy) => {
            let d = simp_ext(e);
            Ext::Opt(Rc::new(d), greedy.clone())
        },
        Ext::Plus(e,greedy) => {
            let d = simp_ext(e);
            Ext::Plus(Rc::new(d), greedy.clone())
        },
        Ext::Star(e, greedy) => {
            let d = simp_ext(e);
            Ext::Star(Rc::new(d), greedy.clone())
        },
        Ext::Bound(e, lb , oub ,greedy ) => {
            let d = simp_ext(e);
            Ext::Bound(Rc::new(d), *lb, oub.clone(), greedy.clone())
        },
        Ext::Carat => Ext::Carat,
        Ext::Dollar => Ext::Dollar,
        Ext::Dot => Ext::Dot,
        Ext::Any(cs) => Ext::Any(cs.clone()),
        Ext::NoneOf(cs) => Ext::NoneOf(cs.clone()),
        Ext::Escape(c) => Ext::Escape(*c),
        Ext::Char(c) => Ext::Char(*c)
    }
}

// convert an external regex syntax to internal re syntax
// for now the greedy flags are ignored.
pub fn ext_to_re(e:&Ext) -> Result<RE,String> {
    match e {
        Ext::Empty => Ok(RE::Eps),
        Ext::GrpNonMarking(d) => ext_to_re(d),
        Ext::Grp(d) => ext_to_re(d),
        Ext::Or(es) => {
            let mut re = RE::Phi;
            for d in es.iter().rev().flat_map(ext_to_re) {
                if re == RE::Phi {
                    re = d
                } else {
                    re = choice!(d,re)
                }
            }
            Ok(re)
        },
        Ext::Concat(es) => {
            let mut re = RE::Eps;
            for d in es.iter().rev().flat_map(ext_to_re) {
                if re == RE::Eps {
                    re = d 
                } else {
                    re = seq!(d,re)
                }
            }
            Ok(re)
        },
        Ext::Opt(e, greedy) => { 
            let re = ext_to_re(e)?;
            Ok(choice!(re, RE::Eps))
        },
        Ext::Plus(e, greedy) => {
            let re = ext_to_re(e)?;
            let re_clone = re.clone();
            Ok(seq!(re, star!(re_clone)))
        },
        Ext::Star(e, greedy) => {
            let re = ext_to_re(e)?;
            Ok(star!(re))
        },
        Ext::Bound(e, lb, None, greedy) => {
            let mut re = star!(ext_to_re(e)?);
            for i in 0..*lb {
                let re2 = ext_to_re(e)?;
                re = seq!(re2, re)
            }
            Ok(re)
        },
        Ext::Bound(e, lb, Some(ub), greedy) => {
            let mut re = RE::Phi;
            for i in 0..*lb {
                let re2 = ext_to_re(e)?;
                if re == RE::Phi {
                    re = re2;
                } else {
                    re = seq!(re2, re)
                }
            }
            for j in *lb..*ub {
                let re3 = choice!(ext_to_re(e)?, RE::Eps);
                re = seq!(re, re3)
            }
            Ok(re)
        },
        Ext::Carat => Err("ext_to_re:encountered carat.".to_owned()),
        Ext::Dollar => Err("ext_to_re:encountered dollar.".to_owned()),
        Ext::Dot => {
            let all_chars:Vec<Ext> = (0..127).into_iter().map(|i|{Ext::Char(char::from(i))}).collect();
            ext_to_re(&Ext::Or(all_chars))
        }
        Ext::Any(chars) => {
            let lits = chars.iter().map(|c| {Ext::Char(*c)}).collect();
            ext_to_re(&Ext::Or(lits))
        },
        Ext::NoneOf(chars) => {
            let all_chars:Vec<Ext> = (0..127).into_iter().flat_map(|i|{
                let char = char::from(i);
                if chars.contains(&char) {
                    None
                } else {
                    Some(Ext::Char(char))
                }
                }).collect();
            ext_to_re(&Ext::Or(all_chars))
        },
        Ext::Escape(c) => Ok(RE::Lit(*c)),
        Ext::Char(c) => Ok(RE::Lit(*c))
    }
}