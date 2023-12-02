fn main() {
    let input = include_str!("./input1.txt");
    let ans = part2(input);

    println!("Day02 part 1 answer: {ans}");
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|game| {
            let Some((_, content)) = game.split_once(": ") else {
                panic!("invalid input")
            };

            let (mut red_max, mut blue_max, mut green_max) =
                (std::u32::MIN, std::u32::MIN, std::u32::MIN);

            content.split("; ").for_each(|set| {
                set.split(", ").for_each(|cube| {
                    let Some((amt, color)) = cube.split_once(' ') else {
                        panic!("invalid input")
                    };
                    let amt = amt.parse::<u32>().unwrap_or_default();
                    match color {
                        "red" if amt > red_max => red_max = amt,
                        "blue" if amt > blue_max => blue_max = amt,
                        "green" if amt > green_max => green_max = amt,
                        _ => (),
                    };
                });
            });
            red_max * blue_max * green_max
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part2() {
        assert_eq!(
            2286,
            part2(
                r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )
        )
    }
}
