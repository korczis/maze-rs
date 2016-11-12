#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;
extern crate maze;

use clap::{App, Arg};
use maze::types::grid::Grid;

use std::env;

const AUTHOR: &'static str = "Tomas Korcak <korczis@gmail.com>";
const DESCRIPTION: &'static str = "Maze Generator";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new(DESCRIPTION)
        .version(VERSION)
        .author(AUTHOR)
        .arg(Arg::with_name("height")
            .help("Height of Maze")
            .short("y")
            .long("height")
            .default_value("5")
        )
        .arg(Arg::with_name("width")
            .help("Width of Maze")
            .short("x")
            .long("width")
            .default_value("5")
        )
        .arg(Arg::with_name("verbose")
            .help("Verbose mode")
            .short("v")
            .long("verbose")
            .multiple(true)
        )
        .get_matches();

    match matches.occurrences_of("verbose") {
        1 => env::set_var("RUST_LOG", "warn"),
        2 => env::set_var("RUST_LOG", "info"),
        3 => env::set_var("RUST_LOG", "debug"),
        _ => {}
    }

    env_logger::init().unwrap();

    let height = matches.value_of("height").unwrap().to_string().parse::<usize>().unwrap();
    let width = matches.value_of("width").unwrap().to_string().parse::<usize>().unwrap();

    let mut grid = Grid::new(width, height);

    grid.generate();
    debug!("{:?}", grid);

    grid.draw_ascii();
}
