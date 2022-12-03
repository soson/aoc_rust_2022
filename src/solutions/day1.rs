pub fn sum_calories(s: &str) -> u32 {
    s.lines().map(|l| l.parse::<u32>().unwrap()).sum()
}

pub fn get_calories_summary(s: &str) -> Vec<u32> {
    s.split("\n\n").map(|c| sum_calories(&c)).collect()
}

pub fn get_max_calories(s: &[u32]) -> u32 {
    match s.into_iter().max() {
        Some(n) => *n,
        None => 0,
    }
}

pub fn count_max_three_sum(s: &[u32]) -> u32 {
    let mut v = s.to_vec();
    v.sort();
    v[v.len() - 3..].into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_calories() {
        let input = String::from("1\n5\n3\n\n2\n3\n7\n\n4\n\n5");
        let summary = get_calories_summary(&input);
        assert_eq!(summary, vec![9, 12, 4, 5]);

        let max_calories = get_max_calories(&summary);
        assert_eq!(max_calories, 12);

        let max_three_sum = count_max_three_sum(&summary);
        assert_eq!(max_three_sum, 26);
    }
}
