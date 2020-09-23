use std::cmp::Ordering;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Debug)]
pub struct HeapKey {
    dist: f32,
    idx: usize,
}

impl Ord for HeapKey {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.dist.partial_cmp(&self.dist) {
            Some(order) => order,
            None => Ordering::Less,
        }
    }
}

impl Eq for HeapKey {}

impl PartialEq for HeapKey {
    fn eq(&self, other: &Self) -> bool {
        (self.dist - other.dist).abs() < 1e-9
    }
}

impl PartialOrd for HeapKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

#[cfg(test)]
mod test {
    use super::HeapKey;
    use std::collections::BinaryHeap;
    #[test]
    fn test_heap_key() {
        let a = HeapKey { dist: 0.1, idx: 0 };
        let b = HeapKey { dist: 0.13, idx: 1 };
        let c = HeapKey { dist: 0.12, idx: 3 };
        let d = HeapKey { dist: 0.12, idx: 4 };

        assert!(a > b);
        assert!(c > b);
        assert_eq!(c, d);
        assert_eq!(c, c);

        let mut heap = BinaryHeap::new();
        heap.push(a);
        heap.push(b);
        heap.push(c);
        heap.push(d);

        let values: Vec<_> = heap.iter().collect();
        println!("{:?}", values);
    }
}
