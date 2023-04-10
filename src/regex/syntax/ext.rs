/**
 * external syntax tree
 */
use std::{rc::Rc, collections::HashSet};

#[derive(Debug, PartialEq)]
pub enum Ext {
    Empty,
    GrpNonMarking(Rc<Ext>),
    Grp(Rc<Ext>),
    Or(Vec<Ext>),
    Concat(Vec<Ext>),
    Opt(Rc<Ext>, bool),
    Plus(Rc<Ext>, bool),
    Star(Rc<Ext>, bool),
    Bound(Rc<Ext>, u64, Option<u64>, bool), // inner regex, lower bound, uppper bound, greedy flag
    Carat,
    Dollar,
    Dot,
    Any(HashSet<char>),
    NoneOf(HashSet<char>),
    Escape(char), // escaped character
    Char(char) // non-escaped character
}

impl Clone for Ext {

    fn clone(&self) -> Self {
        match self {
            Ext::Empty => Ext::Empty,
            Ext::GrpNonMarking(ext) => Ext::GrpNonMarking(Rc::clone(ext)),
            Ext::Grp(ext) => Ext::Grp(Rc::clone(ext)),
            Ext::Or(es) => Ext::Or(es.clone()),
            Ext::Concat(es) => Ext:: Concat(es.clone()),
            Ext::Opt(ext, greedy) => Ext::Opt(Rc::clone(ext), greedy.clone()),
            Ext::Plus(ext, greedy) => Ext::Plus(Rc::clone(ext), greedy.clone()),
            Ext::Star(ext, greedy) => Ext::Star(Rc::clone(ext), greedy.clone()),
            Ext::Bound(ext, lb, ub, greedy) => Ext::Bound(Rc::clone(ext), lb.clone(), ub.clone(), greedy.clone()),
            Ext::Carat => Ext::Carat,
            Ext::Dollar => Ext::Dollar,
            Ext::Dot => Ext::Dot,
            Ext::Any(chars) => Ext::Any(chars.clone()),
            Ext::NoneOf(chars) => Ext::NoneOf(chars.clone()),
            Ext::Escape(char) => Ext::Escape(char.clone()),
            Ext::Char(char) => Ext::Char(char.clone())
        }
    }
}