use std::{
    fmt::Display,
    iter::{Product, Sum},
    ops::Sub,
};

use crate::syntax::{
    print::{PrintError, PrintWithSymbols},
    symb::Symbols,
};

use super::{built_in::EvalBuiltIn, env::Environments, err::RuntimeError, val::Val};

impl<Bool, Numb, Symb, Env, Symbs: Symbols<Symb = Symb>> EvalBuiltIn<Bool, Numb, Symb, Env, Symbs> {
    pub fn load_prelude<'a, Envs: Environments<Symb, Val<Bool, Numb, Symb, Env, Self>, Env = Env>>(
        env: &mut Envs,
        symbols: &mut Symbs,
    ) -> Result<(), &'a str>
    where
        Bool: From<bool> + Into<bool> + PartialEq,
        Numb: Sum + Product + Sub<Output = Numb> + PartialEq + PartialOrd + Display,
        Symb: Copy + PartialEq,
    {
        for (x, v) in [
            ("true", Val::Bool(Bool::from(true))),
            ("false", Val::Bool(Bool::from(false))),
            ("+", Val::BuiltIn(EvalBuiltIn::new(Self::add))),
            ("*", Val::BuiltIn(EvalBuiltIn::new(Self::mul))),
            ("-", Val::BuiltIn(EvalBuiltIn::new(Self::sub))),
            ("=", Val::BuiltIn(EvalBuiltIn::new(Self::et))),
            (">", Val::BuiltIn(EvalBuiltIn::new(Self::gt))),
            ("<", Val::BuiltIn(EvalBuiltIn::new(Self::lt))),
            (">=", Val::BuiltIn(EvalBuiltIn::new(Self::gte))),
            ("<=", Val::BuiltIn(EvalBuiltIn::new(Self::lte))),
            ("not", Val::BuiltIn(EvalBuiltIn::new(Self::not))),
            ("and", Val::BuiltIn(EvalBuiltIn::new(Self::and))),
            ("or", Val::BuiltIn(EvalBuiltIn::new(Self::or))),
            ("eq?", Val::BuiltIn(EvalBuiltIn::new(Self::eq))),
            ("newline", Val::BuiltIn(EvalBuiltIn::new(Self::newline))),
            ("display", Val::BuiltIn(EvalBuiltIn::new(Self::display))),
        ] {
            if let Err(_) = env.define(env.root(), symbols.get_or_store(x), v) {
                return Err(x);
            }
        }

        Ok(())
    }

    fn add(
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    >
    where
        Numb: Sum,
    {
        match vs.into_iter().map(Val::numb).sum() {
            None => Err(RuntimeError::InvalidArguments()),
            Some(n) => Ok(Val::Numb(n)),
        }
    }

    fn mul(
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    >
    where
        Numb: Product,
    {
        match vs.into_iter().map(Val::numb).product() {
            None => Err(RuntimeError::InvalidArguments()),
            Some(n) => Ok(Val::Numb(n)),
        }
    }

    fn sub(
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    >
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

    fn et(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    >
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

    fn gt(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    >
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

    fn lt(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    >
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

    fn gte(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    >
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

    fn lte(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    >
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

    fn not(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    >
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

    fn and(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    >
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

    fn or(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    >
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

    fn eq(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    >
    where
        Bool: PartialEq + From<bool>,
        Numb: PartialEq,
        Symb: PartialEq,
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

    fn newline(
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        _symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    > {
        if vs.is_empty() {
            println!("");
            Ok(Val::Void())
        } else {
            Err(RuntimeError::ArityMismatch())
        }
    }

    fn display(
        mut vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    >
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
