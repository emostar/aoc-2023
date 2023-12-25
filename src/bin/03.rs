advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // Valid numbers that will be used to create the final sum
    let mut numbers = Vec::new();

    // Create a 2d array of input to make a grid
    let mut grid = Vec::new();
    for (x, line) in input.lines().enumerate() {
        grid.push(Vec::new());
        for c in line.chars() {
            grid[x].push(c);
        }
    }

    // Iterate the grid, and when we find a number, get the start and end position of the number
    // and then check each adjacent position (including diagonals) for a special symbol (other than
    // '.' and any number). If we find one, then we add the number to the list of numbers to add,
    // and then continue iterating the grid after the end position of the number we just found.
    for (x, line) in grid.iter().enumerate() {
        let mut was_previous_char_number = false;

        for (y, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                // Skip if we already processed this in the inner loop when reading ahead to find the
                // complete number.
                if was_previous_char_number {
                    continue;
                }
                was_previous_char_number = true;

                // Find the start and end of the number in this line
                let start = (x, y);
                let mut end = (x, y);
                for (y, c) in line.iter().enumerate().skip(y) {
                    if c.is_ascii_digit() {
                        end = (x, y);
                    } else {
                        break;
                    }
                }

                // Get the 4 coordinates and create a box that make the area we will check
                let start_x = if start.0 == 0 { 0 } else { start.0 - 1 };
                let start_y = if start.1 == 0 { 0 } else { start.1 - 1 };
                let end_x = if end.0 == grid.len() - 1 {
                    grid.len() - 1
                } else {
                    end.0 + 1
                };
                let end_y = if end.1 == grid[start.0].len() - 1 {
                    grid[start.0].len() - 1
                } else {
                    end.1 + 1
                };

                // Check each adjacent position for a special symbol
                let mut done = false;
                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        if grid[x][y] != '.' && !grid[x][y].is_ascii_digit() {
                            // Build the number from start to end
                            let mut c = String::new();
                            for x in start.1..=end.1 {
                                c.push(grid[start.0][x]);
                            }
                            // Convert c to an int
                            let n = c.parse().unwrap();
                            numbers.push(n);
                            // All done
                            done = true;
                            break;
                        }
                    }
                    if done {
                        break;
                    }
                }
            } else {
                // Make sure to reset this flag when we find a non-number
                was_previous_char_number = false;
            }
        }
    }

    numbers.iter().sum::<u32>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    // Valid numbers that will be used to create the final sum
    let mut numbers = Vec::new();

    // Create a 2d array of input to make a grid
    let mut grid = Vec::new();
    for (x, line) in input.lines().enumerate() {
        grid.push(Vec::new());
        for c in line.chars() {
            grid[x].push(c);
        }
    }

    // Iterate the grid, but look for the '*' symbol. When we find this, we will check each
    // adjacent square for a number. If there is a number, we need to find the start and end
    // coordinates. After checking all adjacent numbers, if there are only 2 numbers, then we
    // multiple them and add them to to the numbers array to sum up later.
    //
    // Note: When we are find a number adjacent to a '*', we need to search in both left and right
    // directions to get the complete number. The case below needs to be considered when creating
    // a correct solution.
    //
    // 1023.456
    // ....*...
    // ........
    for (x, line) in grid.iter().enumerate() {
        for (y, c) in line.iter().enumerate() {
            if *c == '*' {
                let mut adjacent_numbers = Vec::new();

                let start_x = if x == 0 { 0 } else { x - 1 };
                let start_y = if y == 0 { 0 } else { y - 1 };
                let end_x = if x == grid.len() - 1 {
                    grid.len() - 1
                } else {
                    x + 1
                };
                let end_y = if y == grid[x].len() - 1 {
                    grid[x].len() - 1
                } else {
                    y + 1
                };

                // Check each adjacent position for a number
                for x in start_x..=end_x {
                    let mut was_previous_char_number = false;

                    for y in start_y..=end_y {
                        if grid[x][y].is_ascii_digit() {
                            if was_previous_char_number {
                                continue;
                            }
                            was_previous_char_number = true;

                            // Go backwards and find the start of the entire number
                            let mut start = y;
                            for y in (0..=y).rev() {
                                if !grid[x][y].is_ascii_digit() {
                                    start = y + 1;
                                    break;
                                } else if y == 0 {
                                    start = 0;
                                    break;
                                }
                            }
                            // Go forwards to find the end of the entire number
                            let mut end = y;
                            for y in y..grid[x].len() {
                                if !grid[x][y].is_ascii_digit() {
                                    end = y - 1;
                                    break;
                                } else if y == grid[x].len() - 1 {
                                    end = grid[x].len() - 1;
                                    break;
                                }
                            }

                            // Convert string to a whole number and add it to the list
                            let mut num = String::new();
                            for y in start..=end {
                                num.push(grid[x][y]);
                            }
                            adjacent_numbers.push(num.parse::<u32>().unwrap());
                        } else {
                            was_previous_char_number = false;
                        }
                    }
                }

                if adjacent_numbers.len() == 2 {
                    numbers.push(adjacent_numbers[0] * adjacent_numbers[1]);
                }
            }
        }
    }

    numbers.iter().sum::<u32>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
