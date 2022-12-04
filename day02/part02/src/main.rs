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
            let mut plays = line.split_whitespace();

            let play_1 = Play::try_from(plays.next().unwrap()).unwrap();
            let expected_result = plays.next().unwrap();

            let play_2 = match expected_result {
                "X" => play_1.get_losing_play(),
                "Y" => play_1.get_drawing_play(),
                "Z" => play_1.get_winning_play(),
                _ => unreachable!("Invalid input"),
            };

            let round = Round::new(play_1, play_2);
            let result = round.get_result();
            (result.0 as u32, result.1 as u32)
        })
        .collect();

    let _player_1_result = lines.iter().fold(0, |acc, x| acc + x.0);
    let player_2_result = lines.iter().fold(0, |acc, x| acc + x.1);

    println!("Part 1: {}", player_2_result);
}
