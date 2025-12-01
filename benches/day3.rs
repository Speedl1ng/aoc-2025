use aoc_2025::fetch_aoc_input;
use criterion::{Criterion, criterion_group, criterion_main};

#[derive(Copy, Clone)]
struct Battery {
    position: usize,
    joltage: u32,
}

fn find_best_batteries(
    bank: &[Battery],
    remaining_batteries: u8,
    best_batteries: &mut Vec<Battery>,
) {
    let mut highest_battery: Battery = Battery {
        position: 0,
        joltage: 0,
    };

    for battery in bank {
        if let Some(last_position) = best_batteries.last().map(|b| b.position)
            && battery.position <= last_position
        {
            continue;
        }
        if highest_battery.joltage == 9 {
            break;
        }
        if battery.position + remaining_batteries as usize > bank.len() {
            break;
        }
        if battery.joltage > highest_battery.joltage {
            highest_battery = *battery;
        }
    }

    if remaining_batteries == 0 {
        return;
    }

    best_batteries.push(highest_battery);
    find_best_batteries(bank, remaining_batteries - 1, best_batteries);
}

fn solve(banks: &str, batteries_active: u8) -> u64 {
    let mut result: u64 = 0;

    for bank in banks.split('\n') {
        let batteries: Vec<Battery> = bank
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
            .map(|(position, joltage)| Battery { position, joltage })
            .collect();

        let mut best_batteries_in_bank = Vec::with_capacity(batteries_active.into());
        find_best_batteries(&batteries, batteries_active, &mut best_batteries_in_bank);

        for (index, battery) in best_batteries_in_bank.iter().enumerate() {
            result += u64::from(battery.joltage)
                * 10_u64.pow(u32::from(batteries_active) - u32::try_from(index).unwrap() - 1);
        }
    }

    result
}

fn part1(input: &str) -> u64 {
    solve(input, 2)
}

fn part2(input: &str) -> u64 {
    solve(input, 12)
}

fn bench_part1(c: &mut Criterion) {
    let input = fetch_aoc_input(3);

    let result = part1(&input);
    println!("day3 part1 result: {result}");

    c.bench_function("day3 part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    let input = fetch_aoc_input(3);

    let result = part2(&input);
    println!("day3 part2 result: {result}");

    c.bench_function("day3 part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
