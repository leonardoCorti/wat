use anyhow::Result;
use clap::{ArgGroup, Args, Parser, Subcommand};

mod wwebjsserver_api;

/// wat — a command-line tool for interacting with WhatsApp
#[derive(Parser, Debug)]
#[command(name = "whatsapp-cli", version, about, long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Use a different config file
    #[arg(short = 'c', long = "config", value_name = "FILE")]
    config: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// list all chats
    Chats(ChatsCmd),

    /// send a message
    Message(MessageCmd),

    /// interactive tui
    #[cfg(feature = "tui")]
    Tui,
}

#[derive(Args, Debug)]
struct MessageCmd {
    /// number of the receiver
    to: String,
    /// text message
    message: String,
}

#[derive(Args, Debug)]
#[command(group(
    ArgGroup::new("ordering")
        .args(&["reverse_chronological_order", "alphabetical_order", "chronological_order"])
        .multiple(false) // disallow using both at once
))]
struct ChatsCmd {
    /// how many chats to print
    #[arg(short, long)]
    limit: Option<usize>,

    ///chronological order
    #[arg(short, long)]
    chronological_order: bool,

    /// reverse chronological order (default)
    #[arg(short, long)]
    reverse_chronological_order: bool,

    /// alphabetical order
    #[arg(short, long)]
    alphabetical_order: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("{:#?}", cli);

    match cli.command {
        Commands::Chats(mut ch) => handle_chats(&mut ch).await?,
        Commands::Message(MessageCmd) => {},
        #[cfg(feature = "tui")]
        Commands::Tui => launch_tui().await?,
    }

    Ok(())
}

async fn handle_chats(ch: &mut ChatsCmd) -> Result<()> {
    // enforcing the default order in options parsing
    if !ch.chronological_order && !ch.reverse_chronological_order && !ch.alphabetical_order {
        ch.reverse_chronological_order = true;
    }
    println!("{:#?}", ch);
    Ok(())
}

#[cfg(feature = "tui")]
async fn launch_tui() -> Result<()> {
    // A placeholder TUI entrypoint — implement with ratatui + crossterm
    println!("Launching TUI (placeholder). Build with --features tui to enable real TUI.");
    // TODO: build interactive UI to list chats, open chat, send messages, etc.
    Ok(())
}
