use crate::{play::Play};

#[derive(Debug, Clone, Copy)]
pub struct Round {
    play_1: Play,
    play_2: Play,
}

impl Round {
    pub fn new(play_1: Play, play_2: Play) -> Round {
        Round { play_1, play_2 }
    }

    pub fn get_result(&self) -> (u8, u8) {
        const WIN: u8 = 6;
        const DRAW: u8 = 3;

        let mut play_1_score: u8 = 0;
        let mut play_2_score: u8 = 0;

        if self.play_1 > self.play_2 {
            play_1_score += WIN;
        } else if self.play_1 < self.play_2 {
            play_2_score += WIN;
        } else {
            play_1_score += DRAW;
            play_2_score += DRAW;
        }

        play_1_score += self.play_1.get_value();
        play_2_score += self.play_2.get_value();

        return (play_1_score,play_2_score)
    }
}
