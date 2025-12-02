use aoc_2025::fetch_aoc_input;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

// #[derive(Debug)]
// pub struct Dial(pub i16);

// impl Dial {
//     fn turn_right(&mut self, right_turns: i16) {
//         let dial = self.0;
//         let rest: i16 = right_turns % 100;
//         let mut new = dial + rest;
//         if new >= 100 {
//             new -= 100;
//         }
//         self.0 = new;
//     }

//     fn turn_left(&mut self, left_turns: i16) {
//         let dial = self.0;
//         let rest: i16 = left_turns % 100;
//         let mut new = dial - rest;
//         if new < 0 {
//             new += 100;
//         }
//         self.0 = new;
//     }
// }

fn part1(input: &str) -> i16 {
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
    let example = include_str!("../input/example.txt");
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
    let example = include_str!("../input/example.txt");
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

criterion_group!(benches, bench_part2);
criterion_main!(benches);
