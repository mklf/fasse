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
            return Err(FasseError::DimError {
                dim: self.meta.d,
                var: "x",
            });
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

    fn search(
        &self,
        x: &[f32],
        k: usize,
        dists: &mut [f32],
        labels: &mut [usize],
    ) -> Result<(), FasseError> {
        if x.len() % self.meta.d != 0 {
            return Err(FasseError::DimError {
                dim: self.meta.d,
                var: "x",
            });
        }
        let n = x.len() / self.meta.d;
        if dists.len() != n * k {
            return Err(FasseError::SizeError("dist", n * k, dists.len()));
        }
        if labels.len() != n * k {
            return Err(FasseError::SizeError("label", n * k, labels.len()));
        }

        match self.meta.metric {
            Metric::InnerProduct => {}
            Metric::L2 => {}
            Metric::Lp(_p) => {}
            _ => {
                return Err(FasseError::MetricError(self.meta.metric));
            }
        }

        Ok(())
    }
}
