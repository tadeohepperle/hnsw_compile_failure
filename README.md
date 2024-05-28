This repo shows a strange rust compile error:

```
error[E0599]: the method `insert` exists for struct `Hnsw<Euclidean, &[f32], ChaCha20Rng, 10, 10>`, but its trait bounds were not satisfied
  --> src/main.rs:18:10
   |
1  | struct Euclidean;
   | ---------------- doesn't satisfy `Euclidean: space::Metric<&[f32]>`
...
18 |     hnsw.insert(q, &mut searcher);
   |          ^^^^^^ method cannot be called on `Hnsw<Euclidean, &[f32], ChaCha20Rng, 10, 10>` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `Euclidean: space::Metric<&[f32]>`
note: the trait `space::Metric` must be implemented
  --> /home/tadeo/.cargo/registry/src/index.crates.io-6f17d22bba15001f/space-0.17.0/src/lib.rs:50:1
   |
50 | pub trait Metric<P> {
   | ^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0599`.
error: could not compile `hnsw_compile_failure` (bin "hnsw_compile_failure") due to 1 previous error
```

Rust version:

```
stable-x86_64-unknown-linux-gnu (default)
rustc 1.77.2 (25ef9e3d8 2024-04-09)
```