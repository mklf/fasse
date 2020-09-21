use crate::index::{Index, IndexData, Metric};
use crate::FasseError;

pub struct IndexFlat {
    meta: IndexData,
    xb: Vec<f32>,
}

impl IndexFlat {
    pub fn new(d: usize, metric: Metric) -> IndexFlat {
        IndexFlat {
            meta: IndexData {
                d,
                ntotal: 0,
                is_trained: true,
                metric,
            },
            xb: Vec::new(),
        }
    }
}

impl Index for IndexFlat {
    fn add(&mut self, x: &[f32]) -> Result<(), FasseError> {
        if x.len() % self.meta.d != 0 {
            return Err(FasseError::DimError { dim: self.meta.d });
        }
        self.xb.extend_from_slice(x);
        self.meta.ntotal += x.len() / self.meta.d;
        Ok(())
    }

    fn is_trained(&self) -> bool {
        self.meta.is_trained
    }
    fn ntotal(&self) -> usize {
        self.meta.ntotal
    }
}
