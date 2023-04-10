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
    error::ParseError,
    many, many1, optional,
    parser::char::{char, digit},
    Parser, Stream,
    parser::{token::value},
    parser::{repeat::sep_by1, sequence::Between}, between, attempt,
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
    choice((p_anchor(), p_atom())) 
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
    /* 
    let p_initial_inner = choice((p_rbracket(), token('-')));
        
    let p_initial = optional(p_initial_inner).then(|oi| {
        match oi {
            None => value(vec![]),
            Some(v) => value(vec![v]) 
        }
    });
    
    let p = p_initial.map(|initial| {
        many1::<Vec<_>,_,_>(p_one_enum()).then(move |cs|{
            token(']').then(|_rb| {
                let mut char_set:HashSet<char> = HashSet::new();
                char_set.extend(initial.iter());
                for c in &cs {
                    let cc = c.clone();
                    char_set.extend(cc.iter());
                }
                value(char_set)
            })
        })
    });
    p*/
    value(HashSet::new())
}

pub fn p_one_enum<Input>() -> impl Parser<Input, Output =Vec<char>> 
    where
        Input : Stream<Token = char> 
{
    value(vec![]) // todo
}


pub fn p_dot<Input>() -> impl Parser<Input, Output = Ext>
    where 
        Input : Stream<Token = char> 
{
    value(Ext::Empty) // todo
}


pub fn p_esc_char<Input>() -> impl Parser<Input, Output = Ext>
    where 
        Input : Stream<Token = char> 
{
    value(Ext::Empty) // todo
}



pub fn p_char<Input>() -> impl Parser<Input, Output = Ext>
    where 
        Input : Stream<Token = char> 
{
    value(Ext::Empty) // todo
}

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