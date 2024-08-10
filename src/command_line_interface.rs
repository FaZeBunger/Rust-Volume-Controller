use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}


#[derive(Subcommand, Debug)]
pub enum Command {
    Increment(Application),
    Decrement(Application),
    Toggle(Application),
    AddGame(Application),
    AddBrowser(Application),
    AddApplication(Application),
    IncrementCurrentSelection,
    DecrementCurrentSelection,
    ListSessions,
    SetCurrentApplication(Application),
    SetCurrentGame,
    SetCurrentBrowser,
    ResetJSON,
    GetCurrentSelection,
}

#[derive(Args, Debug)]
pub struct Application {
    pub name: Option<String>,
}


