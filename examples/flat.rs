use fasse::FasseError;
use fasse::Index;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

fn main() -> Result<(), FasseError> {
    let d = 64;
    let nb = 100000;
    let nq = 10000;

    let rng = thread_rng();
    let distrib = Uniform::new(0f32, 1f32);

    let mut xb: Vec<f32> = rng.sample_iter(distrib).take(d * nb).collect();

    let mut xq: Vec<f32> = rng.sample_iter(distrib).take(d * nq).collect();

    for i in 0..nb {
        xb[d * i] += i as f32 / 1000.;
    }
    for i in 0..nq {
        xq[d * i] += i as f32 / 1000.;
    }
    let mut index = fasse::IndexFlat::new(d, fasse::Metric::L2);

    println!("is_trained = {}", index.is_trained());
    index.add(&xb)?;
    println!("ntotal= {}", index.ntotal());

    Ok(())
}
