use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wire(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Source {
    Signal(u16),
    Wire(Wire),
    And(Box<Source>, Box<Source>),
    Or(Box<Source>, Box<Source>),
    LShift(Box<Source>, i32),
    RShift(Box<Source>, i32),
    Not(Box<Source>),
}

impl TryFrom<&str> for Source {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut before_arrow_tokens = value.split_whitespace().take(3);

        let (t1, t2, t3) = (
            before_arrow_tokens.next(),
            before_arrow_tokens.next(),
            before_arrow_tokens.next(),
        );

        let source: Source = match (t1, t2, t3) {
            (Some(x), None, None) => {
                if let Ok(signal) = x.parse::<u16>() {
                    Source::Signal(signal)
                } else {
                    Source::Wire(Wire(x.to_string()))
                }
            }
            (Some("NOT"), Some(src), None) => Source::Not(Box::new(src.try_into().unwrap())),
            (Some(a), Some(op), Some(b)) => match op {
                "AND" => Source::And(
                    Box::new(a.try_into().unwrap()),
                    Box::new(b.try_into().unwrap()),
                ),
                "OR" => Source::Or(
                    Box::new(a.try_into().unwrap()),
                    Box::new(b.try_into().unwrap()),
                ),
                "LSHIFT" => Source::LShift(
                    Box::new(a.try_into().unwrap()),
                    b.parse::<i32>().map_err(|_| ())?,
                ),
                "RSHIFT" => Source::RShift(
                    Box::new(a.try_into().unwrap()),
                    b.parse::<i32>().map_err(|_| ())?,
                ),
                _ => Err(())?,
            },
            _ => Err(())?,
        };

        Ok(source)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    source: Source,
    wire: Wire,
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut arrow_split = value.split("->");

        let source = Source::try_from(arrow_split.next().unwrap())?;
        let wire = arrow_split
            .next()
            .map(str::trim)
            .map(String::from)
            .map(Wire)
            .ok_or(())?;
        Ok(Instruction { source, wire })
    }
}

fn wire_to_signal(
    source: Source,
    instructions: &mut HashMap<Wire, Source>,
    cache: &mut HashMap<Wire, u16>,
) -> u16 {
    match source {
        Source::Signal(signal) => signal,
        Source::Wire(wire) => {
            if let Some(signal) = cache.get(&wire) {
                *signal
            } else {
                let signal = wire_to_signal(instructions[&wire].clone(), instructions, cache);
                cache.insert(wire, signal);
                signal
            }
        }
        Source::And(a, b) => {
            wire_to_signal(*a, instructions, cache) & wire_to_signal(*b, instructions, cache)
        }
        Source::Or(a, b) => {
            wire_to_signal(*a, instructions, cache) | wire_to_signal(*b, instructions, cache)
        }
        Source::LShift(a, b) => wire_to_signal(*a, instructions, cache) << b,
        Source::RShift(a, b) => wire_to_signal(*a, instructions, cache) >> b,
        Source::Not(a) => !wire_to_signal(*a, instructions, cache),
    }
}

fn run_instructions_for(wire: Wire, instructions: impl IntoIterator<Item = Instruction>) -> u16 {
    let mut instructions = instructions
        .into_iter()
        .map(|i| (i.wire, i.source))
        .collect::<HashMap<Wire, Source>>();

    wire_to_signal(Source::Wire(wire), &mut instructions, &mut HashMap::new())
}

fn main() {
    let input = include_str!("../input.txt");
    let instructions = input.lines().map(Instruction::try_from).map(Result::unwrap);
    let result = run_instructions_for(Wire(String::from("a")), instructions);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_instructions() {
        let input = include_str!("../sample.txt");
        let instructions = input
            .lines()
            .flat_map(Instruction::try_from)
            .collect::<Vec<_>>();

        assert_eq!(
            run_instructions_for(Wire(String::from("d")), instructions.clone()),
            72
        );
        assert_eq!(
            run_instructions_for(Wire(String::from("e")), instructions.clone()),
            507
        );
        assert_eq!(
            run_instructions_for(Wire(String::from("f")), instructions.clone()),
            492
        );
        assert_eq!(
            run_instructions_for(Wire(String::from("g")), instructions.clone()),
            114
        );
        assert_eq!(
            run_instructions_for(Wire(String::from("h")), instructions.clone()),
            65412
        );
        assert_eq!(
            run_instructions_for(Wire(String::from("i")), instructions.clone()),
            65079
        );
        assert_eq!(
            run_instructions_for(Wire(String::from("x")), instructions.clone()),
            123
        );
        assert_eq!(
            run_instructions_for(Wire(String::from("y")), instructions.clone()),
            456
        );
    }
}
