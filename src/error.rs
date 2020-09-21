use thiserror::Error;

#[derive(Error, Debug)]
pub enum FasseError {
    #[error("dimension is not a multiple of {dim}")]
    DimError { dim: usize },
}
