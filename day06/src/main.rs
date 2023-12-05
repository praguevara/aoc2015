use std::ops::RangeInclusive;

#[derive(Debug, Default)]
enum Action {
    #[default]
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut splits = l.split_whitespace();

            let action = match splits.next().unwrap() {
                "toggle" => Action::Toggle,
                "turn" => match splits.next().unwrap() {
                    "on" => Action::On,
                    "off" => Action::Off,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };

            let mut start_it = splits
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap());

            let x_start = start_it.next().unwrap();
            let y_start = start_it.next().unwrap();

            splits.next();

            let mut end_it = splits
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap());
            let x_end = end_it.next().unwrap();
            let y_end = end_it.next().unwrap();

            Instruction {
                action,
                x: x_start..=x_end,
                y: y_start..=y_end,
            }
        })
        .collect()
}

struct Grid {
    lights: Box<[[bool; 1_000]; 1_000]>,
}

impl Grid {
    fn new() -> Self {
        Self {
            lights: Box::new([[false; 1_000]; 1_000]),
        }
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        instruction.x.clone().for_each(|x| {
            instruction
                .y
                .clone()
                .for_each(|y| match instruction.action {
                    Action::On => self.lights[x][y] = true,
                    Action::Off => self.lights[x][y] = false,
                    Action::Toggle => self.lights[x][y] = !self.lights[x][y],
                })
        });
    }

    fn lights_on(&self) -> usize {
        self.lights
            .iter()
            .flat_map(|r| r.iter())
            .filter(|&x| *x)
            .count()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let instructions = parse_input(input);
    let mut grid = Grid::new();
    instructions.iter().for_each(|i| grid.apply_instruction(i));
    println!("{}", grid.lights_on());
}
