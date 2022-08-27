use std::hash::Hash;

use crate::syntax::{exp::Exp, symb::Symbols};

use super::{env::Environments, err::RuntimeError, val::Val};

pub type EvalResult<Bool, Numb, Symb, Env> = Result<
    Val<Bool, Numb, Symb, Env, RuntimeError<Bool, Numb, Symb, Env>>,
    RuntimeError<Bool, Numb, Symb, Env>,
>;

pub struct Evaluator<'a, Symbs: Symbols, Envs: Environments> {
    symbols: &'a Symbs,
    environment: &'a mut Envs,
}

enum EvalStep<Bool, Numb, Symb, Env> {
    Done(EvalResult<Bool, Numb, Symb, Env>),
    Loop(Exp<Bool, Numb, Symb>, Env),
}

impl<
        'a,
        Bool: Into<bool> + Clone,
        Numb: Clone,
        Symb: Eq + Hash + Copy,
        Env: Eq + Copy,
        Symbs: Symbols<Symb = Symb>,
        Envs: Environments<
            Var = Symb,
            Val = Val<Bool, Numb, Symb, Env, RuntimeError<Bool, Numb, Symb, Env>>,
            Env = Env,
        >,
    > Evaluator<'a, Symbs, Envs>
{
    pub fn new(symbols: &'a Symbs, environment: &'a mut Envs) -> Self {
        Evaluator {
            symbols: symbols,
            environment: environment,
        }
    }

    pub fn eval(&mut self, exp: Exp<Bool, Numb, Symb>) -> EvalResult<Bool, Numb, Symb, Env> {
        self.eval_loop(exp, self.environment.root())
    }

    fn eval_loop(
        &mut self,
        exp: Exp<Bool, Numb, Symb>,
        at: Env,
    ) -> EvalResult<Bool, Numb, Symb, Env> {
        let mut next_exp = exp;
        let mut next_at = at;
        loop {
            match self.eval_step(next_exp, next_at) {
                EvalStep::Done(r) => {
                    if let Ok(ref l) = r {
                        if let Val::Lamb(_, _, _lambda_at) = l {
                        } else {
                            if at != next_at {
                                self.environment.drop(next_at);
                            }
                        }
                    }

                    return r;
                }
                EvalStep::Loop(exp, at) => {
                    next_exp = exp;

                    if at != next_at && !self.environment.has_children(next_at) {
                        self.environment.drop(next_at);
                    }

                    next_at = at;
                }
            }
        }
    }

    fn eval_step(
        &mut self,
        exp: Exp<Bool, Numb, Symb>,
        at: Env,
    ) -> EvalStep<Bool, Numb, Symb, Env> {
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
                        None => EvalStep::Done(Err(RuntimeError::UnknownSymbol(s))),
                        Some("define") => self.eval_define(s, ls, at),
                        Some("quote") => self.eval_quote(s, ls, at),
                        Some("if") => self.eval_if(s, ls, at),
                        Some("begin") => self.eval_begin(s, ls, at),
                        Some("eval") => self.eval_eval(s, ls, at),
                        Some("lambda") => self.eval_lambda(s, ls, at),
                        Some(_) => EvalStep::Done(Err(RuntimeError::UnknownExpression(s))),
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
        Vec<Val<Bool, Numb, Symb, Env, RuntimeError<Bool, Numb, Symb, Env>>>,
        RuntimeError<Bool, Numb, Symb, Env>,
    > {
        ls.into_iter()
            .rev()
            .map(|e| self.eval_loop(e, at))
            .collect()
    }

    fn eval_define(
        &mut self,
        s: Symb,
        mut ls: Vec<Exp<Bool, Numb, Symb>>,
        at: Env,
    ) -> EvalStep<Bool, Numb, Symb, Env> {
        EvalStep::Done(match (ls.pop(), ls.pop(), ls.pop()) {
            (Some(Exp::Symb(x)), Some(e), None) => {
                self.eval_loop(e, at)
                    .and_then(|v| match self.environment.define(at, x, v) {
                        Ok(()) => Ok(Val::Void()),
                        Err((x, _)) => Err(RuntimeError::AlreadyDefined(x)),
                    })
            }
            _ => Err(RuntimeError::BadFormedExpression(s)),
        })
    }

    fn eval_quote(
        &mut self,
        s: Symb,
        mut ls: Vec<Exp<Bool, Numb, Symb>>,
        at: Env,
    ) -> EvalStep<Bool, Numb, Symb, Env> {
        match (ls.pop(), ls.pop()) {
            (Some(e), None) => EvalStep::Loop(Exp::Quot(Box::new(e)), at),
            _ => EvalStep::Done(Err(RuntimeError::BadFormedExpression(s))),
        }
    }

    fn eval_if(
        &mut self,
        s: Symb,
        mut ls: Vec<Exp<Bool, Numb, Symb>>,
        at: Env,
    ) -> EvalStep<Bool, Numb, Symb, Env> {
        match (ls.pop(), ls.pop(), ls.pop(), ls.pop()) {
            (Some(c), Some(e1), Some(e2), None) => match self.eval_loop(c, at) {
                Ok(v) => EvalStep::Loop(if v.into() { e1 } else { e2 }, at),
                Err(err) => EvalStep::Done(Err(err)),
            },
            _ => EvalStep::Done(Err(RuntimeError::BadFormedExpression(s))),
        }
    }

    fn eval_begin(
        &mut self,
        _s: Symb,
        mut ls: Vec<Exp<Bool, Numb, Symb>>,
        at: Env,
    ) -> EvalStep<Bool, Numb, Symb, Env> {
        if ls.len() <= 1 {
            return match ls.pop() {
                None => EvalStep::Done(Ok(Val::Void())),
                Some(e) => EvalStep::Loop(e, at),
            };
        }

        EvalStep::Done(
            self.eval_args(ls, at)
                .map(|mut args| args.pop().unwrap_or_else(Val::Void)),
        )
    }

    fn eval_eval(
        &mut self,
        s: Symb,
        mut ls: Vec<Exp<Bool, Numb, Symb>>,
        at: Env,
    ) -> EvalStep<Bool, Numb, Symb, Env> {
        match (ls.pop(), ls.pop()) {
            (Some(e), None) => match self.eval_loop(e, at) {
                Ok(Val::Quot(e)) => EvalStep::Loop(e, at),
                Ok(e) => EvalStep::Done(Ok(e)),
                Err(err) => EvalStep::Done(Err(err)),
            },
            _ => EvalStep::Done(Err(RuntimeError::BadFormedExpression(s))),
        }
    }

    fn eval_lambda(
        &mut self,
        s: Symb,
        mut ls: Vec<Exp<Bool, Numb, Symb>>,
        at: Env,
    ) -> EvalStep<Bool, Numb, Symb, Env> {
        EvalStep::Done(match (ls.pop(), ls.pop(), ls.pop()) {
            (Some(Exp::List(ls)), Some(b), None) => {
                match ls.into_iter().map(Exp::symb).rev().collect() {
                    Some(ps) => Ok(Val::Lamb(ps, b, at.clone())),
                    None => Err(RuntimeError::BadFormedExpression(s)),
                }
            }
            _ => Err(RuntimeError::BadFormedExpression(s)),
        })
    }

    fn eval_app_procedure(
        &mut self,
        v: Val<Bool, Numb, Symb, Env, RuntimeError<Bool, Numb, Symb, Env>>,
        ls: Vec<Exp<Bool, Numb, Symb>>,
        at: Env,
    ) -> EvalStep<Bool, Numb, Symb, Env> {
        match v {
            Val::BuiltIn(f) => EvalStep::Done(self.eval_args(ls, at).and_then(f)),
            Val::Lamb(ps, b, at_lambda) => {
                if ps.len() != ls.len() {
                    return EvalStep::Done(Err(RuntimeError::ArityMismatch()));
                }

                match self.eval_args(ls, at) {
                    Err(err) => EvalStep::Done(Err(err)),
                    Ok(args) => match self.environment.push(at_lambda, ps.len()) {
                        None => EvalStep::Done(Err(RuntimeError::CouldNotPushEnvironment())),
                        Some(at) => {
                            if let Err((x, _)) = ps
                                .into_iter()
                                .zip(args)
                                .rev()
                                .map(|(x, v)| self.environment.define(at, x, v))
                                .collect::<Result<(), _>>()
                            {
                                return EvalStep::Done(Err(RuntimeError::AlreadyDefined(x)));
                            }

                            EvalStep::Loop(b, at)
                        }
                    },
                }
            }
            v => EvalStep::Done(Err(RuntimeError::NotAProcedure(v))),
        }
    }
}
