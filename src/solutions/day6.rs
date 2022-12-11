use std::collections::HashSet;

pub fn get_start_of_packet(s: &str, window: usize) -> usize {
    let v: Vec<char> = s.chars().collect();
    let mut position = 0;

    for w in v.windows(window) {
        let s = w.iter().collect::<HashSet<_>>();
        if s.len() == window {
            return position + window;
        }
        position += 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_of_packet() {
        let input = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let start_of_packet = get_start_of_packet(&input, 4);
        assert_eq!(start_of_packet, 5);
        let input = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let start_of_packet = get_start_of_packet(&input, 4);
        assert_eq!(start_of_packet, 6);
        let input = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let start_of_packet = get_start_of_packet(&input, 4);
        assert_eq!(start_of_packet, 10);
        let input = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let start_of_packet = get_start_of_packet(&input, 4);
        assert_eq!(start_of_packet, 11);
    }

    #[test]
    fn test_start_of_message() {
        let input = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let start_of_packet = get_start_of_packet(&input, 14);
        assert_eq!(start_of_packet, 19);
        let input = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let start_of_packet = get_start_of_packet(&input, 14);
        assert_eq!(start_of_packet, 23);
        let input = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let start_of_packet = get_start_of_packet(&input, 14);
        assert_eq!(start_of_packet, 23);
        let input = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let start_of_packet = get_start_of_packet(&input, 14);
        assert_eq!(start_of_packet, 29);
    }
}
