advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            let last = line
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            first * 10 + last
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let mut result = String::new();
            for c in line.chars() {
                result.push(c);
                result = result
                    .replace("one", "1e")
                    .replace("two", "2o")
                    .replace("three", "3e")
                    .replace("four", "4r")
                    .replace("five", "5e")
                    .replace("six", "6x")
                    .replace("seven", "7n")
                    .replace("eight", "8t")
                    .replace("nine", "9e");
            }

            let first = result
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            let last = result
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            first * 10 + last
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
