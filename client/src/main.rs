use anyhow::Result;
use clap::{Args, Parser, Subcommand};

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
    /// Work with chats
    Chats(ChatsCmd),

    /// Send messages
    Send(SendArgs),

    /// Download an attachment
    Download(DownloadArgs),

    /// Account / session management (login, logout, status)
    Session(SessionCmd),

    /// Misc utilities (search, export, settings)
    Misc(MiscCmd),

    /// interactive tui
    #[cfg(feature = "tui")]
    Tui,
}

#[derive(Args, Debug)]
struct ChatsCmd {
    #[command(subcommand)]
    sub: ChatsSub,
}

#[derive(Subcommand, Debug)]
enum ChatsSub {
    /// List recent chats
    List {
        /// show full details (last message preview, unread count)
        #[arg(short, long)]
        details: bool,

        /// limit results
        #[arg(short, long, default_value_t = 50)]
        limit: usize,
    },

    /// Print the full chat with a contact or group
    Print {
        /// contact phone number or group id (human readable allowed)
        #[arg(value_name = "TARGET")]
        target: String,

        /// number of messages to print (most recent)
        #[arg(short, long, default_value_t = 100)]
        count: usize,

        /// show timestamps
        #[arg(short, long)]
        timestamps: bool,
    },

    /// Show metadata for a given chat
    Info { target: String },

    /// Archive a chat
    Archive { target: String },

    /// Unarchive a chat
    Unarchive { target: String },
}

#[derive(Args, Debug)]
struct SendArgs {
    /// recipient phone number or group id
    #[arg(value_name = "TO")]
    to: String,

    /// message text (if omitted, reads from stdin)
    #[arg(value_name = "MESSAGE")]
    message: Option<String>,

    /// path to file to attach
    #[arg(short, long, value_name = "FILE")]
    attach: Option<String>,

    /// schedule the message (RFC3339 timestamp)
    #[arg(long, value_name = "TIME")]
    at: Option<String>,
}

#[derive(Args, Debug)]
struct DownloadArgs {
    /// message id or attachment id
    #[arg(value_name = "ATTACHMENT_ID")]
    attachment_id: String,

    /// output path
    #[arg(short, long, default_value = ".")]
    out: String,
}

#[derive(Args, Debug)]
struct SessionCmd {
    #[command(subcommand)]
    sub: SessionSub,
}

#[derive(Subcommand, Debug)]
enum SessionSub {
    /// Start or resume a session (opens QR flow or uses stored credentials)
    Login,
    /// Logout and clear session
    Logout,
    /// Show session status
    Status,
}

#[derive(Args, Debug)]
struct MiscCmd {
    #[command(subcommand)]
    sub: MiscSub,
}

#[derive(Subcommand, Debug)]
enum MiscSub {
    /// Search messages across chats
    Search {
        query: String,
        #[arg(short, long, default_value_t = 50)]
        limit: usize,
    },

    /// Export a chat to JSON or plain text
    Export {
        target: String,
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// List contacts
    Contacts {
        #[arg(short, long)]
        details: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Chats(ch) => handle_chats(ch).await?,
        Commands::Send(s) => handle_send(s).await?,
        Commands::Download(d) => handle_download(d).await?,
        Commands::Session(s) => handle_session(s).await?,
        Commands::Misc(m) => handle_misc(m).await?,
        #[cfg(feature = "tui")]
        Commands::Tui => launch_tui().await?,
    }

    Ok(())
}

async fn handle_chats(cmd: ChatsCmd) -> Result<()> {
    match cmd.sub {
        ChatsSub::List { details, limit } => {
            println!("Listing up to {} chats (details={})", limit, details);
            // TODO: query backend and print lists
        }
        ChatsSub::Print {
            target,
            count,
            timestamps,
        } => {
            println!(
                "Printing {} messages for '{}' (timestamps={})",
                count, target, timestamps
            );
            // TODO: fetch messages from backend and print
        }
        ChatsSub::Info { target } => {
            println!("Chat info for {}", target);
        }
        ChatsSub::Archive { target } => println!("Archiving {}", target),
        ChatsSub::Unarchive { target } => println!("Unarchiving {}", target),
    }
    Ok(())
}

async fn handle_send(args: SendArgs) -> Result<()> {
    let message = if let Some(m) = args.message {
        m
    } else {
        // read from stdin
        use tokio::io::{self, AsyncReadExt};
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).await?;
        buf
    };

    println!(
        "Sending to {}: {} (attach={:?}, at={:?})",
        args.to, message, args.attach, args.at
    );
    // TODO: enqueue/send using backend
    Ok(())
}

async fn handle_download(args: DownloadArgs) -> Result<()> {
    println!("Downloading {} to {}", args.attachment_id, args.out);
    // TODO: download from backend
    Ok(())
}

async fn handle_session(cmd: SessionCmd) -> Result<()> {
    match cmd.sub {
        SessionSub::Login => println!("Starting login flow (QR / credentials)..."),
        SessionSub::Logout => println!("Logging out and clearing credentials..."),
        SessionSub::Status => println!("Session status: not implemented"),
    }
    Ok(())
}

async fn handle_misc(cmd: MiscCmd) -> Result<()> {
    match cmd.sub {
        MiscSub::Search { query, limit } => println!("Searching for '{}' (limit={})", query, limit),
        MiscSub::Export { target, format } => println!("Exporting {} as {}", target, format),
        MiscSub::Contacts { details } => println!("Listing contacts (details={})", details),
    }
    Ok(())
}

// -------------------------
// Optional TUI (only compiled when feature "tui" is enabled)
// -------------------------

#[cfg(feature = "tui")]
async fn launch_tui() -> Result<()> {
    // A placeholder TUI entrypoint — implement with ratatui + crossterm
    println!("Launching TUI (placeholder). Build with --features tui to enable real TUI.");
    // TODO: build interactive UI to list chats, open chat, send messages, etc.
    Ok(())
}
