use tower_lsp::lsp_types::Position;
use tree_sitter::Point;

pub fn to_lsp_int(num: usize) -> u32 {
    num.try_into().expect("The LSP protical only allows for specifying position using 32 bit integers. If you somehow have overflowed a i32::max, sorry, please restrucutre your program.")
}

pub trait PointExt {
    fn to_position(&self) -> Position;
}

impl PointExt for Point {
    fn to_position(&self) -> Position {
        Position {
            line: to_lsp_int(self.row),
            character: to_lsp_int(self.column),
        }
    }
}
