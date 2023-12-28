use std::thread;
use regex::Regex;
advent_of_code::solution!(5);

#[derive(Debug, Clone)]
struct PipelineStage {
    pub entries: Vec<PipelineEntry>,
}

impl PipelineStage {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, src_start: u64, dst_start: u64, len: u64) {
        self.entries.push(PipelineEntry::from(src_start, dst_start, len));
    }

    pub fn get_dest(&self, src: u64) -> u64 {
        let mut dest = src;
        for entry in &self.entries {
            if dest >= entry.src_start && dest < entry.src_end {
                let diff = entry.dst_start as i128 - entry.src_start as i128;
                dest = (diff + (dest as i128)) as u64;
                break
            }
        }
        dest
    }
}

#[derive(Debug, Clone)]
struct PipelineEntry {
    pub src_start: u64,
    pub src_end: u64,
    pub dst_start: u64,
}

impl PipelineEntry {
    pub fn from(src_start: u64, dst_start: u64, len: u64) -> Self {
        Self {
            src_start,
            src_end: src_start + len,
            dst_start,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lowest_val = 0xFFFFFFFF;

    let re = Regex::new(r"seeds: ([\d\s]+)").unwrap();
    let seeds: Vec<_> = re
        .captures(input.lines().next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let mut pipeline = Vec::new();

    let entry_re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    let map_parts: Vec<_> = input.split("\n\n").skip(1).collect();
    for map in &map_parts {
        let mut pipeline_stage = PipelineStage::new();

        // Iterate the maps for the seed and find the final location
        for part in map.split("\n").skip(1).collect::<Vec<_>>() {
            let entry = entry_re.captures(part).unwrap();
            pipeline_stage.add_entry(
                entry.get(2).unwrap().as_str().parse().unwrap(),
                entry.get(1).unwrap().as_str().parse().unwrap(),
                entry.get(3).unwrap().as_str().parse().unwrap(),
            );
        }

        pipeline.push(pipeline_stage);
    }

    for mut seed in seeds {
        for stage in &pipeline {
            seed = stage.get_dest(seed);
        }

        if seed < lowest_val {
            lowest_val = seed;
        }
    }

    Some(lowest_val)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"seeds: ([\d\s]+)").unwrap();
    let seed_ranges: Vec<_> = re
        .captures(input.lines().next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let mut pipeline = Vec::new();

    let entry_re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    let map_parts: Vec<_> = input.split("\n\n").skip(1).collect();
    for map in &map_parts {
        let mut pipeline_stage = PipelineStage::new();

        // Iterate the maps for the seed and find the final location
        for part in map.split("\n").skip(1).collect::<Vec<_>>() {
            let entry = entry_re.captures(part).unwrap();
            pipeline_stage.add_entry(
                entry.get(2).unwrap().as_str().parse().unwrap(),
                entry.get(1).unwrap().as_str().parse().unwrap(),
                entry.get(3).unwrap().as_str().parse().unwrap(),
            );
        }

        pipeline.push(pipeline_stage);
    }

    let mut handles = Vec::new();

    for chunk in seed_ranges.chunks(2) {
        let chunk = chunk.to_owned();
        let pipeline = pipeline.clone();

        let handle = thread::spawn(move || {
            let mut lowest_val = 0xFFFFFFFF;

            for mut seed in chunk[0]..=chunk[0]+chunk[1] {
                for stage in &pipeline {
                    seed = stage.get_dest(seed);
                }

                if seed < lowest_val {
                    lowest_val = seed;
                }
            }

            lowest_val
        });

        handles.push(handle);
    }

    // Wait for all threads to finish and collect their results
    let results: Vec<_> = handles.into_iter().map(|handle| handle.join().unwrap()).collect();

    // Find the lowest value among the results
    let lowest_val = *results.iter().min().unwrap();

    Some(lowest_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
