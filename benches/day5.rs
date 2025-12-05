#![feature(new_range_api)]
use std::range::RangeInclusive;

use aoc_2025::fetch_aoc_input;
use criterion::{Criterion, criterion_group, criterion_main};

fn part1(input: &str) -> u64 {
    let mut lines = input.split("\n\n");
    let ingredient_database = lines.next().unwrap();
    let ingredient_list = lines.next().unwrap();

    // let mut set = HashSet::new();
    let mut ranges: Vec<RangeInclusive<usize>> = Vec::new();
    for line in ingredient_database.lines() {
        let mut s = line.trim().split('-');
        let left = s.next().unwrap().parse::<usize>().unwrap();
        let right = s.next().unwrap().parse::<usize>().unwrap();
        let range = std::range::RangeInclusive {
            start: left,
            last: right,
        };
        ranges.push(range);
    }
    ranges = merge_ranges(ranges);

    let mut valid = 0;
    for line in ingredient_list.lines() {
        let line = line.trim();
        let id = line.parse::<usize>().unwrap();
        let is_in_range = ranges
            .iter()
            .any(|range| range.start <= id && id <= range.last);
        if is_in_range {
            valid += 1;
        }
    }

    valid
}

fn part2(input: &str) -> u64 {
    let mut lines = input.split("\n\n");
    let ingredient_database = lines.next().unwrap();

    let mut ranges: Vec<RangeInclusive<usize>> = Vec::new();
    for line in ingredient_database.lines() {
        let mut s = line.trim().split('-');
        let left = s.next().unwrap().parse::<usize>().unwrap();
        let right = s.next().unwrap().parse::<usize>().unwrap();
        let new_range = std::range::RangeInclusive {
            start: left,
            last: right,
        };
        let mut adjusted = false;
        for range in ranges.iter_mut() {
            if range.start <= new_range.start
                && new_range.start <= range.last
                && new_range.last > range.last
            {
                range.last = new_range.last;
                adjusted = true;
            }
            if range.start <= new_range.last
                && new_range.last <= range.last
                && new_range.start < range.start
            {
                range.start = new_range.start;
                adjusted = true;
            }
        }

        if !adjusted {
            ranges.push(new_range);
        }
    }

    ranges = merge_ranges(ranges);

    let mut count = 0;
    for range in ranges {
        let len = range.last - range.start + 1;
        count += len;
    }

    // println!("{ranges:?}");
    count as u64
}

fn bench_part1(c: &mut Criterion) {
    let example = "3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32";
    assert_eq!(part1(example), 3);

    let input = fetch_aoc_input(2025, 5).expect("failed to fetch input");
    let result = part1(&input);
    println!("day5 part1 result: {result}");
    c.bench_function("day5 part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    let example = "3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32";
    assert_eq!(part2(example), 14);

    let input = fetch_aoc_input(2025, 5).expect("failed to fetch input");
    let result = part2(&input);
    println!("day5 part2 result: {result}");
    c.bench_function("day5 part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);

fn merge_ranges(mut ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    ranges.sort_by_key(|r| r.start);

    let mut merged = Vec::new();
    let mut current = ranges[0];

    for r in ranges.into_iter().skip(1) {
        if r.start <= current.last + 1 {
            let new_end = (current.last).max(r.last);
            current = RangeInclusive {
                start: current.start,
                last: new_end,
            };
        } else {
            merged.push(current);
            current = r;
        }
    }

    merged.push(current);
    merged
}
