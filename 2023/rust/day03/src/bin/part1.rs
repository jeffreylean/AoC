fn main() {
    let input = include_str!("./input1.txt");
    let ans = part1(input);

    println!("Day03 part 1 answer: {ans}");
}

fn part1(input: &str) -> u32 {
    let mut line_iter = input.lines().skip(1).peekable();
    let mut curr = Vec::<bool>::new();
    let mut prev = Vec::<bool>::new();

    input
        .lines()
        .map(|line| {
            let mut num_str = String::new();
            let mut sum = 0;
            let mut next = Vec::<bool>::new();

            if line_iter.peek().is_some() {
                next = line_iter
                    .next()
                    .unwrap()
                    .chars()
                    .map(validate_symbol)
                    .collect();
            }
            if curr.is_empty() {
                curr = line.chars().map(validate_symbol).collect();
            }

            let mut is_part = false;
            line.char_indices().for_each(|(i, c)| {
                if c.is_ascii_digit() {
                    num_str.push_str(&c.to_string());
                    match i {
                        0 => {
                            if (!prev.is_empty() && (prev[i] || prev[i + 1]))
                                || (!next.is_empty() && (next[i] || next[i + 1]))
                                || curr[i + 1]
                            {
                                is_part = true;
                            }
                        }
                        _ if !curr.is_empty() && curr.len() - 1 == i => {
                            if (!prev.is_empty() && (prev[i] || prev[i - 1]))
                                || (!next.is_empty() && (next[i] || next[i - 1]))
                                || curr[i - 1]
                            {
                                is_part = true;
                            }
                        }
                        _ => {
                            if (!prev.is_empty() && (prev[i] || prev[i - 1] || prev[i + 1]))
                                || (!next.is_empty() && (next[i] || next[i - 1] || next[i + 1]))
                                || (curr[i - 1] || curr[i + 1])
                            {
                                is_part = true;
                            }
                        }
                    }
                    // If number is at the last position
                    if i == curr.len() - 1 {
                        if is_part {
                            sum += num_str.parse::<u32>().unwrap_or_default();
                            is_part = false
                        }
                        num_str.clear();
                    }
                } else {
                    if is_part {
                        sum += num_str.parse::<u32>().unwrap_or_default();
                        is_part = false
                    }
                    num_str.clear();
                }
            });

            prev = curr.to_vec();
            curr = next.to_vec();
            next.clear();

            sum
        })
        .sum()
}

fn validate_symbol(c: char) -> bool {
    let c = c as u8;
    if (48..=57).contains(&c) || c == 46 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            4361,
            part1(
                r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )
        );
        assert_eq!(
            4420,
            part1(
                r"467*.114.1
.........*
.*35.633&.
..........
617*...*..
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )
        )
    }
}
