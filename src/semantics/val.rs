use std::fmt::Display;

use crate::syntax::{
    exp::Exp,
    print::{PrintError, PrintWithSymbols},
    symb::Symbols,
};

/// A value that can result from the evaluation of an `Exp`.
#[derive(Clone)]
pub enum Val<Bool, Numb, Symb, Env, Err> {
    /// Absence of a value, mainly to denote a side effect.
    Void(),
    /// A boolean value of type `Bool`.
    Bool(Bool),
    /// A numeric value of type `Numb`.
    Numb(Numb),
    /// A quoted expression.
    Quot(Exp<Bool, Numb, Symb>),
    /// A lambda with a reference to its environment.
    Lamb(Vec<Symb>, Exp<Bool, Numb, Symb>, Env),
    /// A built-in procedure that can produce an error of type `Err`.
    BuiltIn(fn(Vec<Self>) -> Result<Self, Err>),
}

impl<Bool, Numb, Symb, Env, Err> Val<Bool, Numb, Symb, Env, Err> {
    /// Returns the underlying number of a value if it
    /// corresponds to a number.
    pub fn numb(self) -> Option<Numb> {
        match self {
            Val::Numb(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the underlying boolean of a value if it
    /// corresponds to a boolean.
    pub fn bool(self) -> Option<Bool> {
        match self {
            Val::Bool(b) => Some(b),
            _ => None,
        }
    }
}

impl<Bool: Into<bool>, Numb, Symb, Ctx, Err> Into<bool> for Val<Bool, Numb, Symb, Ctx, Err> {
    fn into(self) -> bool {
        match self {
            Val::Bool(b) => b.into(),
            _ => true,
        }
    }
}

impl<Bool: Into<bool>, Numb: Display, Symb: Copy, Symbs: Symbols<Symb = Symb>, Env, Err>
    PrintWithSymbols<Symbs> for Val<Bool, Numb, Symbs::Symb, Env, Err>
{
    fn print_with(self, symbols: &Symbs) -> Result<String, PrintError<Symbs::Symb>> {
        match self {
            Val::Void() => Ok(format!("#<void>")),
            Val::Bool(b) => Ok(format!("{}", if b.into() { "#t" } else { "#f" })),
            Val::Numb(n) => Ok(format!("{n}")),
            Val::Lamb(_, _, _) => Ok(format!("#<procedure:lambda>")),
            Val::BuiltIn(_) => Ok(format!("#<procedure:builtin>")),
            Val::Quot(e) => e.print_with(symbols).map(|s| format!("'{s}")),
        }
    }
}
