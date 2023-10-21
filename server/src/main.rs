mod config;
mod log;

fn main() {
    let config = config::Config::load(std::path::Path::new("./config.json"));
    println!("{}", config.debug);
}
