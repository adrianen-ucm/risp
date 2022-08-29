use std::fmt::Display;

use super::{
    print::{PrintError, PrintWithSymbols},
    symb::Symbols,
};

/// A Risp expression.
#[derive(Clone, PartialEq, Eq)]
pub enum Exp<Bool, Numb, Symb> {
    /// A number of type `Numb`.
    Numb(Numb),
    /// A boolean of type `Bool`.
    Bool(Bool),
    /// A symbol of type `Symb`.
    Symb(Symb),
    /// A quoted expression.
    Quot(Box<Self>),
    /// A list, assumed to be reversed for the ease of its consumption.
    List(Vec<Self>),
}

impl<Bool, Numb, Symb> Exp<Bool, Numb, Symb> {
    /// Returns the underlying number of a Risp expression if it
    /// corresponds to a number.
    pub fn numb(self) -> Option<Numb> {
        match self {
            Exp::Numb(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the underlying boolean of a Risp expression if it
    /// corresponds to a boolean.
    pub fn bool(self) -> Option<Bool> {
        match self {
            Exp::Bool(b) => Some(b),
            _ => None,
        }
    }

    /// Returns the underlying symbol of a Risp expression if it
    /// corresponds to a symbol.
    pub fn symb(self) -> Option<Symb> {
        match self {
            Exp::Symb(s) => Some(s),
            _ => None,
        }
    }
}

impl<Bool: Into<bool>, Numb: Display, Symb: Copy, Symbs: Symbols<Symb = Symb>>
    PrintWithSymbols<Symbs> for Exp<Bool, Numb, Symbs::Symb>
{
    fn print_with(self, symbols: &Symbs) -> Result<String, PrintError<Symbs::Symb>> {
        match self {
            Exp::Bool(b) => Ok(format!("{}", if b.into() { "#t" } else { "#f" })),
            Exp::Numb(n) => Ok(format!("{n}")),
            Exp::Symb(s) => match symbols.resolve(s) {
                None => Err(PrintError::UnknownSymbol(s)),
                Some(s) => Ok(format!("{s}")),
            },
            Exp::Quot(b) => (*b).print_with(symbols).map(|s| format!("'{s}")),
            Exp::List(ls) => ls
                .into_iter()
                .rev()
                .map(|e| e.print_with(symbols))
                .collect::<Result<Vec<String>, _>>()
                .map(|ss| format!("({})", ss.join(" "))),
        }
    }
}
