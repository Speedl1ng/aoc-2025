use aoc_2025::fetch_aoc_input;
use criterion::{Criterion, criterion_group, criterion_main};

const fn part1(_input: &str) {
    //
}

const fn part2(_input: &str) {
    //
}

fn bench_part1(c: &mut Criterion) {
    let input = fetch_aoc_input(1);

    // let result = part1(&input);
    // println!("day4 part1 result: {result}");

    c.bench_function("day4 part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    let input = fetch_aoc_input(1);

    // let result = part2(&input);
    // println!("day4 part2 result: {result}");

    c.bench_function("day4 part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
