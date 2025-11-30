use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
struct Range {
    start: u64,
    dst: u64,
    len: u64,
}

#[derive(Debug, PartialEq)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn new() -> Self {
        Self { ranges: vec![] }
    }

    fn add(&mut self, range: Range) {
        self.ranges.push(range);
        self.ranges.sort_unstable_by_key(|range| range.start);
    }

    fn transform(&self, seed: u64) -> u64 {
        match self.ranges.binary_search_by_key(&seed, |range| range.start) {
            Ok(index) if self.ranges[index].len >= 1 => self.ranges[index].dst,
            Ok(_) => seed,
            Err(0) => seed,
            Err(index) => {
                let range = &self.ranges[index - 1];

                if seed < range.start + range.len {
                    range.dst + seed - range.start
                } else {
                    seed
                }
            }
        }
    }
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = (Vec<u64>, [Map; 7]);
    type Output1 = u64;
    type Output2 = u64;

    fn parse(input: &str) -> Self::Input {
        let mut section_split = input.split("\n\n");

        let seeds = parse_seeds(section_split.next().unwrap());
        let map_1 = parse_map(section_split.next().unwrap());
        let map_2 = parse_map(section_split.next().unwrap());
        let map_3 = parse_map(section_split.next().unwrap());
        let map_4 = parse_map(section_split.next().unwrap());
        let map_5 = parse_map(section_split.next().unwrap());
        let map_6 = parse_map(section_split.next().unwrap());
        let map_7 = parse_map(section_split.next().unwrap());

        (seeds, [map_1, map_2, map_3, map_4, map_5, map_6, map_7])
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (seeds, steps) = input;

        seeds
            .iter()
            .map(|seed| {
                steps
                    .iter()
                    .fold(*seed, |transformed, map| map.transform(transformed))
            })
            .min()
            .unwrap()
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let (seeds, steps) = input;

        seeds
            .chunks_exact(2)
            .map(|chunk| {
                let (start, len) = (chunk[0], chunk[1]);
                (start..start + len)
                    .map(|seed| {
                        steps
                            .iter()
                            .fold(seed, |transformed, map| map.transform(transformed))
                    })
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap()
    }
}

fn parse_seeds(section: &str) -> Vec<u64> {
    let prefix = "seeds: ";
    let section = &section[prefix.len()..];

    section
        .split_whitespace()
        .map(|number| u64::from_str(number).unwrap())
        .collect()
}

fn parse_map(section: &str) -> Map {
    let mut map = Map::new();

    for range in section.lines().skip(1).map(|line| parse_map_line(line)) {
        map.add(range);
    }

    map
}

fn parse_map_line(line: &str) -> Range {
    let mut numbers = line
        .split_whitespace()
        .map(|number| u64::from_str(number).unwrap());

    Range {
        dst: numbers.next().unwrap(),
        start: numbers.next().unwrap(),
        len: numbers.next().unwrap(),
    }
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    impl From<(u64, u64, u64)> for Range {
        fn from(tuple: (u64, u64, u64)) -> Self {
            Self {
                dst: tuple.0,
                start: tuple.1,
                len: tuple.2,
            }
        }
    }

    fn get_input() -> <Solver as aoc::Solver>::Input {
        let seeds = vec![79, 14, 55, 13];
        let ranges = vec![
            vec![(50, 98, 2).into(), (52, 50, 48).into()],
            vec![(0, 15, 37).into(), (37, 52, 2).into(), (39, 0, 15).into()],
            vec![
                (49, 53, 8).into(),
                (0, 11, 42).into(),
                (42, 0, 7).into(),
                (57, 7, 4).into(),
            ],
            vec![(88, 18, 7).into(), (18, 25, 70).into()],
            vec![
                (45, 77, 23).into(),
                (81, 45, 19).into(),
                (68, 64, 13).into(),
            ],
            vec![(0, 69, 1).into(), (1, 0, 69).into()],
            vec![(60, 56, 37).into(), (56, 93, 4).into()],
        ];

        let mut maps = [
            Map::new(),
            Map::new(),
            Map::new(),
            Map::new(),
            Map::new(),
            Map::new(),
            Map::new(),
        ];

        for i in 0..7 {
            for range in &ranges[i] {
                maps[i].add(*range);
            }
        }

        dbg!(&maps);

        (seeds, maps)
    }

    #[test]
    fn parsing() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 35);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), 46);
    }
}
