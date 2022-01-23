use clap::{AppSettings, Args, Parser, Subcommand};

const MAJOR: &str = "0";
const MINOR: &str = "1";
const FIX: &str = "1";
const VERBAL: &str = "TX Add && Balances List";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
// this causes help message to be printed when run with `tbb balances list` when
// it should just drop into the match block. however, this does print help when
// running `tbb balances` with the missing `list` argument.
#[clap(global_setting(AppSettings::ArgRequiredElseHelp))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Describes version
    Version {},
    /// Interact with balances (list...)
    Balances(BalancesSubcommand),
    /// Interact with txs (add...)
    Tx(TxSubcommand),
}

#[derive(Args)]
struct BalancesSubcommand {
    #[clap(subcommand)]
    command: Option<BalancesCommands>,
}

#[derive(Subcommand)]
enum BalancesCommands {
    /// Lists all balances
    List {},
}

#[derive(Args)]
struct TxSubcommand {}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    match &cli.command {
        Some(Commands::Version {}) => {
            println!("Version: {}.{}.{}-beta {}", MAJOR, MINOR, FIX, VERBAL);
        }
        Some(Commands::Balances(x)) => match &x.command {
            Some(BalancesCommands::List {}) => {
                println!("LIST")
            }
            None => {}
        },
        Some(Commands::Tx(_)) => {
            println!("TX");
        }
        None => {}
    }

    // Continued program logic goes here...
}
