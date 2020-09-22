pub mod index_base;
pub use index_base::{Index, IndexData, Metric};
mod index_flat;
pub mod collections;

pub use index_flat::IndexFlat;
