use std::collections::{HashMap, HashSet};

fn get_shared_item(s1: &str, s2: &str) -> char {
    let set1: HashSet<char> = s1.chars().into_iter().collect();
    let set2: HashSet<char> = s2.chars().into_iter().collect();
    let shared: Vec<char> = set1.intersection(&set2).cloned().collect();
    shared[0]
}

fn count_priority(c: &char) -> u32 {
    if c.is_lowercase() {
        *c as u32 - 'a' as u32 + 1
    } else {
        *c as u32 - 'A' as u32 + 27
    }
}

fn get_priority(line: &str) -> u32 {
    let (first, second) = line.split_at(line.len() / 2);
    let shared_item = get_shared_item(first, second);
    let priority = count_priority(&shared_item);
    priority
}

pub fn sum_priorities(input: &str) -> u32 {
    input.lines().map(|l| get_priority(&l)).sum()
}

fn get_badge(lines: &[&str]) -> char {
    let mut lines_chars: Vec<_> = lines
        .iter()
        .map(|&l| l.chars().collect::<Vec<char>>())
        .collect();

    for l in lines_chars.iter_mut() {
        l.sort();
        l.dedup();
    }

    let all_chars = lines_chars
        .iter()
        .map(|ch| ch.iter().cloned().collect::<String>())
        .collect::<Vec<_>>();

    let mut result: HashMap<char, u32> = HashMap::new();
    for ch in all_chars.join("").chars() {
        result.entry(ch).and_modify(|e| *e += 1).or_insert(1);
    }

    let badge = result
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();

    badge.to_owned()
}

pub fn sum_badges(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let chunks: Vec<_> = lines.chunks(3).collect();
    let badge_priority_sum: u32 = chunks
        .iter()
        .map(|&ch| get_badge(ch))
        .map(|c| count_priority(&c))
        .sum();
    badge_priority_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let sum = sum_priorities(&input);
        assert_eq!(sum, 157);

        let sum = sum_badges(&input);
        assert_eq!(sum, 70);
    }
}
