use crate::FasseError;
pub enum Metric {
    InnerProduct,
    L2,
    L1,
    Linf,
    Lp(f32),
}

pub struct IndexData {
    pub(crate) d: usize,      // vector dimension
    pub(crate) ntotal: usize, // total nb of indexed vectors
    pub(crate) is_trained: bool,
    #[allow(dead_code)]
    pub(crate) metric: Metric,
}

pub trait Index {
    // training on a set of vectors
    #[allow(unused_variables)]
    fn train(&mut self, x: &[f32]) {}

    fn add(&mut self, x: &[f32]) -> Result<(), FasseError>;

    #[allow(unused_variables)]
    fn add_with_ids(&mut self, x: &[f32], xids: &[usize]) -> Result<(), FasseError> {
        unimplemented!("add_with_ids");
    }

    fn is_trained(&self) -> bool {
        unimplemented!("is_trained");
    }

    fn ntotal(&self) -> usize {
        unimplemented!("ntotal");
    }

    /// x: vectors to search size n * d
    /// k: size of neareast neighbors(NN) for each vector
    /// labels: output labels of the NNs
    // dist: pairwise distances
    fn search(
        &self,
        x: &[f32],
        k: usize,
        dists: &mut [f32],
        labels: &mut [usize],
    ) -> Result<(), FasseError>;
}
