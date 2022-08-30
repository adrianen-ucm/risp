use super::{err::RuntimeError, val::Val};

pub struct EvalBuiltIn<Bool, Numb, Symb, Env, Symbs> {
    built_in: fn(
        Vec<Val<Bool, Numb, Symb, Env, Self>>,
        &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    >,
}

impl<Bool, Numb, Symb, Env, Symbs> EvalBuiltIn<Bool, Numb, Symb, Env, Symbs> {
    pub fn new(
        built_in: fn(
            Vec<Val<Bool, Numb, Symb, Env, Self>>,
            &Symbs,
        ) -> Result<
            Val<Bool, Numb, Symb, Env, Self>,
            RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
        >,
    ) -> Self {
        Self { built_in }
    }

    pub fn apply(
        &self,
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        symbols: &Symbs,
    ) -> Result<
        Val<Bool, Numb, Symb, Env, Self>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, Self>>,
    > {
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
