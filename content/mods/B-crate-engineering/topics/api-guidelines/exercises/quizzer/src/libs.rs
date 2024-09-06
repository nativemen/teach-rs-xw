use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Deserialize, Serialize)]
pub struct QuizQuestion {
    pub question: String,
    pub options: [String; 4],
    pub correct_answer: usize,
}

pub fn save_questions(questions: &[QuizQuestion], filename: &str) -> Result<()> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, questions)?;
    Ok(())
}

pub fn load_questions(filename: &str) -> Result<Vec<QuizQuestion>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let questions = serde_json::from_reader(reader)?;
    Ok(questions)
}
