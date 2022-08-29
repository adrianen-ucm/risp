use std::fmt::Debug;

use crate::syntax::{
    print::{PrintError, PrintWithSymbols},
    symb::Symbols,
};

/// Errors that can arise during the evaluation of an `Exp`.
#[derive(Clone)]
pub enum RuntimeError<Symb, Val, BuiltInErr> {
    AlreadyDefined(Symb),
    ArityMismatch(),
    BadFormedExpression(Symb),
    BuiltInError(BuiltInErr),
    CouldNotPushEnvironment(),
    InvalidArguments(),
    MissingProcedure(),
    NotAProcedure(Val),
    UndefinedVariable(Symb),
    UnknownExpression(Symb),
    UnknownSymbol(Symb),
}

impl<
        Symb: Copy + Debug,
        Symbs: Symbols<Symb = Symb>,
        Val: PrintWithSymbols<Symbs>,
        BuiltInErr: PrintWithSymbols<Symbs>,
    > PrintWithSymbols<Symbs> for RuntimeError<Symbs::Symb, Val, BuiltInErr>
{
    fn print_with(self, symbols: &Symbs) -> Result<String, PrintError<Symbs::Symb>> {
        match self {
            RuntimeError::AlreadyDefined(s) => match symbols.resolve(s) {
                None => Err(PrintError::UnknownSymbol(s)),
                Some(s) => Ok(format!("Already defined: {s}")),
            },
            RuntimeError::ArityMismatch() => Ok(format!("Arity mismatch")),
            RuntimeError::BadFormedExpression(s) => match symbols.resolve(s) {
                None => Err(PrintError::UnknownSymbol(s)),
                Some(s) => Ok(format!("Bad formed expression: {s}")),
            },
            RuntimeError::BuiltInError(e) => e.print_with(symbols),
            RuntimeError::CouldNotPushEnvironment() => Ok(format!("Could not push environment")),
            RuntimeError::InvalidArguments() => Ok(format!("Invalid arguments")),
            RuntimeError::MissingProcedure() => Ok(format!("Missing procedure")),
            RuntimeError::NotAProcedure(v) => v
                .print_with(symbols)
                .map(|s| format!("Not a procedure: {s}")),
            RuntimeError::UndefinedVariable(x) => match symbols.resolve(x) {
                None => Err(PrintError::UnknownSymbol(x)),
                Some(s) => Ok(format!("Undefined variable: {s}")),
            },
            RuntimeError::UnknownExpression(s) => match symbols.resolve(s) {
                None => Err(PrintError::UnknownSymbol(s)),
                Some(s) => Ok(format!("Unknown expression: {s}")),
            },
            RuntimeError::UnknownSymbol(s) => Ok(format!("Unknown symbol: {s:?}")),
        }
    }
}
