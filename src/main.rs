#[macro_use]
extern crate lazy_static;

mod app;
mod args;
mod path_printer;
mod walker;

use args::Args;
use walker::Walker;

fn main() {
    let args = Args::parse();

    Walker::new(&args).walk_and_print();
}
