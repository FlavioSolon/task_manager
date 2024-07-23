use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "Task Manager")]
#[clap(about = "Ferramenta CLI para gerenciar minhas tarefas", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Add {
        description: String, 
        #[clap(long = "due-date")] 
        due_date: Option<String>, 
        #[clap(long = "priority")]
        priority: Option<String>,
    },
    List {
        #[clap(long)]
        filter: Option<String>,
    },
    
}
