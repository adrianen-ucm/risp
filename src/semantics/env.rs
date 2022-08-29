/// Types that can be used to manage environments, which are variable-value associations with inheritance.
pub trait Environments<Var, Val> {
    /// A reference to an environment.
    type Env;

    /// The root environment.
    fn root(&self) -> Self::Env;

    /// Checks if a given environment has any child.
    fn has_children(&self, at: Self::Env) -> bool;

    /// Drops the given environment and its children.
    fn drop(&mut self, at: Self::Env);

    /// Creates and returns a new child for a given environment.
    fn push(&mut self, at: Self::Env, capacity: usize) -> Option<Self::Env>;

    /// Find the `Val` associated to a `Var` for a given environment.
    fn get(&self, at: Self::Env, x: &Var) -> Option<&Val>;

    /// Associates a `Val` to a `Var` for a given environment, only the `Var` was
    /// not previously associated in the same environment.
    ///
    /// Returns the ownership of the given paremeters on failure.
    fn define(&mut self, at: Self::Env, x: Var, v: Val) -> Result<(), (Var, Val)>;
}
