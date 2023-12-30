use std::str::FromStr;
use regex::Regex;
advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"\b(\d+)\b").unwrap();
    let lines: Vec<_> = input.lines().collect();

    let times: Vec<_> = re.captures_iter(lines[0])
        .map(|cap| u64::from_str(&cap[1]).unwrap())
        .collect();
    let distances: Vec<_> = re.captures_iter(lines[1])
        .map(|cap| u64::from_str(&cap[1]).unwrap())
        .collect();

    let races: Vec<_> = times.into_iter().zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect();

    let mut counts = Vec::new();

    for race in races {
        let vec: Vec<_> = (0..=race.time).collect();
        let c = vec.iter().filter(|i| *i * (race.time - *i) > race.distance).count();
        counts.push(c as u64);
    }

    Some(counts.into_iter().product::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    part_one(input.replace(' ', "").as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
