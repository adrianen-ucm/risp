use std::fmt::{Debug, Display};

use crate::syntax::{
    print::{PrintError, PrintWithSymbols},
    symb::Symbols,
};

use super::val::Val;

#[derive(Clone)]
pub enum RuntimeError<Bool, Numb, Symb, Env> {
    MissingProcedure(),
    NotAProcedure(Val<Bool, Numb, Symb, Env, Self>),
    UndefinedVariable(Symb),
    UnknownSymbol(Symb),
    AlreadyDefined(Symb),
    UnknownExpression(Symb),
    BadFormedExpression(Symb),
    ArityMismatch(),
    CouldNotPushEnvironment(),
}

impl<Bool: Into<bool>, Numb: Display, Symb: Copy + Debug, Symbs: Symbols<Symb = Symb>, Env>
    PrintWithSymbols<Symbs> for RuntimeError<Bool, Numb, Symbs::Symb, Env>
{
    fn print_with(self, symbols: &Symbs) -> Result<String, PrintError<Symbs::Symb>> {
        match self {
            RuntimeError::MissingProcedure() => Ok(format!("Missing procedure")),
            RuntimeError::NotAProcedure(v) => v
                .print_with(symbols)
                .map(|s| format!("Not a procedure: {s}")),
            RuntimeError::UndefinedVariable(x) => match symbols.resolve(x) {
                None => Err(PrintError::UnknownSymbol(x)),
                Some(s) => Ok(format!("Undefined variable: {s}")),
            },
            RuntimeError::UnknownSymbol(s) => Ok(format!("Unknown symbol: {s:?}")),
            RuntimeError::UnknownExpression(s) => match symbols.resolve(s) {
                None => Err(PrintError::UnknownSymbol(s)),
                Some(s) => Ok(format!("Unknown expression: {s}")),
            },
            RuntimeError::BadFormedExpression(s) => match symbols.resolve(s) {
                None => Err(PrintError::UnknownSymbol(s)),
                Some(s) => Ok(format!("Bad formed expression: {s}")),
            },
            RuntimeError::AlreadyDefined(s) => match symbols.resolve(s) {
                None => Err(PrintError::UnknownSymbol(s)),
                Some(s) => Ok(format!("Already defined: {s}")),
            },
            RuntimeError::ArityMismatch() => Ok(format!("Arity mismatch")),
            RuntimeError::CouldNotPushEnvironment() => Ok(format!("Could not push environment")),
        }
    }
}
