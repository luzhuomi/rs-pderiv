//! # rs_pderiv
//!
//! `rs_pderiv` is a regular expression matching library that utilizes partial derivative operation.
//! The current implementation supports 
//! 1. compiling a regular expression into a transducer, where regex PDs are states and partial derivative operations are the transitions. Each transitions yields a bit representing the index to the production rule of the underlying regular grammar.
//! 2. matching a transducer against an input string will yields a parse tree.

#![recursion_limit = "256"]
pub mod regex;
pub mod test;
use std::result::Result;
use combine::Parser;
use regex::pderiv::parsetree::U;
use regex::re::*;
use regex::syntax::parser::*;
use regex::syntax::desugar::*;
use regex::pderiv::parse::*;





pub struct CompiledRegex {
    pub re : RE,
    pub regex : Regex
}

/// Compile a regex str into a regex
/// # Examples
/// ```
/// let s = "[a-zA-Z]{0,2}";
/// let o = rs_pderiv::compile(s); 
/// let regex = match o 
/// {
///     Err(err) => panic!("compilation failed."),
///     Ok(compiled) => compiled
/// };
/// ```


pub fn compile<'a>(s:&str) -> Result<CompiledRegex, String> {
    let parse_result = parse_ext().parse(s); 
    match parse_result {
        Err(e) => Err(e.to_string()),
        Ok((ext_ast, _rest)) => {
            let simp = simp_ext(&ext_ast);
            let desugared = ext_to_re(&simp);
            match desugared {
                Err(e) => Err(e),
                Ok(re) => {
                    let compiled = build_regex(&re);
                    Ok(CompiledRegex{re:re, regex:compiled})
                }
            }
        }
    }
}

/// parse a string with a compiled Regex
/// # Examples
/// ```
/// use rs_pderiv::regex::pderiv::parsetree::U::{self, *};
/// let s = "[sg]{0,2}";
/// let o = rs_pderiv::compile(s);
/// let regex = match o 
/// {
///     Err(err) => panic!("compilation failed."),
///     Ok(compiled) => compiled
/// };
/// println!("{:?}", regex.re);
/// let input = String::from("sg");
/// match regex.parse(&input) {
///     Some(u) => assert_eq!(u, U::PairU(Box::new(U::LitU('s')), Box::new(U::LitU('g')))),
///     None => panic!("matched failed, no parse tree generated.")
/// }
/// ```
impl CompiledRegex {

    pub fn parse(&self, input:&String) -> Option<U> {
        self.regex.parse_decode_regex(input)
    } 
}
