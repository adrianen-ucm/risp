use std::io::{self, Read};

use risp::{
    semantics::{built_in::EvalBuiltIn, env_tree::EnvironmentTree, eval::Evaluator},
    syntax::{
        parse::Parser,
        print::{PrintError, PrintWithSymbols},
        symb_interner::SymbolsInterner,
    },
};

fn main() {
    // TODO provisional interpreter
    let mut buf = Vec::new();
    let _ = io::stdin().read_to_end(&mut buf);
    let i = std::str::from_utf8(&buf).unwrap().to_string();

    let mut symbols = SymbolsInterner::new();

    let mut env = EnvironmentTree::empty(0);

    if let Err(x) = EvalBuiltIn::load_prelude(&mut env, &mut symbols) {
        println!("Error loading {x} from prelude");
        return;
    };

    // Parse expressions
    let mut parser = Parser::new(&mut symbols);
    let exs = parser.parse_all_exps::<bool, i64>(i.as_str()).unwrap().1;

    // Evaluate expressions
    let mut ev = Evaluator::new(&symbols, &mut env);
    for ex in exs {
        match ev.eval(ex) {
            Err(e) => match e.print_with(&symbols) {
                Err(PrintError::UnknownSymbol(s)) => {
                    println!("Unknown symbol when trying to print an error: {s:?}")
                }
                Ok(s) => println!("Error: {}", s.as_str()),
            },
            Ok(v) => match v.print_with(&symbols) {
                Err(PrintError::UnknownSymbol(s)) => {
                    println!("Unknown symbol when trying to print the result: {s:?}")
                }
                Ok(s) => println!("Result: {}", s.as_str()),
            },
        }
    }
}
