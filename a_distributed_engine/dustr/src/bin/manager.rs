use clap::Parser;
use dustr;

#[actix_web::main]
async fn main() {
    // TODO: figure out where to use those arguments from args
    // let _args = Cli::parse();
    // println!("_args: {:?}", _args);
    deal_with_cli();

    // experiments
    dustr::core::manager::experiments::main_01().await;

    // Running manager node API server ========================================
    let res = dustr::core::manager::endpoints::create_server().await;
    println!("{:?}", res);
}

fn deal_with_cli() {
    let _args = Cli::parse();
    println!("_args: {:?}", _args);
}

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_01() {
        assert!(true);
    }

    #[test]
    fn test_02() {
        let is_this_test_02 = true;
        assert!(is_this_test_02);
    }
}
