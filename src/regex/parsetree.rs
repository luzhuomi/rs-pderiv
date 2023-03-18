use super::re::*;

pub enum U {
    EpsU,
    LitU(char),
    Pair(Box<U>,Box<U>),
    Left(Box<U>),
    Right(Box<U>),
    ListU(Vec<U>)
}


