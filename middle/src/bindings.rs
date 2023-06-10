//bingen dose not seem to handle size of properly
pub const MAX_MAP_SIZE: u32 =
    (MAX_DATABASE_BLKS / 8 + (std::mem::size_of::<label_block>() as u32)) / 1024 + 1;
pub const IDX_START: u16 = (std::mem::size_of::<DB_Block>() as u16) / 2;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
