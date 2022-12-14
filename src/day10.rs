use itertools::Itertools;

fn get_closest_three(n: i32) -> [i32; 3] {
    [n-1, n, n+1]
}

pub fn part1(input: String) {
    let instructions: Vec<(&str, i32)> = input
        .trim()
        .split('\n')
        .map(|x| {
            if x.starts_with("noop") {
                return (x, 0);
            }

            let (a, b): (&str, &str) = x
            .trim()
            .split(' ')
            .collect_tuple().unwrap();

            let c = b.parse::<i32>().unwrap();

            (a, c)
        })
        .collect();

    let mut signal_strengths: Vec<i32> = Vec::new();
    let checks = [20, 60, 100, 140, 180, 220];
    let mut cycle = 0;
    let mut x = 1;

    for (ins, val) in instructions {
        match ins {
            "noop" => {
                cycle += 1;
                if checks.contains(&cycle) {
                    signal_strengths.push(x*cycle);
                }
            },
            "addx" => {
                cycle += 1;
                if checks.contains(&cycle) {
                    signal_strengths.push(x*cycle);
                }
                cycle += 1;
                if checks.contains(&cycle) {
                    signal_strengths.push(x*cycle);
                }
                x += val;
            },
            _ => panic!(),
        }
    }

    let signal_sum: i32 = signal_strengths.into_iter().sum();

    println!("Part 1: {:?}", signal_sum);
}

pub fn part2(input: String) {
    let instructions: Vec<(&str, i32)> = input
        .trim()
        .split('\n')
        .map(|x| {
            if x.starts_with("noop") {
                return (x, 0);
            }

            let (a, b): (&str, &str) = x
            .trim()
            .split(' ')
            .collect_tuple().unwrap();

            let c = b.parse::<i32>().unwrap();

            (a, c)
        })
        .collect();

    let mut cycle = 0;
    let mut x = 1;

    for (ins, val) in instructions {
        match ins {
            "noop" => {
                cycle += 1;
                if get_closest_three(x).contains(&cycle) {
                    print!("#");
                } else {
                    print!(".");
                }

                if cycle % 40 == 0 {
                    print!("\n");
                    cycle = 0;
                }
            },
            "addx" => {
                if get_closest_three(x).contains(&cycle) {
                    print!("#");
                } else {
                    print!(".");
                }
                cycle += 1;

                if cycle % 40 == 0 {
                    print!("\n");
                    cycle = 0;
                }

                if get_closest_three(x).contains(&cycle) {
                    print!("#");
                } else {
                    print!(".");
                }
                cycle += 1;

                if cycle % 40 == 0 {
                    print!("\n");
                    cycle = 0;
                }

                x += val;
            },
            _ => panic!(),
        }

    }
}
