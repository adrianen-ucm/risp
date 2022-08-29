use std::{
    fmt::Display,
    iter::{Product, Sum},
    ops::Sub,
};

use crate::syntax::{
    print::{PrintError, PrintWithSymbols},
    symb::Symbols,
};

use super::{built_in::EvalBuiltIn, err::RuntimeError, res::EvalResult, val::Val};

impl<Bool, Numb, Symb, Env, Symbs> EvalBuiltIn<Bool, Numb, Symb, Env, Symbs> {
    pub fn add(
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self>
    where
        Numb: Sum,
    {
        match vs.into_iter().map(Val::numb).sum() {
            None => Err(RuntimeError::InvalidArguments()),
            Some(n) => Ok(Val::Numb(n)),
        }
    }

    pub fn mul(
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self>
    where
        Numb: Product,
    {
        match vs.into_iter().map(Val::numb).product() {
            None => Err(RuntimeError::InvalidArguments()),
            Some(n) => Ok(Val::Numb(n)),
        }
    }

    pub fn sub(
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self>
    where
        Numb: Sub<Output = Numb>,
    {
        match vs.into_iter().map(Val::numb).collect::<Option<Vec<Numb>>>() {
            None => Err(RuntimeError::InvalidArguments()),
            Some(ns) => {
                let mut iter = ns.into_iter();
                match iter.next() {
                    None => Err(RuntimeError::ArityMismatch()),
                    Some(n) => Ok(Val::Numb(iter.fold(n, |acc, i| acc - i))),
                }
            }
        }
    }

    pub fn et(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self>
    where
        Bool: From<bool>,
        Numb: PartialEq,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Numb(l), Val::Numb(r)) => Ok(Val::Bool(Bool::from(l == r))),
                _ => Err(RuntimeError::InvalidArguments()),
            },
            _ => Err(RuntimeError::ArityMismatch()),
        }
    }

    pub fn gt(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self>
    where
        Bool: From<bool>,
        Numb: PartialOrd,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Numb(l), Val::Numb(r)) => Ok(Val::Bool(Bool::from(l > r))),
                _ => Err(RuntimeError::InvalidArguments()),
            },
            _ => Err(RuntimeError::ArityMismatch()),
        }
    }

    pub fn lt(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self>
    where
        Bool: From<bool>,
        Numb: PartialOrd,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Numb(l), Val::Numb(r)) => Ok(Val::Bool(Bool::from(l < r))),
                _ => Err(RuntimeError::InvalidArguments()),
            },
            _ => Err(RuntimeError::ArityMismatch()),
        }
    }

    pub fn gte(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self>
    where
        Bool: From<bool>,
        Numb: PartialOrd,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Numb(l), Val::Numb(r)) => Ok(Val::Bool(Bool::from(l >= r))),
                _ => Err(RuntimeError::InvalidArguments()),
            },
            _ => Err(RuntimeError::ArityMismatch()),
        }
    }

    pub fn lte(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self>
    where
        Bool: From<bool>,
        Numb: PartialOrd,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Numb(l), Val::Numb(r)) => Ok(Val::Bool(Bool::from(l <= r))),
                _ => Err(RuntimeError::InvalidArguments()),
            },
            _ => Err(RuntimeError::ArityMismatch()),
        }
    }

    pub fn not(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self>
    where
        Bool: From<bool> + Into<bool>,
    {
        match (vs.pop(), vs.pop()) {
            (Some(v), None) => match v {
                Val::Bool(b) => Ok(Val::Bool(Bool::from(!b.into()))),
                _ => Err(RuntimeError::InvalidArguments()),
            },
            _ => Err(RuntimeError::ArityMismatch()),
        }
    }

    pub fn and(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self>
    where
        Bool: From<bool> + Into<bool>,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Bool(l), Val::Bool(r)) => Ok(Val::Bool(Bool::from(l.into() && r.into()))),
                _ => Err(RuntimeError::InvalidArguments()),
            },
            _ => Err(RuntimeError::ArityMismatch()),
        }
    }

    pub fn or(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self>
    where
        Bool: From<bool> + Into<bool>,
    {
        match (vs.pop(), vs.pop(), vs.pop()) {
            (Some(r), Some(l), None) => match (l, r) {
                (Val::Bool(l), Val::Bool(r)) => Ok(Val::Bool(Bool::from(l.into() || r.into()))),
                _ => Err(RuntimeError::InvalidArguments()),
            },
            _ => Err(RuntimeError::ArityMismatch()),
        }
    }

    pub fn eq(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self>
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
                (Val::BuiltIn(_), _) => Err(RuntimeError::InvalidArguments()),
                (_, Val::BuiltIn(_)) => Err(RuntimeError::InvalidArguments()),
                (Val::Lamb(_, _, _), _) => Err(RuntimeError::InvalidArguments()),
                (_, Val::Lamb(_, _, _)) => Err(RuntimeError::InvalidArguments()),
                _ => Ok(Val::Bool(Bool::from(false))),
            },
            _ => Err(RuntimeError::ArityMismatch()),
        }
    }

    pub fn newline(
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self> {
        if vs.is_empty() {
            println!("");
            Ok(Val::Void())
        } else {
            Err(RuntimeError::ArityMismatch())
        }
    }

    pub fn display(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self>
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
                Err(PrintError::UnknownSymbol(s)) => Err(RuntimeError::UnknownSymbol(s)),
            },
            _ => Err(RuntimeError::ArityMismatch()),
        }
    }
}
