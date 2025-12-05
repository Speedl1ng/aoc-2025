use aoc_2025::fetch_aoc_input;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn part1(input: &str) -> i16 {
    let lines = input.trim().lines();
    let mut counter = 0;
    let mut dial: i16 = 50;
    for line in lines {
        let line = line.trim();
        let bytes = line.as_bytes();
        let direction = bytes[0];
        let number: i16 = unsafe {
            str::from_utf8_unchecked(&bytes[1..])
                .parse()
                .expect("Failed to parse number")
        };
        let rest = number % 100;
        if direction == b'L' {
            dial -= rest;
            if dial < 0 {
                dial += 100;
            }
        } else {
            dial += rest;
            if dial >= 100 {
                dial -= 100;
            }
        }
        if dial == 0 {
            counter += 1;
        }
    }
    counter
}

fn part2(input: &str) -> i16 {
    let lines = input.lines();
    let mut counter = 0;
    let mut dial: i16 = 50;
    for line in lines {
        let bytes = line.as_bytes();
        let direction = bytes[0];
        let number: i16 = unsafe {
            str::from_utf8_unchecked(&bytes[1..])
                .parse()
                .expect("Failed to parse number")
        };
        let clicks = number / 100;
        let rest = number % 100;
        if direction == b'L' {
            dial -= rest;
            if dial < 0 {
                dial += 100;
                counter += 1;
            }
        } else {
            dial += rest;
            if dial >= 100 {
                dial -= 100;
                counter += 1;
            }
        }
        counter += clicks;
    }
    counter
}

fn bench_part1(mut c: &mut Criterion) {
    let example = "
    L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82";
    assert_eq!(part1(example), 3);
    let input = fetch_aoc_input(2025, 1).expect("failed to fetch input");
    let result = part1(&input);
    println!("day1 part1 result: {result}");

    c.bench_with_input(
        BenchmarkId::new("day1 part1", "fetched_input"),
        &input,
        |b, s| b.iter(|| part1(s)),
    );
}

fn bench_part2(c: &mut Criterion) {
    let example = "
    L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82";
    assert_eq!(part2(example), 6);
    let input = fetch_aoc_input(2025, 1).expect("failed to fetch input");
    let result = part2(&input);
    println!("day1 part2 result: {result}");

    c.bench_with_input(
        BenchmarkId::new("day1 part2", "fetched_input"),
        &input,
        |b, s| b.iter(|| part2(s)),
    );
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
