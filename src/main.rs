use std::io::{self, Read};

use risp::{
    semantics::{
        built_in::EvalBuiltIn, env::Environments, env_tree::EnvironmentTree, eval::Evaluator,
        val::Val,
    },
    syntax::{
        parse::Parser,
        print::{PrintError, PrintWithSymbols},
        symb::Symbols,
        symb_interner::SymbolsInterner,
    },
};

fn main() {
    // TODO provisional interpreter
    let mut buf = Vec::new();
    let _ = io::stdin().read_to_end(&mut buf);
    let i = std::str::from_utf8(&buf).unwrap().to_string();

    let mut s = SymbolsInterner::new();
    let mut p = Parser::new(&mut s);

    let es = p.parse_all_exps::<bool, i64>(i.as_str()).unwrap().1;

    let mut c = EnvironmentTree::empty(0);

    _ = c.define(c.root(), s.get_or_store("true"), Val::Bool(true));
    _ = c.define(c.root(), s.get_or_store("false"), Val::Bool(false));
    _ = c.define(
        c.root(),
        s.get_or_store("+"),
        Val::BuiltIn(EvalBuiltIn::new(EvalBuiltIn::add)),
    );

    _ = c.define(
        c.root(),
        s.get_or_store("*"),
        Val::BuiltIn(EvalBuiltIn::new(EvalBuiltIn::mul)),
    );

    _ = c.define(
        c.root(),
        s.get_or_store("-"),
        Val::BuiltIn(EvalBuiltIn::new(EvalBuiltIn::sub)),
    );

    _ = c.define(
        c.root(),
        s.get_or_store("="),
        Val::BuiltIn(EvalBuiltIn::new(EvalBuiltIn::et)),
    );

    _ = c.define(
        c.root(),
        s.get_or_store(">"),
        Val::BuiltIn(EvalBuiltIn::new(EvalBuiltIn::gt)),
    );

    _ = c.define(
        c.root(),
        s.get_or_store("<"),
        Val::BuiltIn(EvalBuiltIn::new(EvalBuiltIn::lt)),
    );

    _ = c.define(
        c.root(),
        s.get_or_store(">="),
        Val::BuiltIn(EvalBuiltIn::new(EvalBuiltIn::gte)),
    );

    _ = c.define(
        c.root(),
        s.get_or_store("<="),
        Val::BuiltIn(EvalBuiltIn::new(EvalBuiltIn::lte)),
    );

    _ = c.define(
        c.root(),
        s.get_or_store("not"),
        Val::BuiltIn(EvalBuiltIn::new(EvalBuiltIn::not)),
    );

    _ = c.define(
        c.root(),
        s.get_or_store("and"),
        Val::BuiltIn(EvalBuiltIn::new(EvalBuiltIn::and)),
    );

    _ = c.define(
        c.root(),
        s.get_or_store("or"),
        Val::BuiltIn(EvalBuiltIn::new(EvalBuiltIn::or)),
    );

    _ = c.define(
        c.root(),
        s.get_or_store("eq?"),
        Val::BuiltIn(EvalBuiltIn::new(EvalBuiltIn::eq)),
    );

    _ = c.define(
        c.root(),
        s.get_or_store("newline"),
        Val::BuiltIn(EvalBuiltIn::new(EvalBuiltIn::newline)),
    );

    _ = c.define(
        c.root(),
        s.get_or_store("display"),
        Val::BuiltIn(EvalBuiltIn::new(EvalBuiltIn::display)),
    );

    let mut e = Evaluator::new(&s, &mut c);

    for ex in es {
        match e.eval(ex) {
            Err(e) => match e.print_with(&s) {
                Err(PrintError::UnknownSymbol(s)) => {
                    println!("Unknown symbol when trying to print an error: {s:?}")
                }
                Ok(s) => println!("Error: {}", s.as_str()),
            },
            Ok(v) => match v.print_with(&s) {
                Err(PrintError::UnknownSymbol(s)) => {
                    println!("Unknown symbol when trying to print the result: {s:?}")
                }
                Ok(s) => println!("Result: {}", s.as_str()),
            },
        }
    }
}
