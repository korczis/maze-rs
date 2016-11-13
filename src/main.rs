#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;
extern crate maze;
extern crate serde_json;

use clap::{App, Arg};
use maze::types::cell::BaseCell;
use maze::types::grid::Grid;
use maze::web;

use std::env;
use std::process::exit;
use std::str::FromStr;

const AUTHOR: &'static str = "Tomas Korcak <korczis@gmail.com>";
const DESCRIPTION: &'static str = "Maze Generator";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

enum Algorithm {
    Binary,
    Sidewinder
}

impl FromStr for Algorithm {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "binary" => Ok(Algorithm::Binary),
            "sidewinder" => Ok(Algorithm::Sidewinder),
            _ => Err("no match")
        }
    }
}

enum Format {
    Ascii,
    Json
}

impl FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ascii" => Ok(Format::Ascii),
            "json" => Ok(Format::Json),
            _ => Err("no match")
        }
    }
}

fn main() {
    let matches = App::new(DESCRIPTION)
        .version(VERSION)
        .author(AUTHOR)
        .arg(Arg::with_name("algorithm")
            .help("Algorithm to use")
            .short("a")
            .long("algorithm")
            .possible_values(&["binary", "sidewinder"])
            .default_value("sidewinder")
        )
        .arg(Arg::with_name("format")
            .help("Output format to use")
            .short("f")
            .long("format")
            .possible_values(&["ascii", "json"])
            .default_value("ascii")
        )
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
        .arg(Arg::with_name("rest")
            .help("Run REST Server")
            .short("r")
            .long("rest")
        )
        .arg(Arg::with_name("rest-port")
            .help("REST Port")
            .short("p")
            .long("rest-port")
            .default_value("5000")
        )
        .get_matches();

    match matches.occurrences_of("verbose") {
        1 => env::set_var("RUST_LOG", "warn"),
        2 => env::set_var("RUST_LOG", "info"),
        3 => env::set_var("RUST_LOG", "debug"),
        _ => {}
    }

    env_logger::init().unwrap();

    let port: u16 = matches.value_of("rest-port").unwrap().to_string().parse::<u16>().unwrap();
    if matches.is_present("rest") {
        web::start_web(port);
        exit(0);
    }

    let height = matches.value_of("height").unwrap().to_string().parse::<usize>().unwrap();
    let width = matches.value_of("width").unwrap().to_string().parse::<usize>().unwrap();

    let algorithm = Algorithm::from_str(matches.value_of("algorithm").unwrap());

    let mut grid: Grid<BaseCell> = Grid::new(width, height);
    match algorithm {
        Ok(Algorithm::Binary) => grid.generate_binary(),
        Ok(Algorithm::Sidewinder) => grid.generate_sidewinder(),
        Err(_) => {
            println!("Invalid algorithm specified");
            exit(1);
        }
    }

    debug!("{:?}", grid);

    let format = Format::from_str(matches.value_of("format").unwrap());
    match format {
        Ok(Format::Ascii) => grid.print_ascii(),
        Ok(Format::Json) => grid.print_json(),
        Err(_) => {
            println!("Invalid format specified");
            exit(1);
        }
    }
}
