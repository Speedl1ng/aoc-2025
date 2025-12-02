use aoc_2025::read_input;
use criterion::{Criterion, criterion_group, criterion_main};

fn part1(input: &str) -> u64 {
    let mut result = 0;

    for range in input.split(',') {
        let (first, last): (u64, u64) = range
            .split_once('-')
            .map(|(f, l)| (f.parse().unwrap(), l.parse().unwrap()))
            .unwrap();

        result += (first..=last)
            .filter(|id| {
                let id_str = id.to_string();

                let (left, right) = id_str.split_at(id_str.len() / 2);

                left == right
            })
            .sum::<u64>();
    }

    result
}

fn part2(input: &str) -> u64 {
    let mut result: u64 = 0;

    for range in input.split(',') {
        let (first, last): (u64, u64) = range
            .split_once('-')
            .map(|(f, l)| (f.parse().unwrap(), l.parse().unwrap()))
            .unwrap();

        result += (first..=last)
            .filter(|id| {
                let id_str = id.to_string();

                for size in 1..=(id_str.len() / 2) {
                    let mut chunks = id_str.as_bytes().chunks(size);
                    if chunks
                        .next()
                        .map(|first| chunks.all(|c| c == first))
                        .unwrap()
                    {
                        return true;
                    }
                }

                false
            })
            .sum::<u64>();
    }

    result
}

fn bench_part1(c: &mut Criterion) {
    let input = read_input(2);
    let input = input.join("");

    let result = part1(&input);
    println!("day2 part1 result: {result}");

    c.bench_function("day2 part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    let input = read_input(2);
    let input = input.join("");

    let result = part2(&input);
    println!("day2 part2 result: {result}");

    c.bench_function("day2 part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
