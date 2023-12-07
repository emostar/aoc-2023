use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    // Regex to parse the input
    let game_re = Regex::new(r"^Game (\d+): (.*)").unwrap();
    let red_re = Regex::new(r"(\d+) red").unwrap();
    let green_re = Regex::new(r"(\d+) green").unwrap();
    let blue_re = Regex::new(r"(\d+) blue").unwrap();

    // Constants for the max number of each color
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    input.lines().map(|line| {
        let (game, game_string) = game_re.captures(line).unwrap().iter().skip(1).map(|x| x.unwrap().as_str()).collect_tuple().unwrap();
        let mut games = game_string.split(";");

        let has_illegal_game = games.any(|game| {
            // each color can be optional, but if present, must be less than the max
            let red = red_re.captures(game).map(|x| x.get(1).unwrap().as_str().parse::<u32>().unwrap()).unwrap_or(0);
            let green = green_re.captures(game).map(|x| x.get(1).unwrap().as_str().parse::<u32>().unwrap()).unwrap_or(0);
            let blue = blue_re.captures(game).map(|x| x.get(1).unwrap().as_str().parse::<u32>().unwrap()).unwrap_or(0);

            red > max_red || green > max_green || blue > max_blue
        });

        if has_illegal_game {
            0
        } else {
            game.parse::<u32>().unwrap()
        }
    }).sum::<u32>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    // Regex to parse the input
    let game_re = Regex::new(r"^Game \d+: (.*)").unwrap();
    let red_re = Regex::new(r"(\d+) red").unwrap();
    let green_re = Regex::new(r"(\d+) green").unwrap();
    let blue_re = Regex::new(r"(\d+) blue").unwrap();

    input.lines().map(|line| {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let game_string = game_re.captures(line).unwrap().get(1).unwrap().as_str();
        for game in game_string.split(";") {
            let red = red_re.captures(game).map(|x| x.get(1).unwrap().as_str().parse::<u32>().unwrap()).unwrap_or(0);
            let green = green_re.captures(game).map(|x| x.get(1).unwrap().as_str().parse::<u32>().unwrap()).unwrap_or(0);
            let blue = blue_re.captures(game).map(|x| x.get(1).unwrap().as_str().parse::<u32>().unwrap()).unwrap_or(0);

            if red > max_red {
                max_red = red;
            }

            if green > max_green {
                max_green = green;
            }

            if blue > max_blue {
                max_blue = blue;
            }
        }

        max_red * max_green * max_blue
    }).sum::<u32>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
