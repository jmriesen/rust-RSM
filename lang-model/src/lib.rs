/*
 * Package: Rust Reference Standard M
 *
 * Jacob Riesen <jacobriesen@gmail.com>
 * https://github.com/jmriesen/rust-RSM
 *
 * Based on Reference Standard M by David Wicksell
 * Copyright © 2020-2024 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Which was based on MUMPS V1 by Raymond Douglas Newman
 * Copyright © 1999-2018
 * https://gitlab.com/Reference-Standard-M/mumpsv1
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Affero General Public License (AGPL) as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero
 * General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see https://www.gnu.org/licenses/.
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */
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
