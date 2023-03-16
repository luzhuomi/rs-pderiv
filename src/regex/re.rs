#[derive(Debug)]
pub enum RE {
    Eps,
    Lit(char),
    Seq(Box<RE>,Box<RE>),
    Choice(Box<RE>,Box<RE>),
    Star(Box<RE>),
}

pub fn pderiv(r:&RE, l:&char) -> Vec<RE> {
    match r {
        RE::Eps => Vec::new(),
        RE::Lit(m) => { 
            if l == m { Vec::from([RE::Eps])} else { Vec::new() }
        }
        RE::Seq(r1, r2) => { 
            Vec::new() // fixme
        }
        RE::Choice(r1,r2) => {
            Vec::new() // fixme
        },
        RE::Star(r) => {
            Vec::new() // fixme
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