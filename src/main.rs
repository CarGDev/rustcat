mod cat_core;
mod config;
mod help;
mod options;

fn main() {
    let config = options::evaluate::read();
    cat_core::runner::exec(config);
}
