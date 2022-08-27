use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till, take_while},
    character::complete::{char, digit1},
    combinator::{eof, fail, map},
    multi::{many0, separated_list0},
    sequence::delimited,
    sequence::{preceded, terminated},
    IResult,
};

use super::{exp::Exp, symb::Symbols};

pub struct Parser<'a, Symbs: Symbols> {
    pub symbols: &'a mut Symbs,
}

impl<'a, Symbs: Symbols> Parser<'a, Symbs> {
    pub fn parse_exps<'b, Bool: From<bool>, Numb: FromStr>(
        &mut self,
        input: &'b str,
    ) -> IResult<&'b str, Vec<Exp<Bool, Numb, Symbs::Symb>>> {
        let (input, _) = blanks0(input)?;
        let (input, exps) = many0(terminated(|i| self.parse_exp(i), blanks0))(input)?;
        let (input, _) = eof(input)?;
        Ok((input, exps))
    }

    pub fn parse_exp<'b, Bool: From<bool>, Numb: FromStr>(
        &mut self,
        input: &'b str,
    ) -> IResult<&'b str, Exp<Bool, Numb, Symbs::Symb>> {
        if let Ok((input, ls)) = self.parse_list(input) {
            Ok((input, Exp::List(ls)))
        } else if let Ok((input, e)) = self.parse_quoted(input) {
            Ok((input, Exp::Quot(Box::new(e))))
        } else if let Ok((input, b)) = parse_bool(input) {
            Ok((input, Exp::Bool(b)))
        } else if let Ok((input, n)) = parse_numb(input) {
            Ok((input, Exp::Numb(n)))
        } else if let Ok((input, s)) = parse_symb(input) {
            Ok((input, Exp::Symb(self.symbols.get_or_store(s))))
        } else {
            fail(input)
        }
    }

    fn parse_quoted<'b, Bool: From<bool>, Numb: FromStr>(
        &mut self,
        input: &'b str,
    ) -> IResult<&'b str, Exp<Bool, Numb, Symbs::Symb>> {
        preceded(char('\''), |i| self.parse_exp(i))(input)
    }

    fn parse_list<'b, Bool: From<bool>, Numb: FromStr>(
        &mut self,
        input: &'b str,
    ) -> IResult<&'b str, Vec<Exp<Bool, Numb, Symbs::Symb>>> {
        let (input, _) = char('(')(input)?;
        let (input, _) = blanks0(input)?;
        let (input, mut exps) = many0(terminated(|i| self.parse_exp(i), blanks0))(input)?;
        let (input, _) = char(')')(input)?;
        exps.reverse();
        Ok((input, exps))
    }
}

fn blanks0(input: &str) -> IResult<&str, ()> {
    delimited(
        take_while(is_blank),
        separated_list0(take_while(is_blank), comment),
        take_while(is_blank),
    )(input)
    .map(|(i, _)| (i, ()))
}

fn comment(input: &str) -> IResult<&str, &str> {
    let (input, _) = char(';')(input)?;
    take_till(is_line_break)(input)
}

fn is_line_break(chr: char) -> bool {
    chr == '\n' || chr == '\r'
}

fn is_blank(chr: char) -> bool {
    chr == ' ' || chr == '\t' || is_line_break(chr)
}

fn parse_bool<Bool: From<bool>>(input: &str) -> IResult<&str, Bool> {
    alt((
        map(tag("#t"), |_| Bool::from(true)),
        map(tag("#f"), |_| Bool::from(false)),
    ))(input)
}

fn parse_numb<Numb: FromStr>(input: &str) -> IResult<&str, Numb> {
    let (input, n) = digit1(input)?;
    match n.parse() {
        Ok(n) => Ok((input, n)),
        Err(_) => fail(input),
    }
}

fn parse_symb(input: &str) -> IResult<&str, &str> {
    let (input, sym) = is_not(" \t\n\r()'")(input)?;
    if sym.is_empty() {
        fail(input)
    } else {
        Ok((input, sym))
    }
}
