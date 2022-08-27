use std::io::{self, Read};

use risp::{
    semantics::{env_tree::EnvironmentTree, eval::Evaluator},
    syntax::{
        exp::Exp,
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

    let mut s = SymbolsInterner::new();
    let mut p = Parser { symbols: &mut s };

    let es: Vec<Exp<bool, i32, _>> = p.parse_exps(i.as_str()).unwrap().1;

    let mut c: EnvironmentTree<_, _> = EnvironmentTree::empty(0);
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
