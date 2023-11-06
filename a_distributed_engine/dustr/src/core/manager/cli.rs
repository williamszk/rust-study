use clap::Parser;

pub fn deal_with_cli() {
    let _args = Cli::parse();
    println!("_args: {:?}", _args);
}

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
