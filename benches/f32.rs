use criterion::*;
use symagen::random_data;

use simd_euclidean::*;

fn bench_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("SimdF32");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for i in 0..=10 {
        let dimensionality = 2_usize.pow(i);
        let data = random_data::random_f32(2, dimensionality, -1e10, 1e10, 42);
        let (x, y) = (&data[0], &data[1]);

        let id = BenchmarkId::new("naive", dimensionality);
        group.bench_with_input(id, &dimensionality, |b, _| {
            b.iter(|| black_box(Naive::distance(x, y)))
        });

        let id = BenchmarkId::new("simd", dimensionality);
        group.bench_with_input(id, &dimensionality, |b, _| {
            b.iter(|| black_box(Vectorized::distance(x, y)))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_random);
criterion_main!(benches);
