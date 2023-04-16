

///
/// 
#[derive(Debug, PartialEq)]
pub enum U {
    NilU,
    LitU(char),
    PairU(Box<U>,Box<U>),
    LeftU(Box<U>),
    RightU(Box<U>),
    ListU(Vec<U>)
}


