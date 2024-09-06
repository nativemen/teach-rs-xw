use anyhow::Result;
use clap::Command;
use quizzer::*;
use std::io;

const FILENAME: &str = "questions.json";

fn main() -> Result<()> {
    let matches = Command::new("Quizzer")
        .about("questions & quiz")
        .version("v1.0")
        .author("nativemen")
        .arg_required_else_help(true)
        .help_template(
            "{bin} {version}\n{author}\n{about}\n\nUSAGE:\n    {usage}\n\nOPTIONS:\n{all-args}",
        )
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
        println!("Please enter the question:");
        io::stdin().read_line(&mut question)?;
        question = question.trim().to_string();

        let mut options: [String; 4] = Default::default();
        for (i, option) in options.iter_mut().enumerate() {
            println!("Please enter option {}:", i + 1);
            io::stdin().read_line(option)?;
            *option = option.trim().to_string();
        }

        let mut correct_answer = String::new();
        println!("Please enter the number of the correct answer:");
        io::stdin().read_line(&mut correct_answer)?;
        let correct_answer = correct_answer.trim().parse::<usize>()?;

        questions.push(QuizQuestion {
            question,
            options,
            correct_answer,
        });

        let mut another = String::new();
        println!("Do you want to add another question? (yes/no)");
        io::stdin().read_line(&mut another)?;
        if another.trim().to_lowercase() != "yes" {
            break;
        }
    }

    save_questions(&questions, FILENAME)?;
    Ok(())
}

fn take_quiz() -> Result<()> {
    let mut score = 0;
    let questions = load_questions(FILENAME)?;
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
