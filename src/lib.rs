pub static VERSION: &str = env!("CARGO_PKG_VERSION");

mod error;

pub use error::FasseError;
mod index;

pub use index::Index;
pub use index::Metric;

pub use index::IndexFlat;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
