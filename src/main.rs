#![feature(entry_and_modify)]
use std::collections::{HashMap, HashSet};
use std::time;

#[derive(Clone, Copy)]
enum Wind {
    North,
    East,
    South,
    West,
}

enum Turn {
    Left,
    Right,
    Straight,
    Back,
}

#[derive(Clone, Copy, PartialEq)]
enum Infection {
    Clean,
    Weakened,
    Flagged,
    Infected,
}

type Coords = (i32, i32);
type Grid1 = HashSet<Coords>;
type Grid2 = HashMap<Coords, Infection>;
type Grid3 = [[Infection; 2_000]; 2_000];
type Grid4 = Vec<[usize; 2_000]>;

use Wind::*;
use Turn::*;
use Infection::*;

const NORTH: usize = 0;
const WINDS: [Wind; 4] = [North, West, South, East];

const STRAIGHT: usize = 0;
const LEFT: usize = 1;
const BACK: usize = 2;
const RIGHT: usize = 3;
const TURNS: [usize; 4] = [LEFT, STRAIGHT, RIGHT, BACK];

const CLEAN: usize = 0;
const INFECTED: usize = 2;

fn turn(wind: &Wind, turn: &Turn) -> Wind {
    match (wind, turn) {
        (&East, &Left) | (&West, &Right) | (&South, &Back) => North,
        (&North, &Right) | (&South, &Left) | (&West, &Back) => East,
        (&North, &Left) | (&South, &Right) | (&East, &Back) => West,
        (&East, &Right) | (&West, &Left) | (&North, &Back) => South,
        (&wind, &Straight) => wind,
    }
}

fn orient(infection: &Infection) -> Turn {
    match *infection {
        Clean => Left,
        Weakened => Straight,
        Infected => Right,
        Flagged => Back,
    }
}

fn touch(infection: &Infection) -> Infection {
    match *infection {
        Clean => Weakened,
        Weakened => Infected,
        Infected => Flagged,
        Flagged => Clean,
    }
}

fn go(&(x, y): &Coords, wind: &Wind) -> Coords {
    match *wind {
        North => (x, y - 1),
        East => (x + 1, y),
        South => (x, y + 1),
        West => (x - 1, y),
    }
}

fn part1(mut grid: Grid1, mut pos: Coords, n: usize) -> usize {
    let mut wind = North;
    let mut count = 0;
    for _ in 0..n {
        let infected = grid.remove(&pos);
        if !infected {
            grid.insert(pos);
            count += 1;
        }
        wind = turn(&wind, &if infected { Right } else { Left });
        pos = go(&pos, &wind);
    }
    count
}

fn part2(mut grid: Grid2, mut pos: Coords, n: usize) -> usize {
    let mut wind = North;
    let mut count = 0;
    for _ in 0..n {
        grid.entry(pos)
            .and_modify(|infected| {
                wind = turn(&wind, &orient(infected));
                *infected = touch(infected);
                if *infected == Infected {
                    count += 1;
                }
            })
            .or_insert_with(|| {
                wind = turn(&wind, &orient(&Clean));
                Weakened
            });
        pos = go(&pos, &wind);
    }
    count
}

fn part3(mut grid: Grid3, mut pos: Coords, n: usize) -> usize {
    let mut wind = North;
    let mut count = 0;
    for _ in 0..n {
        let (x, y) = (pos.0 as usize, pos.1 as usize);
        let infected = grid[x][y];
        let touched = touch(&infected);
        grid[x][y] = touched;
        if touched == Infected {
            count += 1;
        }
        wind = turn(&wind, &orient(&infected));
        pos = go(&pos, &wind);
    }
    count
}

fn part4(mut grid: Grid4, mut pos: Coords, n: usize) -> usize {
    let mut wind = NORTH;
    let mut count = 0;
    for _ in 0..n {
        let (x, y) = (pos.0 as usize, pos.1 as usize);
        let infected = grid[x][y];
        let touched = (infected + 1) % 4;
        grid[x][y] = touched;
        if touched == INFECTED {
            count += 1;
        }
        wind = (wind + TURNS[infected]) % 4;
        pos = go(&pos, &WINDS[wind]);
    }
    count
}

fn timed(label: &str, f: &Fn()) {
    println!("Starting {}...", label);
    let start = time::Instant::now();
    f();
    let elapsed = start.elapsed();
    println!("{} : {:?}", label, elapsed);
}

fn main() {
    timed("Part 1", &|| {
        println!("Part 1 - test 7: {}", part1(test_input(&read1), (1, 1), 7));
        println!(
            "Part 1 - test 70: {}",
            part1(test_input(&read1), (1, 1), 70)
        );
        println!(
            "Part 1 - test 10k: {}",
            part1(test_input(&read1), (1, 1), 10_000)
        );
        println!(
            "Part 1 - challenge: {}",
            part1(challenge_input(&read1), (12, 12), 10_000)
        );
    });

    timed("Part 2", &|| {
        println!("Part 2 - test 8: {}", part2(test_input(&read2), (1, 1), 8));
        println!(
            "Part 2 - test 100: {}",
            part2(test_input(&read2), (1, 1), 100)
        );
        println!(
            "Part 2 - test 10M: {}",
            part2(test_input(&read2), (1, 1), 10_000_000)
        );
        println!(
            "Part 2 - challenge: {}",
            part2(challenge_input(&read2), (12, 12), 10_000_000)
        );
    });

    timed("Part 3", &|| {
        println!(
            "Part 3 - test 8: {}",
            part3(test_input(&read3), (1_001, 1_001), 8)
        );
        println!(
            "Part 3 - test 100: {}",
            part3(test_input(&read3), (1_001, 1_001), 100)
        );
        println!(
            "Part 3 - test 10M: {}",
            part3(test_input(&read3), (1_001, 1_001), 10_000_000)
        );
        println!(
            "Part 3 - challenge: {}",
            part3(challenge_input(&read3), (1_012, 1_012), 10_000_000)
        );
    });

    timed("Part 4", &|| {
        println!(
            "Part 4 - test 8: {}",
            part4(test_input(&read4), (1_001, 1_001), 8)
        );
        println!(
            "Part 4 - test 100: {}",
            part4(test_input(&read4), (1_001, 1_001), 100)
        );
        println!(
            "Part 4 - test 10M: {}",
            part4(test_input(&read4), (1_001, 1_001), 10_000_000)
        );
        println!(
            "Part 4 - challenge: {}",
            part4(challenge_input(&read4), (1_012, 1_012), 10_000_000)
        );
    });
}

fn read1(input: &[&str]) -> Grid1 {
    let mut grid = Grid1::new();
    for (y, s) in input.iter().enumerate() {
        for (x, _) in s.chars().enumerate().filter(|p| p.1 == '#') {
            grid.insert((x as i32, y as i32));
        }
    }
    grid
}

fn read2(input: &[&str]) -> Grid2 {
    let mut grid = Grid2::new();
    for (y, s) in input.iter().enumerate() {
        for (x, _) in s.chars().enumerate().filter(|p| p.1 == '#') {
            grid.insert((x as i32, y as i32), Infected);
        }
    }
    grid
}

fn read3(input: &[&str]) -> Grid3 {
    let mut grid = [[Clean; 2_000]; 2_000];
    for (y, s) in input.iter().enumerate() {
        for (x, _) in s.chars().enumerate().filter(|p| p.1 == '#') {
            grid[x + 1_000][y + 1_000] = Infected;
        }
    }
    grid
}

fn read4(input: &[&str]) -> Grid4 {
    let mut grid = vec![[CLEAN; 2_000]; 2_000];
    for (y, s) in input.iter().enumerate() {
        for (x, _) in s.chars().enumerate().filter(|p| p.1 == '#') {
            grid[x + 1_000][y + 1_000] = INFECTED;
        }
    }
    grid
}

fn test_input<Grid>(read: &Fn(&[&str]) -> Grid) -> Grid {
    read(&["..#", "#..", "..."])
}

fn challenge_input<Grid>(read: &Fn(&[&str]) -> Grid) -> Grid {
    read(&[
        ".###.#.#####.##.#...#....",
        "..####.##.##.#..#.....#..",
        ".#####.........#####..###",
        "#.#..##..#.###.###.#.####",
        ".##.##..#.###.###...#...#",
        "#.####..#.#.##.##...##.##",
        "..#......#...#...#.#....#",
        "###.#.#.##.#.##.######..#",
        "###..##....#...##....#...",
        "###......#..#..###.#...#.",
        "#.##..####.##..####...##.",
        "###.#.#....######.#.###..",
        ".#.##.##...##.#.#..#...##",
        "######....##..##.######..",
        "##..##.#.####.##.###.#.##",
        "#.###.#.##....#.##..####.",
        "#.#......##..####.###.#..",
        "#..###.###...#..#.#.##...",
        "#######..#.....#######..#",
        "##..##..#..#.####..###.#.",
        "..#......##...#..##.###.#",
        "....##..#.#.##....#..#.#.",
        "..#...#.##....###...###.#",
        "#.#.#.#..##..##..#..#.##.",
        "#.####.#......#####.####.",
    ])
}
