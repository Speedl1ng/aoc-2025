use aoc_2025::fetch_aoc_input;
use criterion::{Criterion, criterion_group, criterion_main};

fn part1(input: &str) -> u64 {
    let input = input.trim();
    let lines = input.lines();
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for line in lines {
        let line = line.trim();
        let mut vec_line = Vec::with_capacity(line.len());
        for char in line.chars() {
            vec_line.push(char == '@');
        }
        grid.push(vec_line);
    }

    let mut papers = 0;
    let grid_iter = grid.iter().enumerate();
    for (x, row) in grid_iter {
        let row_iter = row.iter().enumerate();
        for (y, symbol) in row_iter {
            if *symbol {
                let subgrid = get_subgrid(&grid, x, y);
                let sum = subgrid.iter().map(|f| *f as u32).sum::<u32>();
                // println!("{sum}");
                if sum < 5 {
                    papers += 1;
                }
                // else {
                // print!("@")
                // }
            }
            // else {
            // print!(".")
            // }
        }
        // println!();
    }

    papers
}

fn part2(input: &str) -> u64 {
    let input = input.trim();
    let lines = input.lines();
    let mut grid: Vec<bool> = Vec::with_capacity(input.len());
    let height = lines.clone().next().unwrap().chars().count();
    let width = lines.clone().count();
    for line in lines {
        let line = line.trim();
        for ch in line.chars() {
            grid.push(ch == '@');
        }
    }

    let mut res = 0;

    loop {
        let mut removes = 0;

        for idx in 0..grid.len() {
            if grid[idx] {
                let y = idx / width;
                let x = idx % width;

                let mut sum = 0;

                (0..9).for_each(|k| {
                    let i = (k / 3) as isize - 1;
                    let j = (k % 3) as isize - 1;

                    let xi = x as isize + j;
                    let yi = y as isize + i;

                    if xi >= 0 && xi < width as isize && yi >= 0 && yi < height as isize {
                        sum += grid[(yi as usize) * width + (xi as usize)] as i32;
                    }
                });

                if sum < 5 {
                    removes += 1;
                    grid[idx] = false;
                }
            }
        }

        res += removes;

        if removes == 0 {
            break;
        }
    }

    res
}

fn bench_part1(c: &mut Criterion) {
    let example = "
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.";
    assert_eq!(part1(example), 13);

    let input = fetch_aoc_input(2025, 4).expect("failed to fetch input");
    let result = part1(&input);
    println!("day4 part1 result: {result}");
    c.bench_function("day4 part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    let example = "
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.";
    assert_eq!(part2(example), 43);

    let input = fetch_aoc_input(2025, 4).expect("failed to fetch input");
    let result = part2(&input);
    println!("day4 part2 result: {result}");
    c.bench_function("day4 part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);

#[inline(always)]
fn get_subgrid(grid: &[Vec<bool>], x: usize, y: usize) -> [bool; 9] {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut sub = [false; 9];

    (0..9).for_each(|k| {
        let i = (k / 3) as isize - 1; // -1, 0, 1
        let j = (k % 3) as isize - 1;

        let xi = x as isize + i;
        let yj = y as isize + j;

        if xi >= 0 && xi < rows && yj >= 0 && yj < cols {
            sub[k] = grid[xi as usize][yj as usize];
        }
    });

    sub
}
