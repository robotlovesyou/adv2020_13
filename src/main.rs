use num_bigint::BigInt;

fn main() {
    let (timestamp, busses) = read_input(include_str!("../input.txt").lines());
    let part_one_answer = part_one(&timestamp, &busses);
    println!("part one answer is {}", part_one_answer);

    let part_two_answer = part_two(&busses);
    println!("part two answer is {}", part_two_answer);
}

fn read_input<'a>(mut lines: impl Iterator<Item = &'a str>) -> (i64, Vec<(BigInt, BigInt)>) {
    let timestamp: i64 = lines.next().unwrap().parse::<i64>().unwrap();
    let busses = lines.next().unwrap();
    let mut ids = Vec::new();
    for (i, bus) in busses.split(',').enumerate() {
        if let Ok(d) = bus.parse::<i64>() {
            ids.push((BigInt::from(d), BigInt::from(i)));
        }
    }
    (timestamp, ids)
}

fn part_one(timestamp: &i64, busses: &[(BigInt, BigInt)]) -> BigInt {
    let mut min_wait = BigInt::from(i64::max_value());
    let mut bus = BigInt::from(0);
    for (id, _) in busses {
        let rem = *timestamp % id;
        if rem == BigInt::from(0) {
            return BigInt::from(0);
        } else {
            let wait = id - rem;
            if wait < min_wait {
                min_wait = wait;
                bus = id.clone();
            }
        }
    }
    min_wait * bus
}

fn part_two(busses: &[(BigInt, BigInt)]) -> BigInt {
    let (first, _) = busses[0].clone();
    let mut gen = BigIntGenerator::new(first.clone(), first);
    let mut solution = BigInt::from(0);
    for i in 1..busses.len() {
        solution = solve_for(&busses[0..=i], gen);
        gen = BigIntGenerator::new(
            solution.clone(),
            busses[0..=i]
                .iter()
                .map(|(bus, _)| bus)
                .fold(BigInt::from(1), |acc, bus| acc * bus)
                .clone(),
        );
    }
    solution
}

fn solve_for(busses: &[(BigInt, BigInt)], gen: BigIntGenerator) -> BigInt {
    let zero = BigInt::from(0);
    for test in gen {
        let ok = busses
            .iter()
            .all(|(bus, offset)| (&test + offset) % bus == zero);
        if ok {
            return test;
        }
    }
    panic!("no solution");
}

struct BigIntGenerator {
    next: BigInt,
    incr: BigInt,
}

impl BigIntGenerator {
    fn new(start: BigInt, incr: BigInt) -> BigIntGenerator {
        BigIntGenerator { next: start, incr }
    }
}

impl Iterator for BigIntGenerator {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.clone();
        self.next += &self.incr;
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TWO_NUMS: &str = "123\n7,13";
    const TEST_THREE_NUMS: &str = "123\n7,13,17";
    const TEST_FOUR_NUMS: &str = "123\n7,13,17,x,19";

    #[test]
    fn it_returns_the_correct_start_offset_for_two_num_test() {
        let (_, busses) = read_input(TEST_TWO_NUMS.lines());
        let offset = part_two(&busses);
        assert_eq!(BigInt::from(77), offset);
    }

    #[test]
    fn it_returns_the_correct_start_offset_for_three_num_test() {
        let (_, busses) = read_input(TEST_THREE_NUMS.lines());
        let offset = part_two(&busses);
        assert_eq!(BigInt::from(168), offset);
    }

    #[test]
    fn it_returns_the_correct_start_offset_for_four_num_test() {
        let (_, busses) = read_input(TEST_FOUR_NUMS.lines());
        let offset = part_two(&busses);
        assert_eq!(BigInt::from(10997), offset);
    }

    #[test]
    fn it_correctly_processes_the_tests_from_the_page() {
        let tests: Vec<(&str, BigInt)> = vec![
            ("999\n7,13,x,x,59,x,31,19", BigInt::from(1068781)),
            ("999\n17,x,13,19", BigInt::from(3417)),
            ("999\n67,7,59,61", BigInt::from(754018)),
            ("999\n67,x,7,59,61", BigInt::from(779210)),
            ("999\n67,7,x,59,61", BigInt::from(1261476)),
            ("999\n1789,37,47,1889", BigInt::from(1202161486)),
        ];
        for (input, expected) in tests {
            let (_, busses) = read_input(input.lines());
            let offset = part_two(&busses);
            assert_eq!(expected, offset);
        }
    }
}
