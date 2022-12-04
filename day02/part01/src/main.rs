mod play;
mod round;

use play::Play;
use round::Round;

fn main() {
    let input = include_str!("../input.txt");

    let lines: Vec<(u32, u32)> = input
        .trim_end()
        .split("\n")
        .map(|line| {
            let plays = line
                .split_whitespace()
                .flat_map(|s| Play::try_from(s))
                .collect::<Vec<Play>>();
            let round = Round::new(plays[0], plays[1]);
            let result = round.get_result();
            (result.0 as u32, result.1 as u32)
        })
        .collect();

    let _player_1_result = lines.iter().fold(0, |acc, x| acc + x.0);
    let player_2_result = lines.iter().fold(0, |acc, x| acc + x.1);

    println!("Part 1: {}", player_2_result);
}
