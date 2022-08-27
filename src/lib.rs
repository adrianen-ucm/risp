pub mod syntax {
    pub mod exp;
    pub mod parse; // TODO abstract as with print and document
    pub mod print;
    pub mod symb;
    pub mod symb_interner;
}

// TODO builtin prelude
pub mod semantics {
    pub mod env;
    pub mod env_tree;
    pub mod err; // TODO document
    pub mod eval; // TODO document and verify
    pub mod val; // TODO abstract builtin procedures to allow symbol/environment access
}
