type Assignment = (u32, u32);
type AssignmentPair = (Assignment, Assignment);

fn parse_assignment(s: &str) -> Assignment {
    let assingment = s
        .splitn(2, "-")
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    (assingment[0], assingment[1])
}

fn parse_assignment_pair(s: &str) -> AssignmentPair {
    let result: Vec<Assignment> = s.splitn(2, ",").map(|a| parse_assignment(a)).collect();
    (result[0], result[1])
}

fn get_assignments(input: &str) -> Vec<AssignmentPair> {
    input.lines().map(|l| parse_assignment_pair(l)).collect()
}

fn fully_contained_predicate(pair: &AssignmentPair) -> bool {
    let (a, b) = pair;
    if (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1) {
        true
    } else {
        false
    }
}

fn partially_contained_predicate(pair: &AssignmentPair) -> bool {
    let (a, b) = pair;
    if a.1 >= b.0 && a.0 <= b.1 {
        true
    } else {
        false
    }
}

fn count_filtered_assignments(input: &str, p: impl Fn(&AssignmentPair) -> bool) -> u32 {
    let assignments = get_assignments(input);
    let filtered: Vec<AssignmentPair> = assignments.into_iter().filter(|a| p(a)).collect();
    filtered.len() as u32
}

pub fn count_fully_contained_assignments(input: &str) -> u32 {
    count_filtered_assignments(input, fully_contained_predicate)
}
pub fn count_partially_contained_assignments(input: &str) -> u32 {
    count_filtered_assignments(input, partially_contained_predicate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignments() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let sum = count_fully_contained_assignments(&input);
        assert_eq!(sum, 2);
        let sum = count_partially_contained_assignments(&input);
        assert_eq!(sum, 4);
    }
}
