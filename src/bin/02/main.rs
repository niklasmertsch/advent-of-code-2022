use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::Move::{Paper, Rock, Scissors};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rock => write!(f, "Rock"),
            Paper => write!(f, "Paper"),
            Scissors => write!(f, "Scissors"),
        }
    }
}

impl Move {
    fn from(c: char) -> Option<Move> {
        match c {
            'A' => Option::from(Rock),
            'B' => Option::from(Paper),
            'C' => Option::from(Scissors),
            'X' => Option::from(Rock),
            'Y' => Option::from(Paper),
            'Z' => Option::from(Scissors),
            _ => None,
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Rock, Rock) => Option::from(Ordering::Equal),
            (Rock, Paper) => Option::from(Ordering::Less),
            (Rock, Scissors) => Option::from(Ordering::Greater),
            (Paper, Rock) => Option::from(Ordering::Greater),
            (Paper, Paper) => Option::from(Ordering::Equal),
            (Paper, Scissors) => Option::from(Ordering::Less),
            (Scissors, Rock) => Option::from(Ordering::Less),
            (Scissors, Paper) => Option::from(Ordering::Greater),
            (Scissors, Scissors) => Option::from(Ordering::Equal),
        }
    }
}

fn main() {
    let moves = get_moves(include_str!("input.txt"));
    println!("game score: {}", get_game_score(&moves));
    println!("updated game score: {}", get_game_score(&update_moves(&moves)));
}

fn get_moves(input: &str) -> Vec<(Move, Move)> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut chars = line.chars();
        let their = Move::from(chars.next().expect("No first char")).expect("invalid char");
        chars.next();
        let my = Move::from(chars.next().expect("No first char")).expect("invalid char");
        result.push((my, their));
    }
    return result;
}

fn get_game_score(moves: &Vec<(Move, Move)>) -> i32 {
    let mut score = 0;
    for (my, their) in moves {
        let single_score = get_single_score(&my, &their);
        score += single_score;
    }
    return score;
}

fn get_single_score(my: &Move, their: &Move) -> i32 {
    let mut score = 0;
    match my {
        Rock => score += 1,
        Paper => score += 2,
        Scissors => score += 3,
    }
    match my.partial_cmp(their).expect("Failed to order") {
        Ordering::Greater => score += 6,
        Ordering::Equal=> score += 3,
        Ordering::Less=> score += 0,
    }
    return score;
}

fn update_moves(moves: &Vec<(Move, Move)>) -> Vec<(Move, Move)> {
    let mut new_moves = Vec::new();
    for (my, their) in moves {
        let my_new = match (their, my) {
            (Rock, Rock) => Scissors,
            (Rock, Paper) => Rock,
            (Rock, Scissors) => Paper,
            (Paper, Rock) => Rock,
            (Paper, Paper) => Paper,
            (Paper, Scissors) => Scissors,
            (Scissors, Rock) => Paper,
            (Scissors, Paper) => Scissors,
            (Scissors, Scissors) => Rock,
        };
        new_moves.push((my_new, their.clone()));
    }
    return new_moves;
}


#[cfg(test)]
mod tests {
    use crate::{get_moves, get_game_score, get_single_score, Move, update_moves};
    use crate::Move::{Paper, Rock, Scissors};

    #[test]
    fn test_move_from() {
        assert_eq!(Move::from('A').expect("parse failed"), Rock);
        assert_eq!(Move::from('X').expect("parse failed"), Rock);

        assert_eq!(Move::from('B').expect("parse failed"), Paper);
        assert_eq!(Move::from('Y').expect("parse failed"), Paper);

        assert_eq!(Move::from('C').expect("parse failed"), Scissors);
        assert_eq!(Move::from('Z').expect("parse failed"), Scissors);

        assert!(Move::from(' ').is_none())
    }
    
    #[test]
    fn test_move_ordering() {
        assert!(Rock > Scissors);
        assert!(Scissors > Paper);
        assert!(Paper > Rock);
    }

    #[test]
    fn test_get_moves_on_empty_input() {
        let moves = get_moves("");

        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn test_get_move() {
        let moves = get_moves("A Y\nB X\nC Z\n");

        assert_eq!(moves.len(), 3);

        assert_eq!(moves[0], (Paper, Rock));
        assert_eq!(moves[1], (Rock, Paper));
        assert_eq!(moves[2], (Scissors, Scissors));
    }

    #[test]
    fn test_get_single_score() {
        // loss
        assert_eq!(1, get_single_score(&Rock, &Paper));
        assert_eq!(2, get_single_score(&Paper, &Scissors));
        assert_eq!(3, get_single_score(&Scissors, &Rock));

        // draw
        assert_eq!(4, get_single_score(&Rock, &Rock));
        assert_eq!(5, get_single_score(&Paper, &Paper));
        assert_eq!(6, get_single_score(&Scissors, &Scissors));

        // win
        assert_eq!(7, get_single_score(&Rock, &Scissors));
        assert_eq!(8, get_single_score(&Paper, &Rock));
        assert_eq!(9, get_single_score(&Scissors, &Paper));
    }

    #[test]
    fn test_get_game_score() {
        let moves = Vec::from([
            (Rock, Paper),
            (Paper, Rock),
            (Scissors, Scissors),
        ]);
        assert_eq!(15, get_game_score(&moves))
    }

    #[test]
    fn test_update_moves() {
        let moves = get_moves("A Y\nB X\nC Z\n");
        let updates_moves = update_moves(&moves);

        assert_eq!(updates_moves, Vec::from([
            (Rock, Rock),
            (Rock, Paper),
            (Rock, Scissors),
        ]));

        assert_eq!(12, get_game_score(&updates_moves));
    }
}