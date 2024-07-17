use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use french_strings::{GermanString};
use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;

fn german_string_comp(string_a: &str, string_b: &str) -> bool {
    let mut str_a = GermanString::new();
    str_a.push_str(string_a);
    let mut str_b = GermanString::new();
    str_b.push_str(string_b);
    return str_a == str_b
}
fn normal_string_comp(string_a: &str, string_b: &str) -> bool {
    let string_a = String::from(string_a);
    let string_b = String::from(string_b);
    return string_a == string_b
}



pub fn strings_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Strings");
    for i in 0..2 {
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(0..12);
        let string_a = Alphanumeric.sample_string(&mut rng, size);
        let random_bool: bool = rng.gen_bool(0.5);
        let string_b = if random_bool {
            Alphanumeric.sample_string(&mut rng, size)
        } else {
            string_a.clone()
        };
        group.bench_with_input(BenchmarkId::new("French Strings", i), &i,
                               |b, i| b.iter(|| german_string_comp(&string_a, &string_b)));
        group.bench_with_input(BenchmarkId::new("American Strings", i), &i,
                               |b, i| b.iter(|| normal_string_comp(&string_a, &string_b)));
    }
    group.finish();
}

criterion_group!(benches, strings_benchmark);
criterion_main!(benches);