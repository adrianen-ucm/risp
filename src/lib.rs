pub mod syntax {
    pub mod exp;
    pub mod parse;
    pub mod print;
    pub mod symb;
    pub mod symb_interner;
}

pub mod semantics {
    pub mod built_in;
    pub mod env;
    pub mod env_tree;
    pub mod err;
    pub mod eval; // TODO verify environment cleaning
    pub mod prelude; // TODO lazy evaluation of and and or
    pub mod res;
    pub mod val;
}
