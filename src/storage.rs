use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::result::Result;
use crate::task::Task;

const FILE_PATH: &str = "tasks.json";

pub fn load_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let file = File::open(FILE_PATH)?;
    let reader = BufReader::new(file);
    let tasks: Vec<Task> = serde_json::from_reader(reader)?;
    Ok(tasks)
}

pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(FILE_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, tasks)?;
    Ok(())
}
