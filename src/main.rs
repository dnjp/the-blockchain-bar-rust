use clap::{Args, Parser, Subcommand};

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

#[derive(Args, Debug)]
struct Transaction {
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
}

#[derive(Subcommand, Debug)]
enum TxCommands {
    /// Adds new TX to database
    Add(Transaction),
}

fn print_version() {
    println!("Version: {}.{}.{}-beta {}", MAJOR, MINOR, FIX, VERBAL);
}

fn list_balances() {
    println!("LIST BALANCES");
}

fn add_transaction(tx: &Transaction) {
    println!("ADD: {:?}", tx);
}

fn parse(commands: &Option<Commands>) {
    match commands {
        Some(Commands::Version {}) => print_version(),
        Some(Commands::Balances(balances)) => match balances {
            BalancesCommands::List {} => list_balances(),
        },
        Some(Commands::Tx(tx)) => match tx {
            TxCommands::Add(tx) => add_transaction(tx),
        },
        None => {}
    }
}

fn main() {
    let cli = Cli::parse();
    parse(&cli.command)
}

#[test]
fn verify_app() {
    use clap::IntoApp;
    Cli::into_app().debug_assert()
}
