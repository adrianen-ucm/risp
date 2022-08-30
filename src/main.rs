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
    let mut buffer = Vec::new();
    if let Err(err) = io::stdin().read_to_end(&mut buffer) {
        println!("Error reading input: {err}");
        return;
    }

    let input = match std::str::from_utf8(&buffer) {
        Ok(input) => input,
        Err(err) => {
            println!("Error decoding input: {err}");
            return;
        }
    };

    let mut symbols = SymbolsInterner::new();
    let mut environment = EnvironmentTree::empty(0);
    if let Err(x) = EvalBuiltIn::load_prelude(&mut environment, &mut symbols) {
        println!("Error loading {x} from prelude");
        return;
    };

    let mut parser = Parser::new(&mut symbols);
    let program = match parser.parse_all_exps::<bool, i64>(input) {
        Ok((_, program)) => program,
        Err(err) => {
            println!("Error parsing the program: {err}");
            return;
        }
    };

    let mut evaluator = Evaluator::new(&symbols, &mut environment);
    for expression in program {
        if let Err(err) = evaluator.eval(expression) {
            match err.print_with(&symbols) {
                Err(PrintError::UnknownSymbol(s)) => {
                    println!("Unknown symbol when trying to print a runtime error: {s:?}")
                }
                Ok(err) => {
                    println!("Runtime error: {err}")
                }
            }
        }
    }
}
