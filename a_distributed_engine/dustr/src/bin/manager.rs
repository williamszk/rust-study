use dustr;

#[actix_web::main]
async fn main() {
    // TODO: figure out where to use those arguments from args
    // let _args = Cli::parse();
    dustr::core::manager::cli::deal_with_cli();

    // experiments
    // dustr::core::manager::experiments::_main_01().await;
    // dustr::core::manager::send_payload::experiments::_send_map_to_worker().await;
    dustr::core::manager::send_payload::experiments::_send_struct_to_worker().await;

    // Running manager node API server ========================================
    let res = dustr::core::manager::endpoints::create_server().await;
    println!("Result of creating server: {:?}", res);
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
