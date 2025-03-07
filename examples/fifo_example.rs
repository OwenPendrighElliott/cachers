use cachers::{Cache, FIFOCache};

fn fifo_fib(n: u64, cache: &mut FIFOCache<u64, u64>) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    match cache.get(&n) {
        Some(v) => *v,
        None => {
            let result = fifo_fib(n - 1, cache) + fifo_fib(n - 2, cache);
            cache.set(n, result);
            result
        }
    }
}

fn main() {
    let mut cache = FIFOCache::new(10);
    println!("{}", fifo_fib(10, &mut cache));
    println!("{:?}", cache.stats());
    println!("{}", fifo_fib(10, &mut cache));
    println!("{:?}", cache.stats());
    println!("{}", fifo_fib(20, &mut cache));
    println!("{:?}", cache.stats());
}
