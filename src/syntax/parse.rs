use std::{cell::RefCell, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till, take_while},
    character::complete::{char, digit1},
    combinator::{all_consuming, map, map_res, verify},
    multi::{many0, separated_list0},
    sequence::{delimited, pair},
    sequence::{preceded, terminated},
    IResult,
};

use super::{exp::Exp, symb::Symbols};

/// A parser of Risp scripts that uses the `nom` parser combinator library.
pub struct Parser<'a, Symbs: Symbols> {
    pub symbols: &'a mut Symbs,
}

impl<'a, Symbs: Symbols> Parser<'a, Symbs> {
    /// Parse all the Risp expressions from an input `&str`, consuming it entirely.
    pub fn parse_all_exps<'b, Bool: From<bool>, Numb: FromStr>(
        &mut self,
        input: &'b str,
    ) -> IResult<&'b str, Vec<Exp<Bool, Numb, Symbs::Symb>>> {
        all_consuming(preceded(blanks0, |i| self.parse_exps(i)))(input)
    }

    fn parse_exp<'b, Bool: From<bool>, Numb: FromStr>(
        &mut self,
        input: &'b str,
    ) -> IResult<&'b str, Exp<Bool, Numb, Symbs::Symb>> {
        let this = RefCell::new(self);
        let result = alt((
            (map(|i| this.borrow_mut().parse_list(i), |ls| Exp::List(ls))),
            map(
                |i| this.borrow_mut().parse_quoted(i),
                |e| Exp::Quot(Box::new(e)),
            ),
            map(parse_bool, |b| Exp::Bool(b)),
            map(parse_numb, |n| Exp::Numb(n)),
            map(parse_symb, |s| {
                Exp::Symb(this.borrow_mut().symbols.get_or_store(s))
            }),
        ))(input);
        result
    }

    fn parse_exps<'b, Bool: From<bool>, Numb: FromStr>(
        &mut self,
        input: &'b str,
    ) -> IResult<&'b str, Vec<Exp<Bool, Numb, Symbs::Symb>>> {
        many0(terminated(|i| self.parse_exp(i), blanks0))(input)
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
        map(
            delimited(pair(char('('), blanks0), |i| self.parse_exps(i), char(')')),
            |mut ls| {
                ls.reverse();
                ls
            },
        )(input)
    }
}

fn blanks0(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        take_while(is_blank),
        separated_list0(take_while(is_blank), comment),
        take_while(is_blank),
    )(input)
}

fn comment(input: &str) -> IResult<&str, &str> {
    preceded(char(';'), take_till(is_line_break))(input)
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
    map_res(digit1, |n: &str| n.parse())(input)
}

fn parse_symb(input: &str) -> IResult<&str, &str> {
    verify(is_not(" \t\n\r()'"), |s: &str| !s.is_empty())(input)
}
