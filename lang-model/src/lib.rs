mod models;
pub use models::*;
//Remove to seporate repo.
pub fn create_tree(source_code: &str) -> tree_sitter::Tree {
    use tree_sitter::Parser;
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_mumps::language()).unwrap();

    #[cfg(test)]
    let tree = parser.parse(source_code, None).unwrap();
    #[cfg(test)]
    dbg!(&tree.root_node().to_sexp());

    parser.parse(source_code, None).unwrap()
}

///M Specific Keep in RSM
impl<'a> Expression<'a> {
    pub fn is_inderect(&self) -> bool {
        matches!(self.children(), ExpressionChildren::InderectExpression(_))
    }
}
///M Specific Keep in RSM
impl<'a> command<'a> {
    pub fn argumentless(&self) -> bool {
        use commandChildren as E;
        match self.children() {
            E::WriteCommand(command) => command.args().is_empty(),
            E::BrakeCommand(command) => command.args().is_empty(),
            E::CloseCommand(command) => command.args().is_empty(),
            E::For(command) => command.args().is_empty(),
            E::DoCommand(command) => command.args().is_empty(),
            E::ElseCommand(_) => true,
            E::NewCommand(_) => true,
            E::QUITCommand(_) => true,
        }
    }
}
