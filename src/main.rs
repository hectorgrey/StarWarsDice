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
}

struct AbilityDie;
struct ProficiencyDie;
struct BoostDie;
struct DifficultyDie;
struct ChallengeDie;
struct SetbackDie;
struct ForceDie;

trait Die {
    fn roll() -> DieValue;
}

impl Die for AbilityDie {
    fn roll() -> DieValue {
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
    fn roll() -> DieValue {
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
    fn roll() -> DieValue {
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
    fn roll() -> DieValue {
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
    fn roll() -> DieValue {
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
    fn roll() -> DieValue {
        let result = rand::thread_rng().gen_range(1, 7);
        match result {
            1 | 2 => DieValue::Blank,
            3 | 4 => DieValue::Failure,
            _     => DieValue::Threat,
        }
    }
}

impl Die for ForceDie {
    fn roll() -> DieValue {
        let result = rand::thread_rng().gen_range(1, 13);
        match result {
            1 ... 6 => DieValue::Dark,
            7       => DieValue::DarkDark,
            8 ... 9 => DieValue::Light,
            _       => DieValue::LightLight,
        }
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
        };
    }
}

fn normal_roll() {
    let mut input = String::new();

    println!("Please enter the dice you need:");
    println!("(use A for Ability dice, P for Proficiency dice, B for Boost dice,\n\
              D for Difficulty dice, C for Challenge dice, S for setback dice and\n\
              F for Force dice, like so: \"A2 P1 D3 F2\")");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let inputs: Vec<&str> = input.split_whitespace().collect();

    let mut ability = 0;
    let mut proficiency = 0;
    let mut boost = 0;
    let mut difficulty = 0;
    let mut challenge = 0;
    let mut setback = 0;
    let mut force = 0;

    for i in inputs {
        let (code, number_as_str) = i.split_at(1);
        let number = number_as_str.parse().expect("Incorrect Input");
        match code {
            "A" => ability += number,
            "P" => proficiency += number,
            "B" => boost += number,
            "D" => difficulty += number,
            "C" => challenge += number,
            "S" => setback += number,
            "F" => force += number,
            _   => {},
        };
    }

    roll_normal_dice(ability, proficiency, boost, difficulty, challenge,
                     setback, force);
}

fn roll_normal_dice(ability: u8, proficiency: u8, boost: u8, difficulty: u8,
                    challenge: u8, setback: u8, force: u8) {
    let mut hand = Vec::new();

    for _ in 0..ability {
        hand.push(AbilityDie::roll());
    }

    for _ in 0..proficiency {
        hand.push(ProficiencyDie::roll());
    }

    for _ in 0..boost {
        hand.push(BoostDie::roll());
    }

    for _ in 0..difficulty {
        hand.push(DifficultyDie::roll());
    }

    for _ in 0..challenge {
        hand.push(ChallengeDie::roll());
    }

    for _ in 0..setback {
        hand.push(SetbackDie::roll());
    }

    for _ in 0..force {
        hand.push(ForceDie::roll());
    }

    let mut advantage = 0;
    let mut success = 0;
    let mut triumph = 0;
    let mut light = 0;
    let mut threat = 0;
    let mut failure = 0;
    let mut dispair = 0;
    let mut dark = 0;

    for die in hand {
        match die {
            DieValue::Advantage          => advantage += 1,
            DieValue::Success            => success += 1,
            DieValue::AdvantageSuccess   => {advantage += 1; success += 1;},
            DieValue::AdvantageAdvantage => advantage += 2,
            DieValue::SuccessSuccess     => success += 2,
            DieValue::Triumph            => triumph += 1,
            DieValue::Threat             => threat += 1,
            DieValue::Failure            => failure += 1,
            DieValue::ThreatFailure      => {threat += 1; failure += 1},
            DieValue::ThreatThreat       => threat += 2,
            DieValue::FailureFailure     => failure += 2,
            DieValue::Dispair            => dispair += 1,
            DieValue::Dark               => dark += 1,
            DieValue::DarkDark           => dark += 2,
            DieValue::Light              => light += 1,
            DieValue::LightLight         => light += 2,
            _                            => {},
        }
    }

    println!("You rolled {} successes, {} advantage, {} triumphs,\n\
             {} failures, {} threat, {} dispair,\n\
             {} light side force points and {} dark side force points.",
             failure, threat, dispair, success, advantage, triumph, light,
             dark);
}

fn d10_roll() {
    let mut input = String::new();

    println!("How many d10 do you require?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let dice: u8 = input.trim().parse().expect("Expected a number.");
    input.clear();

    println!("What do you wish to add to your die roll?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let modifier: u8 = input.trim().parse().expect("Expected a number.");
    input.clear();

    let mut total = modifier;
    for _ in 0..dice {
        total += rand::thread_rng().gen_range(1, 11);
    }

    println!("You rolled a total of {}.", total);
}

fn d100_roll() {
    let mut input = String::new();
    println!("What do you wish to add to your die roll?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let modifier: u8 = input.trim().parse().expect("Expected a number.");

    let mut total = modifier;
    total += rand::thread_rng().gen_range(1, 101);

    println!("You rolled a total of {}.", total);
}
