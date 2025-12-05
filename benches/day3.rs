use std::{cmp::Ordering, mem::swap};

use aoc_2025::fetch_aoc_input;
use criterion::{Criterion, criterion_group, criterion_main};

fn part1(input: &str) -> u64 {
    let lines = input.lines();
    let mut res = 0;
    for line in lines {
        let len = line.len();

        let (mut h1, mut h2) = ((0u8, -1i16), (0u8, -1i16));
        for (index, char) in line.bytes().enumerate() {
            let number: u8 = char - 48;
            if number > h2.0 {
                h2.0 = number;
                h2.1 = index as i16;
                if h2.0 > h1.0 {
                    swap(&mut h1, &mut h2);
                    if index != len - 1 {
                        h2.0 = 0;
                    }
                }
            }
        }

        let (left, right) = if h1.1 > h2.1 {
            (h2.0, h1.0)
        } else {
            (h1.0, h2.0)
        };

        let joltage = (left * 10) + right;
        res += joltage as u64;
    }

    res
}

fn part2(input: &str) -> u64 {
    let lines = input.lines();
    let mut res = 0;
    for line in lines {
        let bytes = line.as_bytes();
        let mut array = [0u8; 12];
        let len = bytes.len();
        if len == 0 {
            break;
        }
        let mut current_index = 0;
        for i in 0..11 {
            let (index, value) = bytes[current_index..(bytes.len() - 11 + i)]
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| match a.cmp(b) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => Ordering::Greater,
                    other => other,
                })
                .unwrap();

            array[i] = *value - 48;
            current_index = current_index + index + 1;
        }

        let (_index, value) = bytes[current_index..]
            .iter()
            .enumerate()
            .max_by_key(|(_index, value)| *value)
            .unwrap();
        array[11] = *value - 48;

        array.reverse();

        let mut joltage = 0;
        for (index, value) in array.iter().enumerate() {
            let value = *value as usize;
            let m = value * (10usize.pow(index as u32).max(1));
            joltage += m;
        }

        res += joltage as u64;
    }

    res
}

fn bench_part1(c: &mut Criterion) {
    let example = include_str!("../input/example.txt");
    assert_eq!(part1(example), 357);

    let input = fetch_aoc_input(2025, 3).expect("failed to fetch input");
    let result = part1(&input);
    println!("day3 part1 result: {result}");
    c.bench_function("day3 part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    let example = include_str!("../input/example.txt");
    assert_eq!(part2(example), 3121910778619);

    let input = fetch_aoc_input(2025, 3).expect("failed to fetch input");
    let result = part2(&input);
    println!("day3 part2 result: {result}");
    c.bench_function("day3 part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
