extern crate combine;

use super::ext::*;


use std::{
    env, fmt,
    fs::File,
    io::{self, Read},
};

use combine::{
    token,
    choice,
    error::ParseError,
    many, optional,
    parser::char::{char, digit},
    Parser, Stream,
    parser::token::value,
    parser::repeat::{sep_by1, many1},
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

// a parser that that always success

pub fn parse_ext<Input>() -> impl Parser<Input, Output=Ext> 
    where
        Input : Stream<Token = char>
{
    p_ere().map(| ext | {
        ext
    })
}

pub fn p_ere<Input>() -> impl Parser<Input, Output=Ext> 
    where
        Input : Stream<Token = char>
{
    sep_by1(p_branch(), token('|')).map(|branches|{
        Ext::Or(branches)
    })
    // value(Ext::Empty)
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
    choice((token('^').then( |s| { value(Ext::Carat)}), 
                token('$').then(|s| { value(Ext::Dollar)})))
}

pub fn p_atom<Input>() -> impl Parser<Input, Output = Ext> 
    where 
        Input : Stream<Token = char> 
{
    choice((p_group(), p_charclass(), p_dot(), p_esc_char(), p_char()))
}




pub fn p_group<Input>() -> impl Parser<Input, Output = Ext>
    where 
        Input : Stream<Token = char> 
{
    value(Ext::Empty) // todo
}


pub fn p_charclass<Input>() -> impl Parser<Input, Output = Ext>
    where 
        Input : Stream<Token = char> 
{
    value(Ext::Empty) // todo
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