# simd-euclidean

Blazing fast euclidean distance implementation, utilizing LLVM and Rust's auto-vectorization.

For vectors >= 32 elements, the SIMD-enabled algorithm is 2 to 8 times faster, with longer inputs provided greater speedups.

Two traits are exposed by this library, `Naive` and `Vectorized`, which both provide a `squared_distance` and `distance` function. 

```rust
use rand::prelude::*;
use simd_euclidean::*;

// Vectorized::distance will dispatch to Naive::distance for an input of this size
let v: f64 = Vectorized::distance([0.1, 0.2, 0.3, 0.4].as_ref(), [0.4, 0.3, 0.2, 0.1].as_ref());
let n: f64 = Naive::distance([0.1, 0.2, 0.3, 0.4].as_ref(), [0.4, 0.3, 0.2, 0.1].as_ref());
assert!((n - v).abs() < 1e-5);

for &i in [16, 32, 64, 128].into_iter() {
  // Dispatch to F32x4 or F32x8 (above 64 elements)
    let mut rng = rand::thread_rng();
    let a = (0..i).map(|_| rng.gen::<f32>()).collect::<Vec<f32>>();
    let b = (0..i).map(|_| rng.gen::<f32>()).collect::<Vec<f32>>();

    let v: f32 = Vectorized::distance(&a, &b);
    let n: f32 = Naive::distance(&a, &b);
    assert!((n - v).abs() < 1e-5);
}
```

The `Vectorized` trait attempts to heuristically determine which SIMD layout (F32x4, F32x8, etc) will be fastest with the given input size.

Shown below is a comparison between `Naive::distance` and `Vectorized::distance` on random vectors of single precision floating point numbers. 

![Benchmark, f32](linesf32.svg "Benchmark, f32")

And for double precision `f64` vectors:

![Benchmark, f64](linesf64.svg "Benchmark, f64")
