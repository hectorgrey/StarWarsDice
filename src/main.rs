extern crate rand;

use std::io;
use rand::Rng;

enum DieValue {
    Blank,
    Advantage,
    Success,
    AdvantageSuccess,
    AdvantageAdvantage,
    SuccessSuccess,
    Triumph,
    Threat,
    Failure,
    ThreatFailure,
    ThreatThreat,
    FailureFailure,
    Dispair,
    Dark,
    DarkDark,
    Light,
    LightLight,
    Numeric(u8),
}

struct AbilityDie;
struct ProficiencyDie;
struct BoostDie;
struct DifficultyDie;
struct ChallengeDie;
struct SetbackDie;
struct ForceDie;
struct D10;
struct D100;

trait Die {
    fn roll () -> DieValue;
}

impl Die for AbilityDie {
    fn roll () -> DieValue {
        let result = rand::thread_rng().gen_range(1, 9);
        match result {
            1     => DieValue::Blank,
            2 | 3 => DieValue::Success,
            4     => DieValue::SuccessSuccess,
            5 | 6 => DieValue::Advantage,
            7     => DieValue::AdvantageSuccess,
            _     => DieValue::AdvantageAdvantage,
        }
    }
}

impl Die for ProficiencyDie {
    fn roll () -> DieValue {
        let result = rand::thread_rng().gen_range(1, 13);
        match result {
            1       => DieValue::Blank,
            2 | 3   => DieValue::Success,
            4 | 5   => DieValue::SuccessSuccess,
            6       => DieValue::Advantage,
            8 | 9   => DieValue::AdvantageSuccess,
            10 | 11 => DieValue::AdvantageAdvantage,
            _       => DieValue::Triumph,
        }
    }
}

impl Die for BoostDie {
    fn roll () -> DieValue {
        let result = rand::thread_rng().gen_range(1, 7);
        match result {
            1 | 2 => DieValue::Blank,
            3     => DieValue::Success,
            4     => DieValue::AdvantageSuccess,
            5     => DieValue::AdvantageAdvantage,
            _     => DieValue::Advantage,
        }
    }
}

impl Die for DifficultyDie {
    fn roll () -> DieValue {
        let result = rand::thread_rng().gen_range(1, 9);
        match result {
            1         => DieValue::Blank,
            2         => DieValue::Failure,
            3         => DieValue::FailureFailure,
            4 | 5 | 6 => DieValue::Threat,
            7         => DieValue::ThreatThreat,
            _         => DieValue::ThreatFailure,
        }
    }
}

impl Die for ChallengeDie {
    fn roll () -> DieValue {
        let result = rand::thread_rng().gen_range(1, 13);
        match result {
            1       => DieValue::Blank,
            2  | 3  => DieValue::Failure,
            4  | 5  => DieValue::FailureFailure,
            6  | 7  => DieValue::Threat,
            8  | 9  => DieValue::ThreatFailure,
            10 | 11 => DieValue::ThreatThreat,
            _       => DieValue::Dispair,
        }
    }
}

impl Die for SetbackDie {
    fn roll () -> DieValue {
        let result = rand::thread_rng().gen_range(1, 7);
        match result {
            1 | 2 => DieValue::Blank,
            3 | 4 => DieValue::Failure,
            _     => DieValue::Threat,
        }
    }
}

impl Die for ForceDie {
    fn roll () -> DieValue {
        let result = rand::thread_rng().gen_range(1, 13);
        match result {
            1 ... 6 => DieValue::Dark,
            7       => DieValue::DarkDark,
            8 ... 9 => DieValue::Light,
            _       => DieValue::LightLight,
        }
    }
}

impl Die for D10 {
    fn roll() -> DieValue {
        DieValue::Numeric(rand::thread_rng().gen_range(1, 11))
    }
}

impl Die for D100 {
    fn roll() -> DieValue {
        DieValue::Numeric(rand::thread_rng().gen_range(1, 101))
    }
}

fn main() {
    loop {
        println!("What kind of roll do you require?");
        println!("(\"normal\", \"d10\" or \"d100\", or \"quit\" to quit)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer)
                   .expect("Failed to read input");
        match answer.trim() {
            "quit"   => break,
            "normal" => normal_roll(),
            "d10"    => d10_roll(),
            "d100"   => d100_roll(),
            _        => println!("Input not recognised: {}", answer.trim()),
        }
    }
}

fn normal_roll() {
    let mut input = String::new();
    println!("How many Ability dice (green d8) do you require?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ability: u8 = input.trim().parse().expect("Expected a number.");
    println!("How many Proficiency dice (yellow d12) do you require?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let proficiency: u8 = input.trim().parse().expect("Expected a number.");
    println!("How many Boost dice (blue d6) do you require?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let boost: u8 = input.trim().parse().expect("Expected a number.");
    println!("How many Difficulty dice (purple d8) do you require?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let difficulty: u8 = input.trim().parse().expect("Expected a number.");
    println!("How many Challenge dice (red d12) do you require?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let challenge: u8 = input.trim().parse().expect("Expected a number.");
    println!("How many Setback dice (black d6) do you require?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let setback: u8 = input.trim().parse().expect("Expected a number.");
    println!("How many Force dice (white d12) do you require?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let force: u8 = input.trim().parse().expect("Expected a number.");
}

fn d10_roll() {
    println!("How many d10 do you require?");
    println!("What do you wish to add to your die roll?");
}

fn d100_roll() {
    println!("What do you wish to add to your die roll?");
}
