
use std::collections::HashMap;
use bitvec::prelude::*;
use super::super::re::*;
use super::bits::*;
use super::parsetree::*;

type ITrans = HashMap<(RE,char), Vec<(RE, BitVec)>>;
type Finals = HashMap<RE, BitVec>;
type Init = RE;

pub struct Regex ( ITrans, Init, Finals );


/* 
pub fn buildRegex(r:&RE) -> Regex {
    let sig = r.sigma();
}
*/
