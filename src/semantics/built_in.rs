use super::{res::EvalResult, val::Val};

/// A built-in procedure that produces an `EvalResult` from some given values
/// and has access to some symbols.
pub struct EvalBuiltIn<Bool, Numb, Symb, Env, Symbs> {
    built_in: fn(
        Vec<Val<Bool, Numb, Symb, Env, Self>>,
        &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self>,
}

impl<Bool, Numb, Symb, Env, Symbs> EvalBuiltIn<Bool, Numb, Symb, Env, Symbs> {
    /// Create a built-in procedure from a function pointer.
    pub fn new(
        built_in: fn(
            Vec<Val<Bool, Numb, Symb, Env, Self>>,
            &Symbs,
        ) -> EvalResult<Bool, Numb, Symb, Env, Self>,
    ) -> Self {
        Self { built_in }
    }

    /// Apply the built-in procedure.
    pub fn apply(
        &self,
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        symbols: &Symbs,
    ) -> EvalResult<Bool, Numb, Symb, Env, Self> {
        (self.built_in)(vs, symbols)
    }
}

impl<Bool, Numb, Symb, Env, Symbs> Clone for EvalBuiltIn<Bool, Numb, Symb, Env, Symbs> {
    fn clone(&self) -> Self {
        Self {
            built_in: self.built_in,
        }
    }
}
