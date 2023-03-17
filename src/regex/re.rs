#[derive(Debug)]
pub enum RE {
    Eps,
    Lit(char),
    Seq(Box<RE>,Box<RE>),
    Choice(Box<RE>,Box<RE>),
    Star(Box<RE>),
}

impl Clone for RE {
    
    fn clone(&self) -> Self {
        match self {
            RE::Eps => RE::Eps,
            RE::Lit(l) => RE::Lit(l.clone()),
            RE::Seq(r1, r2) => RE::Seq(r1.clone(), r2.clone()),
            RE::Choice(r1, r2) => RE::Choice(r1.clone(), r2.clone()),
            RE::Star(r) => RE::Star(r.clone())
        }
    }
}


pub fn pderiv(r:&RE, l:&char) -> Vec<Box<RE>> {
    match r {
        RE::Eps => vec![],
        RE::Lit(m) => { 
            if l == m { 
                vec![Box::new(RE::Eps)]
            } else { 
                vec![]
            }
        }
        RE::Seq(r1, r2) => { 
            if nullable(r1) {
                let ts =  pderiv(r1,l);
                let vs =  pderiv(r2,l);
                let mut res = vec![];
                for t in ts {
                    res.push(Box::new(RE::Seq(t, r2.clone())));                    
                };
                for v in vs {
                    res.push(v);
                }
                res

            } else {
                let ts = pderiv(r1,l);
                let mut res = Vec::new();
                for t in ts {
                    res.push(Box::new(RE::Seq(t, r2.clone())));
                };
                res    
            }
        }
        RE::Choice(r1,r2) => {
            let ts = pderiv(r1,l);
            let vs = pderiv(r2,l);
            let mut res = vec![];
            for t in ts { res.push(t) };
            for v in vs { res.push(v) };
            res
        },
        RE::Star(r1) => {
            let ts = pderiv(r1, l);
            let mut res = vec![];
            for t in ts {
                res.push(Box::new(RE::Seq(t, Box::new(r.clone()))));    
            }
            res
        }
        
    }
}

pub fn nullable(r:&RE) -> bool {
    match r {
        RE::Eps => true,
        RE::Star(_) => true,
        RE::Choice(r1,r2) => nullable(r1) || nullable(r2),
        RE::Seq(r1,r2) => nullable(r1) && nullable(r2),
        RE::Lit(_) => false
    }
}