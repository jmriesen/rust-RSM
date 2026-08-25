use tower_lsp::lsp_types::Position;
use tree_sitter::{Point, StreamingIterator};

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
pub fn collect<T: Clone>(mut iter: impl StreamingIterator<Item = T>) -> Vec<T> {
    let mut vec = vec![];
    while let Some(item) = iter.next() {
        vec.push(item.clone());
    }
    vec
}
