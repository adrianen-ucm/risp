/// Types that can store strings, referenced by the so called symbols.
pub trait Symbols {
    /// A symbol (i.e. reference to a stored string).
    type Symb;

    /// Returns the string that corresponds to a given symbol, if stored.
    fn resolve(&self, symbol: Self::Symb) -> Option<&str>;

    /// Returns the symbol that corresponds to a given string, by storing it if necessary.
    fn get_or_store(&mut self, string: &str) -> Self::Symb;
}
