fn main() {
    // Read the input from input1.txt
    let input = include_str!("./input1.txt");
    let ans = part1(input);

    println!("Day01 part 1 answer: {ans}");
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut iter = line.chars().filter_map(|c| c.to_digit(10).or(None));

            match (iter.next(), iter.next_back()) {
                (Some(x), Some(y)) => x * 10 + y,
                (Some(x), None) => x * 10 + x,
                (None, Some(y)) => y * 10 + y,
                (None, None) => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(10, part1("1sdfsdgerg34534t5fwefwe0"));
        assert_ne!(11, part1("1sdfsdgerg34534t5fwefwe0"))
    }
}
