use regex::Regex;
use std::collections::BTreeMap;
use std::vec;

#[derive(Debug)]
pub struct Step {
    amount: usize,
    from: usize,
    to: usize,
}

impl Step {
    fn new(amount: usize, from: usize, to: usize) -> Self {
        Self { amount, from, to }
    }
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.from == other.from && self.to == other.to
    }
}

type Stack = Vec<String>;

fn parse_stacks(input: &str) -> Result<Vec<Stack>, Box<dyn std::error::Error>> {
    let mut stacks_map = BTreeMap::new();
    let re_stacks = Regex::new(r"\[\w\]")?;
    for line in input.lines() {
        if re_stacks.is_match(line) {
            for cr in re_stacks.find_iter(line) {
                let key = cr.start();
                let value = cr.as_str()[1..2].to_string();
                stacks_map.entry(key).or_insert_with(|| vec![]).push(value);
            }
        }
    }

    let mut stacks_vec: Vec<Vec<String>> = vec![];
    for mut s in stacks_map.into_values() {
        s.reverse();
        stacks_vec.push(s);
    }
    Ok(stacks_vec)
}

fn parse_moves(input: &str) -> Result<Vec<Step>, Box<dyn std::error::Error>> {
    let re_moves = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
    let mut output = vec![];
    for l in input.lines() {
        if let Some(captures) = re_moves.captures(l) {
            let moves = (1..=3)
                .map(|i| captures.get(i).unwrap().as_str().parse::<usize>())
                .collect::<Result<Vec<_>, _>>()?;
            output.push(Step::new(moves[0], moves[1], moves[2]));
        };
    }
    Ok(output)
}

fn move_crates_9000(stacks: &mut Vec<Stack>, step: Step) {
    let Step { amount, from, to } = step;
    let idx = stacks[from - 1].len() - amount;
    let mut moved_crates = stacks[from - 1].split_off(idx);
    moved_crates.reverse();
    stacks[to - 1].append(&mut moved_crates);
}

fn move_crates_9001(stacks: &mut Vec<Stack>, step: Step) {
    let Step { amount, from, to } = step;
    let idx = stacks[from - 1].len() - amount;
    let mut moved_crates = stacks[from - 1].split_off(idx);
    stacks[to - 1].append(&mut moved_crates);
}

pub fn get_top_boxes(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut stacks = parse_stacks(input)?;
    let moves = parse_moves(input)?;

    for m in moves {
        move_crates_9000(&mut stacks, m);
    }

    let boxes = stacks.iter().fold(String::new(), |mut acc, s| {
        let mut top_crate = s.last().unwrap();
        acc.push_str(&mut top_crate);
        acc
    });

    Ok(boxes.to_string())
}

pub fn get_top_boxes2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut stacks = parse_stacks(input)?;
    let moves = parse_moves(input)?;

    for m in moves {
        move_crates_9001(&mut stacks, m);
    }

    let boxes = stacks.iter().fold(String::new(), |mut acc, s| {
        let mut top_crate = s.last().unwrap();
        acc.push_str(&mut top_crate);
        acc
    });

    Ok(boxes.to_string())
}

#[cfg(test)]
mod tests {
    use crate::solutions::day5::get_top_boxes;
    use crate::solutions::day5::get_top_boxes2;

    use super::parse_moves;
    use super::parse_stacks;
    use super::Step;

    #[test]
    fn test_stacks() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let output = vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]];

        let stacks = parse_stacks(input);
        assert_eq!(stacks.is_ok(), true);
        let stacks = stacks.unwrap();
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks, output);
    }

    #[test]
    fn test_moves() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let output = vec![
            Step::new(1, 2, 1),
            Step::new(3, 1, 3),
            Step::new(2, 2, 1),
            Step::new(1, 1, 2),
        ];
        let moves = parse_moves(input);
        assert!(moves.is_ok());
        assert_eq!(moves.unwrap(), output);
    }

    #[test]
    fn test_top_boxes() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let moves = get_top_boxes(&input);
        assert!(moves.is_ok());
        assert_eq!(moves.unwrap(), "CMZ");
    }

    #[test]
    fn test_top_boxes2() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let moves = get_top_boxes2(&input);
        assert!(moves.is_ok());
        assert_eq!(moves.unwrap(), "MCD");
    }
}
