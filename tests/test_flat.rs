use fasse::Index;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
#[test]
fn index_flat_l2() {
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

    assert!(index.is_trained());
    assert!(index.add(&xb).is_ok());
    assert_eq!(index.ntotal(), nb);

    assert!(index.add(&xb[1..]).is_err());

    const K: usize = 4;
    const N: usize = 5;

    let mut dist = [0f32; K * N];
    let mut labels = [0usize; K * N];

    assert!(index
        .search(&xb[..d * N + 1], K, &mut dist, &mut labels)
        .is_err());

    assert!(index
        .search(&xb[..d * N], K, &mut dist[1..], &mut labels)
        .is_err());

    assert!(index
        .search(&xb[..d * N], K, &mut dist, &mut labels[1..])
        .is_err());

    assert!(index
        .search(&xb[..d * N], K, &mut dist, &mut labels)
        .is_ok());
}
