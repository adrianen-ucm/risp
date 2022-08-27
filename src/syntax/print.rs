use super::symb::Symbols;

/// Types that depend on `Symbols` to be printed.
pub trait PrintWithSymbols<Symbs: Symbols> {
    /// Print with the given `Symbols`.
    fn print_with(self, symbols: &Symbs) -> Result<String, PrintError<Symbs::Symb>>;
}

/// Errors that can arise when printing with `Symbols`.
pub enum PrintError<Symb> {
    UnknownSymbol(Symb),
}
