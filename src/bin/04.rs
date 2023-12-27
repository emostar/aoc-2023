use std::collections::HashSet;
use regex::Regex;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers = Vec::new();

    let re = Regex::new(r"Card\s+\d+: ([\d\s]+) \| ([\d\s]+)").unwrap();
    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let winning_numbers: HashSet<u32> = caps.get(1)
                .map_or("", |m| m.as_str())
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            let scratch_numbers: HashSet<u32> = caps.get(2)
                .map_or("", |m| m.as_str())
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            // Count the number of scratch_numbers in winning_numbers
            let mut count = 0;
            for scratch_number in scratch_numbers {
                if winning_numbers.contains(&scratch_number) {
                    count += 1;
                }
            }

            if count > 0 {
                numbers.push(2_u32.pow(count - 1))
            }
        }
    }


    numbers.iter().sum::<u32>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = Vec::new();

    let re = Regex::new(r"Card\s+\d+: ([\d\s]+) \| ([\d\s]+)").unwrap();
    for (n, line) in input.lines().enumerate() {
        if let Some(caps) = re.captures(line) {
            let winning_numbers: HashSet<u32> = caps.get(1)
                .map_or("", |m| m.as_str())
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            let scratch_numbers: HashSet<u32> = caps.get(2)
                .map_or("", |m| m.as_str())
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            // Count the number of scratch_numbers in winning_numbers
            let mut count = 0;
            for scratch_number in scratch_numbers {
                if winning_numbers.contains(&scratch_number) {
                    count += 1;
                }
            }

            // Add empty slots for card winnings, if not present
            if cards.len() < n + count + 1 {
                cards.resize(n + count + 1, 0);
            }

            // Count this card
            cards[n] += 1;

            // Increase the counts for further cards
            for i in 1..=count {
                cards[n + i] += 1 * cards[n];
            }
        }
    }


    cards.iter().sum::<u32>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
