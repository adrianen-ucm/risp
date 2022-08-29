use super::{err::RuntimeError, val::Val};

/// The `Result` of evaluating an `Exp` into a `Val` where a
/// `RuntimeError` can happen.
pub type EvalResult<Bool, Numb, Symb, Env, BuiltIn> = Result<
    Val<Bool, Numb, Symb, Env, BuiltIn>,
    RuntimeError<Symb, Val<Bool, Numb, Symb, Env, BuiltIn>>,
>;
