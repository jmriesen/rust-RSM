use tree_sitter::Node;
lang_model_macro::models!();

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
//TODO fix this lint.
//This will require some major work to handle the error case properly.
#[allow(clippy::result_unit_err)]
pub fn type_tree<'a>(
    tree: &'a tree_sitter::Tree,
    source_code: &'a str,
) -> Result<source_file<'a>, ()> {
    use tree_sitter::{Query, QueryCursor};
    let mut query_cursor = QueryCursor::new();
    let error_query = Query::new(tree_sitter_mumps::language(), "(ERROR)").unwrap();
    let errors = query_cursor.matches(&error_query, tree.root_node(), source_code.as_bytes());
    if errors.count() != 0 {
        Err(())
    } else {
        Ok(source_file::create(tree.root_node()))
    }
}

