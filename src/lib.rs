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

// compile a regex str into a re AST
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

// parse a string with a compiled Regex
impl CompiledRegex {
    pub fn parse(&self, input:&String) -> Option<U> {
        self.regex.parse_decode_regex(input)
    } 
}
