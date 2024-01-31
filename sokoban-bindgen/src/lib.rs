pub mod hash_table;
pub mod red_black_tree;
pub use concat_idents::concat_idents as ____ci;
pub use hash_table::*;
pub use red_black_tree::*;

pub const SUCCESS: u32 = 0;
pub const FAILURE: u32 = u32::MAX;
