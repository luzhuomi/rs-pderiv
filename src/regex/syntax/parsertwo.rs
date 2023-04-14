extern crate combine;
use combine::{
    between, choice, many, many1, optional, parser, satisfy, sep_by, token, Parser, Stream, ParseError, StdParseResult, parser::char::digit
};
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::rc::Rc;

use super::ext::Ext; // Assuming your AST definition is in another module

/* 
lazy_static! {
    static ref REGEX_SPECIAL_CHARS: HashSet<char> = ['^', '$', '.', '*', '+', '?', '|', '(', ')', '[', ']', '{', '}', '\\', '-', ':'].iter().cloned().collect();
}

fn is_special_char(c: char) -> bool {
    REGEX_SPECIAL_CHARS.contains(&c)
}
// first generation it said Output = char
fn escape<Input>() -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    token('\\').with(satisfy(|c| is_special_char(c)).map(Ext::Escape))
}

fn char<Input>() -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    satisfy(|c| !is_special_char(c)).map(Ext::Char)
}

fn carat<Input>() -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    token('^').map(|_| Ext::Carat)
}

fn dollar<Input>() -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    token('$').map(|_| Ext::Dollar)
}

fn dot<Input>() -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    token('.').map(|_| Ext::Dot)
}

// fixed the type annotation errors after two messages
fn any<Input>() -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    between(
        token('['),
        token(']'),
        many(satisfy(|c| !is_special_char(c) || c == '\\')),
    )
    .map(|chars: Vec<char>| Ext::Any(chars.into_iter().collect::<HashSet<char>>()))
}


fn none_of<Input>() -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    between(
        (token('['), token('^')),
        token(']'),
        many(satisfy(|c| !is_special_char(c) || c == '\\')),
    )
    .map(|chars:Vec<char>| Ext::NoneOf(chars.into_iter().collect()))
}

// first it has it should replace parser(regex) to parser(regex).parse_lazy().
// then it suggests to use defer(regex), but defer is not a function in combine.
// third attempt it suggest to use a inner_regex function with ParserResult, which fails
// fourth attempts, it suggest to change to StdParserResult
// fifth attempt, add .into()

fn inner_regex<Input>(input: &mut Input) -> StdParseResult<Ext, Input>
where
    Input: Stream<Token = char>,
{
    regex().parse_stream(input).into()
}



fn group<Input>() -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    between(token('('), token(')'), parser(inner_regex))
        .map(|inner| Ext::Grp(Rc::new(inner)))
}

fn group_non_marking<Input>() -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    between((token('('), token('?'), token(':')), token(')'), parser(inner_regex))
        .map(|inner| Ext::GrpNonMarking(Rc::new(inner)))
}


fn atom<Input>() -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    choice((
        escape(),
        char(),
        carat(),
        dollar(),
        dot(),
        any(),
        none_of(),
        group(),
        group_non_marking(),
    ))
}

// attempt 1, some type error with Rc

fn base<Input>() -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    (
        choice((
            group_non_marking(),
            group(),
            any(),
            none_of(),
            escape(),
            char(),
            dot(),
            carat(),
            dollar(),
        )),
        optional((
            token('?'),
            optional(token('?')).map(|opt| opt.is_some()),
        )),
    )
        .map(|(inner, opt)| {
            if let Some((_, greedy)) = opt {
                Ext::Opt(Rc::new(inner), greedy)
            } else {
                inner
            }
        })
}


fn postfix<Input>(a: Ext) -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    choice((
        optional((
            token('{'),
            natural(),
            optional((token(','), optional(natural()))),
            token('}'),
            optional(token('?')).map(|opt| opt.is_some()),
        ))
        .map({
            let a = a.clone();
            move |r| match r {
                Some((_, lower, upper, _, greedy)) => Ext::Bound(
                    Rc::new(a.clone()),
                    lower,
                    upper.map(|(_, opt)| opt.unwrap_or(lower)),
                    !greedy,
                ),
                None => a.clone(),
            }
        }),
        optional(token('*'))
            .map({
                let a = a.clone();
                move |r| match r {
                    Some(_) => Ext::Star(Rc::new(a.clone()), true),
                    None => a.clone(),
                }
            }),
        optional(token('+'))
            .map({
                let a = a.clone();
                move |r| match r {
                    Some(_) => Ext::Plus(Rc::new(a.clone()), true),
                    None => a.clone(),
                }
            }),
        optional((
            token('?'),
            optional(token('?')).map(|opt| opt.is_some()),
        ))
        .map({
            let a = a.clone();
            move |r| match r {
                Some((_, greedy)) => Ext::Opt(Rc::new(a.clone()), !greedy),
                None => a.clone(),
            }
        }),
    ))
}


// attempt 1, change the error to use map_err
// attempt 2, defined a customed error struct
// attempt 3, use FnOpaque, which is not defined.
// attempt 4, rewrote natural but .and_then was given two args instead of 1
// attempt 5, realized that we should use String instead?
// attempt 6, need to and type hints, but there is a type mismatch
// attempt 7, it suggests that should use with instead of and_then.. hahaha good luck
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct IntParseError(ParseIntError);

impl From<ParseIntError> for IntParseError {
    fn from(e: ParseIntError) -> Self {
        IntParseError(e)
    }
}

fn natural<Input>() -> impl Parser<Input, Output = u64>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<char, Input::Range, Input::Position, StreamError = IntParseError>,
{
    many1(digit())
        .map(|chars: Vec<char>| chars.into_iter().collect::<String>())
        .with(parser(|input: &mut Input| {
            let digits = input.uncons_while(|c| c.is_digit(10))?;
            digits.parse::<u64>().map_err(|e| {
                <Input::Error as ParseError<char, _, _>>::from_error(input.position(), IntParseError::from(e))
            })
        }))
}






fn concat<Input>() -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    many1(postfix()).map(|terms| {
        if terms.len() == 1 {
            terms.into_iter().next().unwrap()
        } else {
            Ext::Concat(terms)
        }
    })
}

fn or<Input>() -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    sep_by(concat(), token('|')).map(|terms| {
        if terms.len() == 1 {
            terms.into_iter().next().unwrap()
        } else {
            Ext::Or(terms)
        }
    })
}

pub fn regex<Input>() -> impl Parser<Input, Output = Ext>
where
    Input: Stream<Token = char>,
{
    or().skip(optional(token('\n')))
}

*/