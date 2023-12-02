fn main() {
    let input = include_str!("./input1.txt");
    let ans = part1(input);

    println!("Day02 part 1 answer: {ans}");
}

#[derive(Default, Debug, Clone, Copy)]
struct Color {
    red: u32,
    green: u32,
    blue: u32,
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            // Get id and game sets
            let Some((id, set)) = line.split_once(": ").and_then(|(meta, game)| {
                meta.split(' ')
                    .next_back()
                    .unwrap()
                    .parse::<u32>()
                    .map(|i| (i, game))
                    .ok()
            }) else {
                panic!("invalid input")
            };

            // Process each game sets
            let mut c = Color::default();
            set.split("; ").for_each(|each| {
                each.split(", ").for_each(|cube| {
                    if cube.ends_with("red") {
                        let amt = cube
                            .split_once(' ')
                            .unwrap()
                            .0
                            .parse::<u32>()
                            .unwrap_or_default();
                        if amt > 12 {
                            c.red = amt;
                        }
                    } else if cube.ends_with("blue") {
                        let amt = cube
                            .split_once(' ')
                            .unwrap()
                            .0
                            .parse::<u32>()
                            .unwrap_or_default();
                        if amt > 12 {
                            c.blue = amt;
                        }
                    } else if cube.ends_with("green") {
                        let amt = cube
                            .split_once(' ')
                            .unwrap()
                            .0
                            .parse::<u32>()
                            .unwrap_or_default();
                        if amt > 12 {
                            c.green = amt;
                        }
                    }
                });
            });

            (c.red <= 12 && c.green <= 13 && c.blue <= 14).then_some(id)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            2,
            part1("Game 2: 1 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
        );
        assert_eq!(
            8,
            part1(
                r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )
        )
    }
}
