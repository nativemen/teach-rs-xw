use anyhow::Result;
use clap::Command;
use quizzer::*;
use std::io::{self};

fn main() -> Result<()> {
    let matches = Command::new("Quizzer")
        .subcommand(Command::new("enter").about("Enter quiz questions"))
        .subcommand(Command::new("quiz").about("Take a quiz"))
        .get_matches();

    match matches.subcommand() {
        Some(("enter", _)) => enter_questions(),
        Some(("quiz", _)) => take_quiz(),
        _ => Ok(()),
    }
}

fn enter_questions() -> Result<()> {
    let mut questions = Vec::new();
    loop {
        let mut question = String::new();
        println!("Enter the question:");
        io::stdin().read_line(&mut question)?;
        question = question.trim().to_string();

        let mut options: [String; 4] = Default::default();
        for (i, option) in options.iter_mut().enumerate() {
            println!("Enter option {}:", i + 1);
            io::stdin().read_line(option)?;
            *option = option.trim().to_string();
        }

        let mut correct_answer = String::new();
        println!("Enter the number of the correct answer:");
        io::stdin().read_line(&mut correct_answer)?;
        let correct_answer = correct_answer.trim().parse::<usize>()?;

        questions.push(QuizQuestion {
            question,
            options,
            correct_answer,
        });

        println!("Do you want to add another question? (yes/no)");
        let mut another = String::new();
        io::stdin().read_line(&mut another)?;
        if another.trim().to_lowercase() != "yes" {
            break;
        }
    }

    save_questions(&questions, "questions.json")?;
    Ok(())
}
fn take_quiz() -> Result<()> {
    let questions = load_questions("questions.json")?;
    let mut score = 0;
    for (i, q) in questions.iter().enumerate() {
        println!("Question {}: {}", i + 1, q.question);
        for (j, option) in q.options.iter().enumerate() {
            println!("({}) {}", j + 1, option);
        }

        let mut answer = String::new();
        println!("Please input your answer:");
        io::stdin().read_line(&mut answer)?;
        let answer = answer.trim().parse::<usize>()?;

        if answer == q.correct_answer {
            score += 1;
        }
    }

    println!("Your scores is {} / {}", score, questions.len());

    Ok(())
}
