extern crate combine;

use super::ext::*;


use std::{
    fmt,
    rc::Rc, io, collections::HashSet
};



use combine::{
    parser,
    token,
    choice,
    any,
    unexpected_any,
    many, many1, optional,
    parser::char::{digit,string},
    Parser, Stream,
    parser::{token::value},
    parser::{repeat::sep_by1}, between, attempt, none_of, look_ahead,
};



#[cfg(feature = "std")]
use combine::{
    stream::{easy, position::SourcePosition},
    EasyParser,
};


enum Error<E> {
    Io(io::Error),
    Parse(E),
}


impl<E> fmt::Display for Error<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "{}", err),
            Error::Parse(ref err) => write!(f, "{}", err),
        }
    }
}


/* 
pub enum RParser<Input> 
    where
    Input : Stream<Token = char>
{
    FromBetween(Rc<Between<Input, RParser<Input>, RParser<Input>, RParser<Input>>>)
    FromSepBy1(Rc<SepBy1<>)
}

impl <Input> Parser<Input> for RParser<Input> 
    where
    Input : Stream<Token = char>
{
    type Output = (Ext, Input);
    type PartialState = ();
    fn parse(& mut self, input:Input) -> Result<(Self::Output,Input), Input::Error> {
        match self {
            RParser::FromBetween(btn) => btn.parse(input) 
        }
    }
}
*/


pub fn parse_ext<Input>() -> impl Parser<Input, Output=Ext> 
    where
        Input : Stream<Token = char>
{
    p_ere().map(| ext | {
        ext
    })
}


pub fn p_ere_<Input>() -> impl Parser<Input, Output=Ext> 
    where
        Input : Stream<Token = char>
{
    sep_by1(p_branch(), token('|')).map(|branches|{
        Ext::Or(branches)
    })
    // value(Ext::Empty)
}

// magic macro to "fix" the opaque type issue
parser!{
    fn p_ere[Input]()(Input) -> Ext 
    where 
        [Input : Stream<Token = char>]
    {
        p_ere_()
    }
}


pub fn p_branch<Input>() -> impl Parser<Input, Output = Ext> 
    where 
        Input : Stream<Token = char>
{
   many1(p_exp()).map(|exps| {
        Ext::Concat(exps)
   })
}


pub fn p_exp<Input>() -> impl Parser<Input, Output = Ext> 
    where 
        Input : Stream<Token = char>
{
    choice((p_anchor(), p_atom())).then(|aoa| {
        // won't work, see note Z
        // p_post_anchor_or_atom(aoa)
        p_post_anchor_or_atom().map(move |bound|{
            // apply aoa's clone to bound
            // aoa has Rc components, hence cheap to clone, i.e shallow clone
            match bound {
                PostAnchorOrAtom::Nothing => aoa.clone(),
                PostAnchorOrAtom::Bound(l,h, b) => Ext::Bound(Rc::new(aoa.clone()), l, h, b),
                PostAnchorOrAtom::Opt(b) => Ext::Opt(Rc::new(aoa.clone()), b),
                PostAnchorOrAtom::Plus(b) => Ext::Plus(Rc::new(aoa.clone()), b),
                PostAnchorOrAtom::Star(b) => Ext::Star(Rc::new(aoa.clone()), b)
            }
        })
    })
}

pub fn p_anchor<Input>() -> impl Parser<Input, Output =Ext> 
    where
        Input : Stream<Token = char>
{
    choice((token('^').then( |_s| { value(Ext::Carat)}), 
                token('$').then(|_s| { value(Ext::Dollar)})))
}

pub fn p_atom<Input>() -> impl Parser<Input, Output = Ext> 
    where 
        Input : Stream<Token = char>
{
    choice((p_group(), p_charclass(), p_dot(), p_esc_char(), p_char()))
}


pub fn p_question_mark<Input>() -> impl Parser<Input, Output=char> 
    where 
        Input : Stream<Token= char>
{
    token('?')
}

pub fn p_colon<Input>() -> impl Parser<Input, Output=char> 
    where 
        Input : Stream<Token= char>
{
    token(':')
}




pub fn p_group<Input>() -> impl Parser<Input, Output = Ext>
    where 
        Input : Stream<Token = char>
{
    let non_marking = (
        p_question_mark::<Input>().then(| _qm | {
            p_colon::<Input>().then(| _cl | {
                p_ere()
            })
        })
    ).map( | ext| { 
        ext
    });
    let b = between(token('('), token(')'), choice((
        attempt(non_marking),
        p_ere().then(|x|{value(Ext::Grp(Rc::new(x)))})
    )));
    b
}




pub fn p_charclass<Input>() -> impl Parser<Input, Output = Ext>
    where 
        Input : Stream<Token = char>
{
    p_lbracket().then(| _lb | {
        choice((
            token('^').then( | _car| {
                p_enum().then( | x | { value(Ext::NoneOf(x))})

            })
            , p_enum().then( | x | { value(Ext::Any(x))})
        ))
    })
}


pub fn p_lbracket<Input>() -> impl Parser<Input, Output=char> 
    where 
        Input : Stream<Token= char>
{
    token('[')
}


pub fn p_rbracket<Input>() -> impl Parser<Input, Output=char> 
    where 
        Input : Stream<Token= char>
{
    token(']')
}


// enum ends with ']'
pub fn p_enum<Input>() -> impl Parser<Input, Output = HashSet<char>> 
    where 
        Input : Stream<Token = char>
{
    
    let p_initial_inner = choice((p_rbracket(), token('-')));
        
    let p_initial = optional(p_initial_inner).then(|oi| {
        match oi {
            None => value(vec![]),
            Some(v) => value(vec![v]) 
        }
    });
    
    let p = p_initial.then(|initial:Vec<char>| {
        many1::<HashSet<_>,_,_>(p_one_enum()).then(move |cs|{
            token(']').with({
                let mut char_set:HashSet<char> = HashSet::new();
                char_set.extend(initial.clone().into_iter());
                for c in &cs {
                    let cc = c.clone();
                    char_set.extend(cc.iter());
                }
                value(char_set)
            })
        })
    });
    p
}

pub fn p_one_enum<Input>() -> impl Parser<Input, Output =Vec<char>> 
    where
        Input : Stream<Token = char>
{
    choice((p_range(), p_char_set()))
}

pub fn p_range<Input>() -> impl Parser<Input, Output = Vec<char>> 
    where 
        Input : Stream<Token = char>
{
    attempt( 
        choice((attempt(p_esc_char_()), none_of(vec![']']))).then(|start|{
            token('-').with(choice((attempt(p_esc_char_()), none_of(vec![']']))).then(move |end|{
                let r:Vec<char> = (start .. end).collect();
                value(r)
            }))
        })
    )
    
}


pub fn p_char_set<Input>() -> impl Parser<Input, Output = Vec<char>>
    where
        Input : Stream<Token = char>
{
    choice((attempt(p_esc_char_()), none_of(vec![']']))).then(|c| {
        match c {
            '-' => {
                // it was 
                /* 
                choice((look_ahead(token(']')).with(value(true)), value(false))).then(|at_end| {
                    if !at_end {
                        
                        unexpected_any("p_char_set failed: a dash is in the wrong place in a bracket.")
                    } else {
                        value(vec![c])
                    }
                })
                */
                choice((look_ahead(token(']')).with(value(vec![c])), unexpected_any("p_char_set failed: a dash is in the wrong place in a bracket."))).left()
            },
            _ =>  value(vec![c]).right()
        }
    })
}
 

pub fn p_dot<Input>() -> impl Parser<Input, Output = Ext>
    where 
        Input : Stream<Token = char> 
{
    token('.').with(
        value(Ext::Dot) 
    )
}


pub fn p_esc_char<Input>() -> impl Parser<Input, Output = Ext>
    where 
        Input : Stream<Token = char> 
{
    token('\\').with(
        choice(
            (attempt(p_tab())
                ,attempt(p_return())
                ,attempt(p_newline())
                ,attempt(p_oct_ascii())
                ,any())).then(|c|{
                    value(Ext::Escape(c))
                })
    )
}


pub fn p_esc_char_<Input>() -> impl Parser<Input, Output = char>
    where
        Input : Stream<Token = char>
{
    token('\\').with(
        choice(
            (attempt(p_tab())
                ,attempt(p_return())
                ,attempt(p_newline())
                ,attempt(p_oct_ascii())
                ,any()))
    )
}

pub fn p_tab<Input>() -> impl Parser<Input, Output = char>
    where 
        Input : Stream<Token = char> 
{
    token('t').with(value('\t'))
}

pub fn p_return<Input>() -> impl Parser<Input, Output = char>
    where 
        Input : Stream<Token = char> 
{
    token('r').with(value('\r'))
}

pub fn p_newline<Input>() -> impl Parser<Input, Output = char>
    where 
        Input : Stream<Token = char>
{
    token('n').with(value('\n'))
}
    

pub fn p_oct_ascii<Input>() -> impl Parser<Input, Output = char>
    where 
        Input : Stream<Token = char>
{
    
    digit().with(
        digit().then(|d2| 
        { digit().then(move |d3| 
            {
                let i2 = d2.to_digit(10).expect("this should not happen. p_oct_ascii expect a digit, but it's not.");
                let i3 = d3.to_digit(10).expect("this should not happen. p_oct_ascii expect a digit, but it's not.");
                let c = char::from_digit(i2 * 8 + i3, 10).expect("this shot not happen. p_oct_ascii failed to convert an ascii into to char.");
                // todo wrap the above into the parser error instead of panic
                value(c)
            })
        }
    ))
}
    


pub fn p_char<Input>() -> impl Parser<Input, Output = Ext>
    where 
        Input : Stream<Token = char> 
{
    let specials:Vec<char> = "^.[$()|*+?{\\".chars().collect();
    none_of(specials).then(|c| {
        value(Ext::Char(c))
    })
}

#[derive(Clone)]
pub enum PostAnchorOrAtom {
    Opt(bool),
    Plus(bool),
    Star(bool),
    Bound(u64, Option<u64>, bool),
    Nothing
}


pub fn p_post_anchor_or_atom<Input>() -> impl Parser <Input, Output = PostAnchorOrAtom> 
    where 
        Input : Stream<Token = char> 
{
    value(PostAnchorOrAtom::Nothing) // todo
}

pub fn p_bound_non_greedy<Input>() -> impl Parser<Input, Output = PostAnchorOrAtom>
    where
        Input : Stream<Token = char>
{
    attempt(between(token('{'), string("}?"), p_bound_spec()).then(|(low, hi)|{
        value(PostAnchorOrAtom::Bound(low,hi,false))
    }))
}

pub fn p_bound_spec<Input>() -> impl Parser<Input, Output = (u64, Option<u64>)>
    where
        Input : Stream<Token = char>
{
    many1::<String, Input,_>(digit()).then(|low_s|{
        let low_res = low_s.parse::<u64>();
        match low_res {
            Ok(low) => {
                choice((attempt(token(',').with(
                    many::<String,Input,_>(digit()).then(move |high_s| {
                        let high_res = high_s.parse::<u64>();
                        match high_res {
                            Ok(high) => {
                                if low <= high {
                                    value((low, Some(high)))
                                } else {
                                    value((low, Some(low)))
                                }        
                            },
                            Err(e) => {
                                value((low, None))
                            }
                        }
                    })
                )), value((low, None)))).left()
            }
            Err(e) => {
                unexpected_any("p_bound_spec failed: a dash is in the wrong place in a bracket.").right()
            }
        }

    })
}


// NOTE: Z
// the following won't compile unless Ext impl Copy, because atom is constructed before the postfix is parsed.
// the parsers have to carry around the atom value every where, its life time is unknown.

/*
pub fn p_post_anchor_or_atom<Input>(atom:Ext) -> impl Parser<Input, Output = Ext>
    where
        Input : Stream<Token = char> 
{
    value(atom) // todo
}


pub fn p_bound_non_greedy<Input>(atom:Ext) -> impl Parser<Input, Output= Ext> 
    where
        Input : Stream<Token = char> 
{
    attempt(between(token('{'), string("}?"), p_bound_spec(atom, false)))
}


pub fn p_bound_spec<'a, Input>(atom:Ext, b:bool) -> impl Parser<Input, Output = Ext>
    where
        Input : Stream<Token = char>
{
    let atom_r = &atom;
    many1::<Vec<char>,Input,_>(digit()).then(move |low_v|{
        let low_s:String = low_v.into_iter().collect();
        let low_res = low_s.parse::<u64>();
        match low_res {
            Ok(low) => {
                choice((attempt(token(',').with(
                    many::<Vec<char>,Input,_>(digit()).then(move |high_v| {
                        if high_v.len() > 0 {
                            let high_s:String = high_v.into_iter().collect();
                            let high_res = high_s.parse::<u64>();
                            match high_res {
                                Ok(high) => {
                                    if low <= high {
                                        value(Ext::Bound(atom_r.clone().into(), low, Some(high), b))
                                    } else {
                                        value(Ext::Bound(atom_r.clone().into(), low, Some(low), b))
                                    }        
                                },
                                Err(e) => {
                                    value(Ext::Bound(atom_r.clone().into(), low, Some(low), b))
                                }
                            }
                        } else {
                            value(Ext::Bound(atom_r.clone().into(), low, None, b))
                        }
                    })
                )), value(Ext::Bound(atom_r.clone().into(),low,Some(low),b )))).left()
            }
            Err(e) => {
                unexpected_any("p_bound_spec failed: a dash is in the wrong place in a bracket.").right()
            }
        }

    })
}

 */


/* 
extern crate nom;
use nom::{
  IResult,
  bytes::complete::{tag, take_while_m_n},
  combinator::map_res,
  branch::alt,
  sequence::tuple, multi::{separated_list1, many1}
};

use super::ext::*;


pub fn parse_ext(input:&str) -> IResult<&str, Ext> {
    let (input,e) = p_ere(input)?;
    Ok((input, Ext::Empty))// fixme
}

pub fn p_ere(input:&str) -> IResult<&str, Ext> {
    let (input, branches) = separated_list1(tag("|"), p_branch)(input)?;
    if branches.len() == 0 {
        Ok((input, Ext::Empty))
    } else {
        Ok((input, Ext::Or(branches)))
    }
}

pub fn p_branch(input:&str) -> IResult<&str, Ext> {
    let (input, branches) = many1(p_exp)(input)?;
    if branches.len() == 0 {
        Ok((input, Ext::Empty))
    } else {
        Ok((input, Ext::Concat(branches)))
    }
}

pub fn p_exp(input:&str) -> IResult<&str, Ext> {
    (alt((p_anchor, p_atom)).and_then(p_post_anchor_or_atom))(input) // not sure whether alt is always backtracking or not
}



pub fn p_anchor(input:&str) -> IResult<&str, Ext> {
    alt((p_carat, p_dollar))(input)
}

pub fn p_carat(input:&str) -> IResult<&str, Ext> {
    let (input, _) = tag("^")(input)?;
    Ok((input, Ext::Carat))
}

pub fn p_dollar(input:&str) -> IResult<&str, Ext> {
    let (input, _) = tag("$")(input)?;
    Ok((input, Ext::Dollar))
}

pub fn p_atom(input:&str) -> IResult<&str, Ext> {
    alt((p_group, p_charclass, p_dot, p_esc_char, p_char))(input)
}

pub fn p_group(input:&str) -> IResult<&str, Ext> {

}
*/