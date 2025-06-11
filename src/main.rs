mod cli;
use cli::{Cli, Commands};
use clap::Parser;

mod file_explorer;
mod word_counter;
mod task_manager;
mod note_keeper;
mod math_eval;
mod json_viewer;
mod web_scraper;
mod api_client;

use note_keeper::{NoteKeeper};
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::FileExplorer{ path } => {
            file_explorer::run(&path);
        }
        Commands::WordCounter{ path } => {
            word_counter::run(&path);
        }
        Commands::TaskManager{task,action} => {
            task_manager::run(task,action)        
        }
        Commands::NoteKeeper(args) => {
            let mut keeper = NoteKeeper::new();

            match args.action.as_str() {
                
                "add" => {
                    if args.title.is_empty() || args.content.is_empty(){
                        eprintln!("Please provide both title an content.");
                    } else {
                        keeper.add_note(&args.title, &args.content);
                    }
                }
                
                "list" => {
                    keeper.list_notes();
                }
                
                "view" => {
                    if let Some(note) = keeper.view_note(args.id){
                        println!(" [{}] {}\n{}", note.id, note.title, note.content);
                    } else {
                        eprintln!("Note not found.");
                    }
                }
                
                "delete" => {
                    keeper.delete_note(args.id);
                }
                
                _ => {
                    eprintln!(" Unknown action: {}", args.action);
                }
            }
        }
        Commands::MathEval{args} => {
            math_eval::run(&args.expr);
        }
        Commands::JsonViewer{args} => {
            json_viewer::run(&args.path);
        }
        Commands::WebScraper { args } => {
            if let Err(e) = web_scraper::run(&args.url).await {
                eprintln!("Error:{}", e);
            }
        }
        Commands::ApiClient => {
            if let Err(e) = api_client::run().await {
                eprintln!("API Call failed: {}", e);
            }
        }

    }
}
