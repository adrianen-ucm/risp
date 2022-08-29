use super::val::Val;

pub struct EvalBuiltIn<Bool, Numb, Symb, Env, Symbs, Err> {
    built_in: fn(
        Vec<Val<Bool, Numb, Symb, Env, Self>>,
        &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, Err>,
}

impl<Bool, Numb, Symb, Env, Symbs, Err> EvalBuiltIn<Bool, Numb, Symb, Env, Symbs, Err> {
    pub fn new(
        built_in: fn(
            Vec<Val<Bool, Numb, Symb, Env, Self>>,
            &Symbs,
        ) -> Result<Val<Bool, Numb, Symb, Env, Self>, Err>,
    ) -> Self {
        Self { built_in: built_in }
    }

    pub fn apply(
        &self,
        vs: Vec<Val<Bool, Numb, Symb, Env, Self>>,
        symbols: &Symbs,
    ) -> Result<Val<Bool, Numb, Symb, Env, Self>, Err> {
        (self.built_in)(vs, symbols)
    }
}

impl<Bool, Numb, Symb, Env, Symbs, Err> Clone for EvalBuiltIn<Bool, Numb, Symb, Env, Symbs, Err> {
    fn clone(&self) -> Self {
        Self {
            built_in: self.built_in,
        }
    }
}
