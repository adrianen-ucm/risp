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
    pub mod eval;
    pub mod prelude;
    pub mod res;
    pub mod val;
}
