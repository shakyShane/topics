use topics_core::from_cli;

fn main() {
    // std::env::set_var("RUST_LOG", "topics=trace");
    env_logger::init();
    std::process::exit(match from_cli() {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
