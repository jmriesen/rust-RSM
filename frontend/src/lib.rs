#![feature(array_chunks)]
use ir::commands::Command;
pub mod commands;
pub mod expression;
pub mod external_calls;
pub mod extrinsic_function;
pub mod intrinsic_functions;
pub mod intrinsic_var;
pub mod operators;
pub mod variable;

pub trait TreeSitterParser<'a> {
    type NodeType;
    fn new(sitter: &Self::NodeType, source_code: &str) -> Self;
}

pub type Routine = Vec<Vec<Command>>;

pub fn parse_routine(source_code: &str) -> Routine {
    let tree = lang_model::create_tree(dbg!(source_code));
    let tree = lang_model::type_tree(&tree, source_code).unwrap();

    let tags = tree.children();
    let block = tags[0].block().unwrap();
    let mut lines = vec![];
    assert_eq!(block.children().len(), 1, "unimplemented");
    for line in block.children() {
        let line = match line {
            lang_model::BlockChildren::line(line) => line,
            lang_model::BlockChildren::Block(_line) => unimplemented!(),
        };
        let mut commands = vec![];
        let mut line_tail = line.children().into_iter();
        while let Some(command) = line_tail.next() {
            commands.push(commands::new(&command, source_code, &mut line_tail));
        }
        lines.push(commands);
    }
    lines
}

pub fn wrap_command_in_routine(source_code: &str) -> Routine {
    let source_code = source_code.replace('\n', "\n ");
    let source_code = &format!("tag {source_code}\n");
    parse_routine(source_code)
}

pub fn command_from_source(source_code: &str) -> Command {
    let commands = wrap_command_in_routine(source_code);
    commands
        .into_iter()
        .flatten()
        .next()
        .expect("there should only be one command pressent")
}
