use std::{
    collections::{BTreeMap, HashMap, HashSet},
    mem::size_of,
};
const SIZE_OF_PTR: usize = size_of::<*mut c_void>();

use libc::c_void;

#[derive(PartialEq, Eq, Debug, Default)]
pub struct DifferencesList(BTreeMap<usize, (u8, u8)>);

impl DifferencesList {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert_int(&mut self, left: i32, right: i32, index: usize) {
        let left = left.to_ne_bytes().into_iter();
        let right = right.to_ne_bytes().into_iter();
        for (i, (left, right)) in left.zip(right).enumerate() {
            if left != right {
                self.0.insert(index + i, (left, right));
            }
        }
    }
}

pub fn test_memory_segment_equality<'a>(left: &'a [u8], right: &'a [u8]) -> DifferencesList {
    let left_base = std::ptr::from_ref(left).cast::<c_void>() as isize;
    let right_base = std::ptr::from_ref(right).cast::<c_void>() as isize;
    assert_eq!(left.len(), right.len());
    let mut iter = left
        .array_windows::<SIZE_OF_PTR>()
        .zip(right.array_windows::<SIZE_OF_PTR>())
        .enumerate();

    let mut errors = vec![];
    while let Some((index, (left, right))) = iter.next() {
        //Try treating the data as a pointer and calculate there offsets from the baseline
        let left_ptr_offset = isize::from_le_bytes(*left).checked_sub(left_base);
        let right_ptr_offset = isize::from_le_bytes(*right).checked_sub(right_base);

        if left_ptr_offset == right_ptr_offset && left_ptr_offset.is_some() {
            //continue
            //advance the iterator and continue.
            for _ in 1..SIZE_OF_PTR {
                iter.next();
            }
        } else if left[0] == right[0] {
            // continue
        } else {
            errors.push(index);
        }
    }
    //TODO note we are not actually checking the last SIZE-1 bytes.
    DifferencesList(
        errors
            .iter()
            .map(|index| (*index, (left[*index], right[*index])))
            .collect(),
    )
}