use clap::{Parser, Subcommand,Args};

#[derive(Parser)]
#[command(name ="rustbox")]
#[command(about = "A multi-tool Cli written in rust", long_about = None)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    FileExplorer{
        path:String,
    },
    
    WordCounter{
        path:String,
    },
    
    TaskManager{
        task:String,
        action:String,
    },
    
    NoteKeeper(NoteArgs),
    
    MathEval{
        #[command(flatten)]
        args: MathEvalArgs,
    },

    JsonViewer{
        #[command(flatten)]
        args: JsonViewerArgs,
    },

    WebScraper{
        #[command(flatten)]
        args: WebScraperArgs,
    },

    ApiClient,
}

#[derive(Args)]
pub struct NoteArgs {
    #[arg(short, long)]
    pub action: String,

    #[arg(short, long, default_value = "")]
    pub title:String,

    #[arg(short, long, default_value = "")]
    pub content: String,

    #[arg(short, long, default_value_t = 0)]
    pub id: u32,
}

#[derive(Args)]
pub struct MathEvalArgs {
    #[arg(short,long)]
    pub expr: String,
}

#[derive(Args)]
pub struct JsonViewerArgs {
    #[arg(short, long)]
    pub path: String
}

#[derive(Args)]
pub struct WebScraperArgs {
    #[arg(short, long)]
    pub url: String
}
