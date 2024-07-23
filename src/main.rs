use clap::Parser;
use std::error::Error;

mod cli;
mod storage;
mod task;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Cli::parse();

    // Carrega as tarefas existentes do arquivo
    let mut tasks = storage::load_tasks().unwrap_or_else(|_| Vec::new());

    match args.command {
        cli::Command::Add {
            description,
            due_date,
            priority,
        } => {
            // Adiciona uma nova tarefa
            let new_task = task::Task {
                description,
                due_date,
                priority,
                completed: false,
            };
            tasks.push(new_task);
            println!("Tarefa adicionada com sucesso!");
        }
        cli::Command::List { filter: _ } => {
            // Lista todas as tarefas
            for task in &tasks { // Itera sobre uma referência ao vetor
                println!("{:?}", task);
            }
        }
    }

    // Salva as tarefas atualizadas no arquivo
    storage::save_tasks(&tasks)?;

    Ok(())
}
