use super::Counter;
use fnv::FnvBuildHasher;

/**
A faster but less safe version of Counter.

By default, HashMap uses a hashing algorithm selected to provide resistance against HashDoS attacks.
Counter does it too.
This algorithm is unfortunately slow.
For the most algorithmic challenges faster and less safe algorithms are often preferred.
FastCounter uses the popular ```fnv::FnvBuildHasher```.
*/
pub type FastCounter<T> = Counter<T, FnvBuildHasher>;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _cnt: FastCounter<i32> = FastCounter::new();
    }
}