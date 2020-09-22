pub mod index_base;
pub use index_base::{Index, IndexData, Metric};
pub mod collections;
mod index_flat;

pub use index_flat::IndexFlat;
