pub mod syntax {
    pub mod exp;
    pub mod parse;
    pub mod print;
    pub mod symb;
    pub mod symb_interner;
}

// TODO builtin prelude
pub mod semantics {
    pub mod env;
    pub mod env_tree;
    pub mod err;
    pub mod eval; // TODO verify environment cleaning
    pub mod val; // TODO abstract builtin procedures to allow symbol/environment access
}
