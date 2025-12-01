use aoc_2025::fetch_aoc_input;
use criterion::{Criterion, criterion_group, criterion_main};

const START: i16 = 50;

fn part1(input: &str) -> u16 {
    let mut current = START;
    let mut result = 0;

    for l in input.split('\n') {
        let (direction, distance_str) = l.split_at(1);
        let distance = distance_str.parse::<i16>().unwrap();

        match direction {
            "L" => {
                current -= distance;
            }
            "R" => {
                current += distance;
            }
            _ => panic!(),
        }

        current %= 100;

        if current == 0 {
            result += 1;
        }
    }

    result
}

fn part2(input: &str) -> u16 {
    let mut current = START;
    let mut result = 0;

    for l in input.split('\n') {
        let (direction, distance_str) = l.split_at(1);
        let distance = distance_str.parse::<i16>().unwrap();

        match direction {
            "L" => {
                if current.is_positive() && (current - distance).is_negative() {
                    result += 1;
                }
                current -= distance;
            }
            "R" => {
                if current.is_negative() && (current + distance).is_positive() {
                    result += 1;
                }
                current += distance;
            }
            _ => panic!(),
        }

        let stepped_over = current.unsigned_abs() / 100;
        current %= 100;

        if current == 0 {
            result += 1;
            result += stepped_over.saturating_sub(1);
        } else {
            result += stepped_over;
        }
    }
    result
}

fn bench_part1(c: &mut Criterion) {
    let input = fetch_aoc_input(1);

    let result = part1(&input);
    println!("day1 part1 result: {result}");

    c.bench_function("day1 part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    let input = fetch_aoc_input(1);

    let result = part2(&input);
    println!("day1 part2 result: {result}");

    c.bench_function("day1 part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
