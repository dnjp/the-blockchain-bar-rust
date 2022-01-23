use clap::{Parser, Subcommand};

const MAJOR: &str = "0";
const MINOR: &str = "1";
const FIX: &str = "1";
const VERBAL: &str = "TX Add && Balances List";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Describes version
    Version {},

    /// Interact with balances (list...)
    #[clap(subcommand)]
    Balances(BalancesCommands),

    /// Interact with txs (add...)
    #[clap(subcommand)]
    Tx(TxCommands),
}

#[derive(Subcommand, Debug)]
enum BalancesCommands {
    /// Lists all balances
    List {},
}

#[derive(Subcommand, Debug)]
enum TxCommands {
    /// Adds new TX to database
    Add {
        /// From what account to send tokens
        #[clap(long)]
        from: String,

        /// To what account to send tokens
        #[clap(long)]
        to: String,

        /// How many tokens to send
        #[clap(long)]
        value: u64,

        /// Possible values: 'reward'
        #[clap(long)]
        #[clap(default_value_t = String::from(""))]
        data: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Version {}) => {
            println!("Version: {}.{}.{}-beta {}", MAJOR, MINOR, FIX, VERBAL);
        }
        Some(Commands::Balances(balances)) => match balances {
            BalancesCommands::List {} => {
                println!("LIST");
            }
        },
        Some(Commands::Tx(tx)) => match tx {
            TxCommands::Add {
                from: _,
                to: _,
                value: _,
                data: _,
            } => {
                println!("ADD: {:?}", tx);
            }
        },
        None => {}
    }
}

#[test]
fn verify_app() {
    use clap::IntoApp;
    Cli::into_app().debug_assert()
}
