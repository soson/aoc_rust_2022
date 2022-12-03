#[derive(Debug)]
enum Guess {
    Rock,
    Paper,
    Scissors,
}

impl Guess {
    pub fn get_result(&self, other: &Self) -> RoundResult {
        match (self, other) {
            (Self::Rock, Self::Rock) => RoundResult::Draw,
            (Self::Rock, Self::Paper) => RoundResult::Loss,
            (Self::Rock, Self::Scissors) => RoundResult::Win,
            (Self::Paper, Self::Rock) => RoundResult::Win,
            (Self::Paper, Self::Paper) => RoundResult::Draw,
            (Self::Paper, Self::Scissors) => RoundResult::Loss,
            (Self::Scissors, Self::Rock) => RoundResult::Loss,
            (Self::Scissors, Self::Paper) => RoundResult::Win,
            (Self::Scissors, Self::Scissors) => RoundResult::Draw,
        }
    }
    fn count_points(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum RoundResult {
    Win,
    Loss,
    Draw,
}

impl RoundResult {
    fn count_points(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

fn get_oponnent_guess(c: char) -> Guess {
    match c {
        'A' => Guess::Rock,
        'B' => Guess::Paper,
        _ => Guess::Scissors,
    }
}

fn get_my_guess(c: char) -> Guess {
    match c {
        'X' => Guess::Rock,
        'Y' => Guess::Paper,
        _ => Guess::Scissors,
    }
}

fn get_round_result(line: &str) -> u32 {
    let guess = line
        .split_whitespace()
        .map(|guess| guess.chars().nth(0).unwrap())
        .collect::<Vec<char>>();

    let opponent = get_oponnent_guess(guess[0]);
    let my_guess = get_my_guess(guess[1]);
    let result = my_guess.get_result(&opponent);
    result.count_points() + my_guess.count_points()
}

fn get_supposed_result(c: char) -> RoundResult {
    match c {
        'X' => RoundResult::Loss,
        'Y' => RoundResult::Draw,
        _ => RoundResult::Win,
    }
}

fn get_needed_guess(g: &Guess, r: &RoundResult) -> Guess {
    match (g, r) {
        (Guess::Paper, RoundResult::Win) => Guess::Scissors,
        (Guess::Paper, RoundResult::Draw) => Guess::Paper,
        (Guess::Paper, RoundResult::Loss) => Guess::Rock,
        (Guess::Rock, RoundResult::Win) => Guess::Paper,
        (Guess::Rock, RoundResult::Draw) => Guess::Rock,
        (Guess::Rock, RoundResult::Loss) => Guess::Scissors,
        (Guess::Scissors, RoundResult::Win) => Guess::Rock,
        (Guess::Scissors, RoundResult::Draw) => Guess::Scissors,
        (Guess::Scissors, RoundResult::Loss) => Guess::Paper,
    }
}

fn get_round_result2(line: &str) -> u32 {
    let guess = line
        .split_whitespace()
        .map(|guess| guess.chars().nth(0).unwrap())
        .collect::<Vec<char>>();

    let opponent = get_oponnent_guess(guess[0]);
    let result = get_supposed_result(guess[1]);
    let my_guess = get_needed_guess(&opponent, &result);
    result.count_points() + my_guess.count_points()
}

pub fn get_result(s: &str) -> u32 {
    s.lines().map(|l| get_round_result(l)).sum()
}

pub fn get_result2(s: &str) -> u32 {
    s.lines().map(|l| get_round_result2(l)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_paper_scissors() {
        let input = "A Y
        B X
        C Z";
        let result = get_result(&input);
        assert_eq!(result, 15);

        let result2 = get_result2(&input);
        assert_eq!(result2, 12);
    }
}
