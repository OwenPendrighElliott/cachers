# arcache

[![Docs.rs](https://docs.rs/arcache/badge.svg)](https://docs.rs/arcache)
[![Crates.io](https://img.shields.io/crates/v/arcache.svg)](https://crates.io/crates/arcache)
![Test Status](https://github.com/OwenPendrighElliott/arcache/actions/workflows/tests.yml/badge.svg)
[![License](https://img.shields.io/crates/l/arcache.svg)](https://crates.io/crates/arcache)

A crate which implements a variety of caches with different eviction policies. All cache implementations are thread-safe and can be used in a multi-threaded environment. Cache implementations all share the `Cache` trait which means that they are interchangeable once instantiated.

The cache store values in an `Arc` (hence `arccache`) so that they can be shared between threads without needing to clone the value. Caches all implement an internal mutability pattern to make them easy to use in multi-threaded applications.

```rust
use arcache::{Cache, LRUCache};

fn main() {
    let cache = LRUCache::<&str, String>::new(10); // mutability is internally handled so you can use `let` instead of `let mut`
    
    // like std::collections::HashMap, you can use the `set` returns the previous value if it exists
    let original_value = cache.set("key", "value".to_string());

    assert!(original_value.is_none());
    
    // get returns an Option<Arc<V>> where V is the value type
    let value = cache.get(&"key");

    assert!(value.is_some());
    assert_eq!(*value.unwrap(), "value".to_string()); // value is wrapped in an Arc so you need to dereference it
    println!("{:?}", cache.stats());
}
```

The `Cache` trait lets you write functions with generic signatures and swap cache implementations, this is useful if you want to uses multiple cache types with the same code.

```rust
use arcache::{Cache, LFUCache, LRUCache};
use std::{hash::Hash, sync::Arc};

fn do_something<C>(cache: C)
where
    C: Cache<&'static str, String>,
{
    cache.set("hello", "world".to_string());
    if let Some(val) = cache.get(&"hello") {
        println!("Got: {}", val);
    }
}

fn main() {
    let lru_cache = LRUCache::<&'static str, String>::new(2);
    do_something(lru_cache);
    let lfu_cache = LFUCache::<&'static str, String>::new(2);
    do_something(lfu_cache);
}
```

## Implemented caches

+ `LRUCache`
+ `LFUCache`
+ `MRUCache`
+ `TTLCache`
+ `FIFOCache`
+ `LIFOCache`
+ `RandomReplacementCache`

### On the roadmap

+ `ARCCache`
+ `LFUTTLCache` (LFU with expiration)

## Usage

See `/examples` for example usage. You can run these like so:

```bash
cargo run --example fifo_example --release
cargo run --example lfu_example --release
cargo run --example lru_example --release
cargo run --example lru_fib_timed_example --release
cargo run --example multithreaded_lru_example --release
cargo run --example multithreaded_ttl_example --release
```

To add `arcache` to your project run `cargo add arcache`.