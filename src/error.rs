use thiserror::Error;

#[derive(Error, Debug)]
pub enum FasseError {
    #[error("dimension is not a multiple of {dim} for {var}")]
    DimError { dim: usize, var: &'static str },
    #[error("{0} size mismatch, expect {1}, got {2}")]
    SizeError(&'static str, usize, usize)
}
