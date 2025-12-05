use aoc_2025::{fetch_aoc_input, next_even_len_down, next_even_len_up};
use criterion::{Criterion, criterion_group, criterion_main};
use rayon::{
    iter::{ParallelBridge, ParallelIterator},
    str::ParallelString,
};

fn part1(input: &str) -> u64 {
    let iter = input.trim().par_split(',').map(|split| {
        let mut s = split.split('-');
        let left = s.next().unwrap();
        let right = s.next().unwrap();

        (next_even_len_up(left), next_even_len_down(right))
    });

    iter.map(|ranges| {
        (ranges.0..=ranges.1)
            .par_bridge()
            .map(|x| {
                let s = x.to_string();
                let len = s.len() / 2;
                if s[..len] == s[len..] {
                    return x;
                }
                0
            })
            .sum::<u64>()
    })
    .sum::<u64>()
}

fn part2(input: &str) -> u64 {
    let iter = input.trim().split(',').map(|split| {
        let mut s = split.split('-');
        let left = s.next().unwrap();
        let right = s.next().unwrap();

        // (next_even_len_up(left), next_even_len_down(right))
        (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap())
    });

    iter.map(|ranges| {
        (ranges.0..=ranges.1)
            .par_bridge()
            .filter(|x| {
                let s = x.to_string();
                let len = s.len() / 2;
                for size in 1..=len {
                    let mut chunks = s.as_bytes().chunks(size);
                    let first = chunks.next().unwrap();
                    if chunks.all(|chunk| chunk == first) {
                        return true;
                    }
                }
                false
            })
            .sum::<u64>()
    })
    .sum::<u64>()
}

fn bench_part1(c: &mut Criterion) {
    let example = include_str!("../input/example.txt");
    assert_eq!(part1(example), 1227775554);

    let input = fetch_aoc_input(2025, 2).expect("failed to fetch input");
    let result = part1(&input);
    println!("day2 part1 result: {result}");

    c.bench_function("day2 part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    let example = include_str!("../input/example.txt");
    assert_eq!(part2(example), 4174379265);

    let input = fetch_aoc_input(2025, 2).expect("failed to fetch input");
    let result = part2(&input);
    println!("day2 part2 result: {result}");
    c.bench_function("day2 part2", |b| b.iter(|| part1(&input)));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
