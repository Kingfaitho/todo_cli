mod task;
use task::Task;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use clap::{Parser, Subcommand};
use std::path::Path;

const FILE_PATH: &str = "tasks.json";

#[derive(Parser)]
#[command(name = "ToDo CLI")]
#[command(about = "A simple command-line to-do list application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Complete { id: u32},
    Delete { id: u32},
}
fn load_tasks() -> io::Result<Vec<Task>> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(Vec::new());
    }
    let file = File::open(FILE_PATH)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}

fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(FILE_PATH)?;
let writer = BufWriter::new(file);
serde_json::to_writer_pretty(writer, tasks)?;
 Ok(())

}

fn main () -> io::Result<()> {
    let cli = Cli::parse();
    let mut tasks = load_tasks()?;

    match cli.command {
        Commands:: Add { description } => {
            let id = tasks.len() as u32 + 1;
            let task = Task {
                id,
                description,
                completed: false,
            };
            tasks.push(task);
            save_tasks(&tasks)?;
            println!("Task added.");
        }
        Commands::List => {
            for task in &tasks {
                println!(
                    "{}. [{}] {}",
                    task.id,
                    if task.completed { "x" } else { "" },
                    task.description
                );
            }
        }
        Commands::Complete { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.completed = true;
                save_tasks(&tasks)?;
                println!("Task marked as completed.");
            } else { 
                println!("Task not found.");
            }
        }
        Commands::Delete { id } => {
            let original_len = tasks.len();
            tasks.retain(|t| t.id !=id);
            if tasks.len() < original_len {
                save_tasks(&tasks)?;
                println!("Task deleted.");
            } else {
                println!("Task not found.");
            }
        }

    }
    Ok(())
}

