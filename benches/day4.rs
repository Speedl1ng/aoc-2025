use aoc_2025::fetch_aoc_input;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

const fn part1(input: &str) {
    //
}

const fn part2(input: &str) {
    //
}

fn bench_part1(c: &mut Criterion) {
    let input = fetch_aoc_input(2025, 4).expect("failed to fetch input");
    c.bench_with_input(
        BenchmarkId::new("day4 part1", "fetched_input"),
        &input,
        |b, s| b.iter(|| part1(s)),
    );
}

fn bench_part2(c: &mut Criterion) {
    let input = fetch_aoc_input(2025, 4).expect("failed to fetch input");
    c.bench_with_input(
        BenchmarkId::new("day4 part2", "fetched_input"),
        &input,
        |b, s| b.iter(|| part2(s)),
    );
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
