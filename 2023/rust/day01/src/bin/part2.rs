fn main() {
    // Read the input from input1.txt
    let input = include_str!("./input2.txt");
    let ans = part2(input);

    println!("Day02 part 2 answer: {ans}");
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(process_line)
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

fn process_line(s: &str) -> String {
    let number: [(&str, &str); 9] = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let mut temp = String::new();
    let mut out = String::new();

    s.chars().for_each(|c| {
        temp.push(c);
        out.push(c);

        for (num, digit) in number {
            if temp.ends_with(num) {
                out.push_str(digit)
            }
        }
    });

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(29, part2("two1nine"));
        assert_eq!(83, part2("eightwothree"));
        assert_eq!(13, part2("abcone2threexyz"));
        assert_eq!(24, part2("xtwone3four"));
        assert_eq!(42, part2("4nineeightseven2"));
        assert_eq!(14, part2("zoneight234"));
        assert_eq!(76, part2("7pqrstsixteen"));
        assert_eq!(21, part2("xkfd2eightsevenfourtwonegr"));
    }
}
