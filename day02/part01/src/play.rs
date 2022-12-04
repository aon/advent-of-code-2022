use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for Play {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissors),
            _ => Err("Invalid play"),
        }
    }
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Play::Rock, Play::Rock) => true,
            (Play::Paper, Play::Paper) => true,
            (Play::Scissors, Play::Scissors) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Play::Rock, Play::Paper) => Some(Ordering::Less),
            (Play::Rock, Play::Scissors) => Some(Ordering::Greater),
            (Play::Paper, Play::Rock) => Some(Ordering::Greater),
            (Play::Paper, Play::Scissors) => Some(Ordering::Less),
            (Play::Scissors, Play::Rock) => Some(Ordering::Less),
            (Play::Scissors, Play::Paper) => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Play{
    pub fn get_value(&self) -> u8 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}
