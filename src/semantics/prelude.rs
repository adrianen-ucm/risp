use std::{
    fmt::{Debug, Display},
    iter::{Product, Sum},
    ops::Sub,
};

use crate::syntax::{
    print::{PrintError, PrintWithSymbols},
    symb::Symbols,
};

use super::{built_in::EvalBuiltIn, val::Val};

// TODO more specific
pub enum PreludeError<Symb> {
    InvalidArguments(),
    ArityMismatch(),
    UnknownSymbol(Symb),
}

impl<Symb: Copy + Debug, Symbs: Symbols<Symb = Symb>> PrintWithSymbols<Symbs>
    for PreludeError<Symbs::Symb>
{
    fn print_with(self, _symbols: &Symbs) -> Result<String, PrintError<<Symbs as Symbols>::Symb>> {
        match self {
            PreludeError::InvalidArguments() => Ok(format!("Invalid arguments")),
            PreludeError::ArityMismatch() => Ok(format!("Arity mismatch")),
            PreludeError::UnknownSymbol(s) => Ok(format!("Unknown symbol: {s:?}")),
        }
    }
}

impl<Bool, Numb, Symb, Env, Symbs> EvalBuiltIn<Bool, Numb, Symb, Env, Symbs, PreludeError<Symb>> {
    pub fn add(
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, PreludeError<Symb>>
    where
        Numb: Sum,
    {
        match vs.into_iter().map(Val::numb).sum() {
            None => Err(PreludeError::InvalidArguments()),
            Some(n) => Ok(Val::Numb(n)),
        }
    }

    pub fn mul(
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, PreludeError<Symb>>
    where
        Numb: Product,
    {
        match vs.into_iter().map(Val::numb).product() {
            None => Err(PreludeError::InvalidArguments()),
            Some(n) => Ok(Val::Numb(n)),
        }
    }

    pub fn sub(
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, PreludeError<Symb>>
    where
        Numb: Sub<Output = Numb>,
    {
        match vs.into_iter().map(Val::numb).collect::<Option<Vec<Numb>>>() {
            None => Err(PreludeError::InvalidArguments()),
            Some(ns) => {
                let mut iter = ns.into_iter();
                match iter.next() {
                    None => Err(PreludeError::ArityMismatch()),
                    Some(n) => Ok(Val::Numb(iter.fold(n, |acc, i| acc - i))),
                }
            }
        }
    }

    pub fn et(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, PreludeError<Symb>>
    where
        Bool: From<bool>,
        Numb: PartialEq,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Numb(l), Val::Numb(r)) => Ok(Val::Bool(Bool::from(l == r))),
                _ => Err(PreludeError::InvalidArguments()),
            },
            _ => Err(PreludeError::ArityMismatch()),
        }
    }

    pub fn gt(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, PreludeError<Symb>>
    where
        Bool: From<bool>,
        Numb: PartialOrd,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Numb(l), Val::Numb(r)) => Ok(Val::Bool(Bool::from(l > r))),
                _ => Err(PreludeError::InvalidArguments()),
            },
            _ => Err(PreludeError::ArityMismatch()),
        }
    }

    pub fn lt(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, PreludeError<Symb>>
    where
        Bool: From<bool>,
        Numb: PartialOrd,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Numb(l), Val::Numb(r)) => Ok(Val::Bool(Bool::from(l < r))),
                _ => Err(PreludeError::InvalidArguments()),
            },
            _ => Err(PreludeError::ArityMismatch()),
        }
    }

    pub fn gte(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, PreludeError<Symb>>
    where
        Bool: From<bool>,
        Numb: PartialOrd,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Numb(l), Val::Numb(r)) => Ok(Val::Bool(Bool::from(l >= r))),
                _ => Err(PreludeError::InvalidArguments()),
            },
            _ => Err(PreludeError::ArityMismatch()),
        }
    }

    pub fn lte(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, PreludeError<Symb>>
    where
        Bool: From<bool>,
        Numb: PartialOrd,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Numb(l), Val::Numb(r)) => Ok(Val::Bool(Bool::from(l <= r))),
                _ => Err(PreludeError::InvalidArguments()),
            },
            _ => Err(PreludeError::ArityMismatch()),
        }
    }

    pub fn not(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, PreludeError<Symb>>
    where
        Bool: From<bool> + Into<bool>,
    {
        match (vs.pop(), vs.pop()) {
            (Some(v), None) => match v {
                Val::Bool(b) => Ok(Val::Bool(Bool::from(!b.into()))),
                _ => Err(PreludeError::InvalidArguments()),
            },
            _ => Err(PreludeError::ArityMismatch()),
        }
    }

    pub fn and(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, PreludeError<Symb>>
    where
        Bool: From<bool> + Into<bool>,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Bool(l), Val::Bool(r)) => Ok(Val::Bool(Bool::from(l.into() && r.into()))),
                _ => Err(PreludeError::InvalidArguments()),
            },
            _ => Err(PreludeError::ArityMismatch()),
        }
    }

    pub fn or(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, PreludeError<Symb>>
    where
        Bool: From<bool> + Into<bool>,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Bool(l), Val::Bool(r)) => Ok(Val::Bool(Bool::from(l.into() || r.into()))),
                _ => Err(PreludeError::InvalidArguments()),
            },
            _ => Err(PreludeError::ArityMismatch()),
        }
    }

    pub fn eq(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, PreludeError<Symb>>
    where
        Bool: PartialEq + From<bool>,
        Numb: PartialEq,
        Symb: PartialEq,
        Env: PartialEq,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Void(), Val::Void()) => Ok(Val::Bool(Bool::from(true))),
                (Val::Bool(l), Val::Bool(r)) => Ok(Val::Bool(Bool::from(l == r))),
                (Val::Numb(l), Val::Numb(r)) => Ok(Val::Bool(Bool::from(l == r))),
                (Val::Quot(l), Val::Quot(r)) => Ok(Val::Bool(Bool::from(l == r))),
                (Val::BuiltIn(_), _) => Err(PreludeError::InvalidArguments()),
                (_, Val::BuiltIn(_)) => Err(PreludeError::InvalidArguments()),
                (Val::Lamb(_, _, _), _) => Err(PreludeError::InvalidArguments()),
                (_, Val::Lamb(_, _, _)) => Err(PreludeError::InvalidArguments()),
                _ => Ok(Val::Bool(Bool::from(false))),
            },
            _ => Err(PreludeError::ArityMismatch()),
        }
    }

    pub fn newline(
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, PreludeError<Symb>> {
        if vs.is_empty() {
            println!("");
            Ok(Val::Void())
        } else {
            Err(PreludeError::ArityMismatch())
        }
    }

    pub fn display(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, PreludeError<Symb>>
    where
        Bool: Into<bool>,
        Numb: Display,
        Symb: Copy,
        Symbs: Symbols<Symb = Symb>,
    {
        match (vs.pop(), vs.pop()) {
            (Some(v), None) => match v.print_with(symbols) {
                Ok(s) => {
                    println!("{s}");
                    Ok(Val::Void())
                }
                Err(PrintError::UnknownSymbol(s)) => Err(PreludeError::UnknownSymbol(s)),
            },
            _ => Err(PreludeError::ArityMismatch()),
        }
    }
}
