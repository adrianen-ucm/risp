use std::{hash::Hash, marker::PhantomData};

use crate::syntax::{exp::Exp, symb::Symbols};

use super::{
    built_in::EvalBuiltIn, env::Environments, err::RuntimeError, res::EvalResult, val::Val,
};

/// An evaluator of Risp expressions, with read only access to some
/// symbols and the capability of mutating an environment.
pub struct Evaluator<'a, Val, Symbs: Symbols, Envs: Environments<Symbs::Symb, Val>> {
    symbols: &'a Symbs,
    environment: &'a mut Envs,
    val: PhantomData<Val>,
}

enum EvalStep<Bool, Numb, Symb, Env, BuiltIn> {
    Done(EvalResult<Bool, Numb, Symb, Env, BuiltIn>),
    Loop(Exp<Bool, Numb, Symb>, Env),
}

impl<
        'a,
        Bool: Into<bool> + From<bool> + Clone,
        Numb: Clone,
        Symb: Eq + Hash + Copy,
        Env: Eq + Copy,
        Symbs: Symbols<Symb = Symb>,
        Envs: Environments<
            Symb,
            Val<Bool, Numb, Symb, Env, EvalBuiltIn<Bool, Numb, Symb, Env, Symbs>>,
            Env = Env,
        >,
    >
    Evaluator<
        'a,
        Val<Bool, Numb, Symb, Env, EvalBuiltIn<Bool, Numb, Symb, Env, Symbs>>,
        Symbs,
        Envs,
    >
{
    /// Creates a new `Evaluator` with the given symbols and environment.
    pub fn new(symbols: &'a Symbs, environment: &'a mut Envs) -> Self {
        Self {
            symbols,
            environment,
            val: PhantomData,
        }
    }

    /// Tries to evaluate an `Exp` into a `Val`.
    pub fn eval(
        &mut self,
        exp: Exp<Bool, Numb, Symb>,
    ) -> EvalResult<Bool, Numb, Symb, Env, EvalBuiltIn<Bool, Numb, Symb, Env, Symbs>> {
        self.eval_loop(exp, self.environment.root())
    }

    fn eval_loop(
        &mut self,
        exp: Exp<Bool, Numb, Symb>,
        at: Env,
    ) -> EvalResult<Bool, Numb, Symb, Env, EvalBuiltIn<Bool, Numb, Symb, Env, Symbs>> {
        let (mut next_exp, mut next_at) = (exp, at);
        loop {
            match self.eval_step(next_exp, next_at) {
                EvalStep::Done(r) => {
                    // TODO an attept to clean up the environment during evaluation.
                    if let Ok(ref l) = r {
                        if let Val::Lamb(_, _, _lambda_at) = l {
                        } else if at != next_at {
                            self.environment.drop(next_at);
                        }
                    }

                    return r;
                }
                EvalStep::Loop(exp, at) => {
                    // TODO an attept to clean up the environment during evaluation.
                    if at != next_at && !self.environment.has_children(next_at) {
                        self.environment.drop(next_at);
                    }

                    (next_exp, next_at) = (exp, at)
                }
            }
        }
    }

    fn eval_step(
        &mut self,
        exp: Exp<Bool, Numb, Symb>,
        at: Env,
    ) -> EvalStep<Bool, Numb, Symb, Env, EvalBuiltIn<Bool, Numb, Symb, Env, Symbs>> {
        match exp {
            Exp::Numb(n) => EvalStep::Done(Ok(Val::Numb(n))),
            Exp::Bool(b) => EvalStep::Done(Ok(Val::Bool(b))),
            Exp::Quot(b) => EvalStep::Done(Ok(match *b {
                Exp::Numb(n) => Val::Numb(n),
                Exp::Bool(b) => Val::Bool(b),
                _ => Val::Quot(*b),
            })),
            Exp::Symb(s) => match self.environment.get(at, &s) {
                Some(v) => EvalStep::Done(Ok(v.clone())),
                None => EvalStep::Done(Err(RuntimeError::UndefinedVariable(s))),
            },
            Exp::List(mut ls) => match ls.pop() {
                Some(Exp::Symb(s)) => match self.environment.get(at, &s) {
                    Some(v) => self.eval_app_procedure(v.clone(), ls, at),
                    None => match self.symbols.resolve(s) {
                        Some("define") => {
                            EvalStep::Done(match (ls.pop(), ls.pop(), ls.pop()) {
                                (Some(Exp::Symb(x)), Some(e), None) => self
                                    .eval_loop(e, at)
                                    .and_then(|v| match self.environment.define(at, x, v) {
                                        Ok(()) => Ok(Val::Void()),
                                        Err((x, _)) => Err(RuntimeError::AlreadyDefined(x)),
                                    }),
                                _ => Err(RuntimeError::BadFormedExpression(s)),
                            })
                        }
                        Some("quote") => match (ls.pop(), ls.pop()) {
                            (Some(e), None) => EvalStep::Loop(Exp::Quot(Box::new(e)), at),
                            _ => EvalStep::Done(Err(RuntimeError::BadFormedExpression(s))),
                        },
                        Some("if") => match (ls.pop(), ls.pop(), ls.pop(), ls.pop()) {
                            (Some(c), Some(e1), Some(e2), None) => match self.eval_loop(c, at) {
                                Ok(v) => EvalStep::Loop(if v.into() { e1 } else { e2 }, at),
                                Err(err) => EvalStep::Done(Err(err)),
                            },
                            _ => EvalStep::Done(Err(RuntimeError::BadFormedExpression(s))),
                        },
                        Some("begin") => EvalStep::Done(
                            self.eval_args(ls, at)
                                .map(|mut args| args.pop().unwrap_or_else(Val::Void)),
                        ),
                        Some("eval") => match (ls.pop(), ls.pop()) {
                            (Some(e), None) => match self.eval_loop(e, at) {
                                Ok(Val::Quot(e)) => EvalStep::Loop(e, at),
                                Ok(e) => EvalStep::Done(Ok(e)),
                                Err(err) => EvalStep::Done(Err(err)),
                            },
                            _ => EvalStep::Done(Err(RuntimeError::BadFormedExpression(s))),
                        },
                        Some("lambda") => EvalStep::Done(match (ls.pop(), ls.pop(), ls.pop()) {
                            (Some(Exp::List(ls)), Some(b), None) => {
                                match ls.into_iter().rev().map(Exp::symb).collect() {
                                    Some(ps) => Ok(Val::Lamb(ps, b, at.clone())),
                                    None => Err(RuntimeError::BadFormedExpression(s)),
                                }
                            }
                            _ => Err(RuntimeError::BadFormedExpression(s)),
                        }),
                        Some("and") => {
                            let mut last = Val::Bool(Bool::from(true));
                            while let Some(e) = ls.pop() {
                                match self.eval_loop(e, at) {
                                    Err(err) => return EvalStep::Done(Err(err)),
                                    Ok(Val::Bool(b)) => {
                                        if b.into() {
                                            last = Val::Bool(Bool::from(true));
                                        } else {
                                            return EvalStep::Done(Ok(Val::Bool(Bool::from(
                                                false,
                                            ))));
                                        }
                                    }

                                    Ok(v) => last = v,
                                }
                            }

                            EvalStep::Done(Ok(last))
                        }
                        Some("or") => {
                            while let Some(e) = ls.pop() {
                                match self.eval_loop(e, at) {
                                    Err(err) => return EvalStep::Done(Err(err)),
                                    Ok(Val::Bool(b)) => {
                                        if b.into() {
                                            return EvalStep::Done(Ok(Val::Bool(Bool::from(true))));
                                        }
                                    }
                                    Ok(v) => return EvalStep::Done(Ok(v)),
                                }
                            }

                            EvalStep::Done(Ok(Val::Bool(Bool::from(false))))
                        }
                        Some(_) => EvalStep::Done(Err(RuntimeError::UnknownExpression(s))),
                        None => EvalStep::Done(Err(RuntimeError::UnknownSymbol(s))),
                    },
                },
                Some(e) => match self.eval_loop(e, at) {
                    Ok(v) => self.eval_app_procedure(v.clone(), ls, at),
                    Err(err) => EvalStep::Done(Err(err)),
                },
                None => EvalStep::Done(Err(RuntimeError::MissingProcedure())),
            },
        }
    }

    fn eval_args(
        &mut self,
        ls: Vec<Exp<Bool, Numb, Symb>>,
        at: Env,
    ) -> Result<
        Vec<Val<Bool, Numb, Symb, Env, EvalBuiltIn<Bool, Numb, Symb, Env, Symbs>>>,
        RuntimeError<Symb, Val<Bool, Numb, Symb, Env, EvalBuiltIn<Bool, Numb, Symb, Env, Symbs>>>,
    > {
        ls.into_iter()
            .rev()
            .map(|e| self.eval_loop(e, at))
            .collect()
    }

    fn eval_app_procedure(
        &mut self,
        v: Val<Bool, Numb, Symb, Env, EvalBuiltIn<Bool, Numb, Symb, Env, Symbs>>,
        ls: Vec<Exp<Bool, Numb, Symb>>,
        at: Env,
    ) -> EvalStep<Bool, Numb, Symb, Env, EvalBuiltIn<Bool, Numb, Symb, Env, Symbs>> {
        match v {
            Val::BuiltIn(f) => EvalStep::Done(
                self.eval_args(ls, at)
                    .and_then(|vs| f.apply(vs, self.symbols)),
            ),
            Val::Lamb(ps, b, at_lambda) => {
                if ps.len() != ls.len() {
                    EvalStep::Done(Err(RuntimeError::ArityMismatch()))
                } else {
                    match self.eval_args(ls, at) {
                        Err(err) => EvalStep::Done(Err(err)),
                        Ok(args) => match self.environment.push(at_lambda, ps.len()) {
                            None => EvalStep::Done(Err(RuntimeError::CouldNotPushEnvironment())),
                            Some(at) => {
                                match ps
                                    .into_iter()
                                    .zip(args)
                                    .map(|(x, v)| self.environment.define(at, x, v))
                                    .collect()
                                {
                                    Ok(()) => EvalStep::Loop(b, at),
                                    Err((x, _)) => {
                                        EvalStep::Done(Err(RuntimeError::AlreadyDefined(x)))
                                    }
                                }
                            }
                        },
                    }
                }
            }
            v => EvalStep::Done(Err(RuntimeError::NotAProcedure(v))),
        }
    }
}
