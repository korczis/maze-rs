#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap;
extern crate css_color_parser;
extern crate maze;
extern crate serde_json;
extern crate time;

use clap::{App, Arg};
use css_color_parser::Color as CssColor;

use maze::distance;
use maze::types::cell::BaseCell;
use maze::types::grid::Grid;
use maze::web;

use std::env;
use std::process::exit;
use std::str::FromStr;

const AUTHOR: &'static str = "Tomas Korcak <korczis@gmail.com>";
const DESCRIPTION: &'static str = "Maze Generator";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const DEFAULT_CELL_SIZE: u32 = 80;
const DEFAULT_WALL_SIZE: u32 = 20;
const DEFAULT_HEIGHT: usize = 5;
const DEFAULT_WIDTH: usize = 5;
const DEFAULT_PORT: u16 = 5000;
const DEFAULT_COLOR_CELL: [u8; 3] = [255, 255, 255];
const DEFAULT_COLOR_WALL: [u8; 3] = [0, 0, 0];

enum Algorithm {
    AldousBroder,
    Binary,
    Sidewinder,
    Wilson
}

impl FromStr for Algorithm {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "aldous-broder" => Ok(Algorithm::AldousBroder),
            "binary" => Ok(Algorithm::Binary),
            "sidewinder" => Ok(Algorithm::Sidewinder),
            "wilson" => Ok(Algorithm::Wilson),
            _ => Err("no match")
        }
    }
}

enum Format {
    Ascii,
    Json,
    Png
}

impl FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ascii" => Ok(Format::Ascii),
            "json" => Ok(Format::Json),
            "png" => Ok(Format::Png),
            _ => Err("no match")
        }
    }
}

fn main() {
    let default_cell_size = &DEFAULT_CELL_SIZE.to_string()[..];
    let default_wall_size = &DEFAULT_WALL_SIZE.to_string()[..];
    let default_height = &DEFAULT_HEIGHT.to_string()[..];
    let default_width = &DEFAULT_WIDTH.to_string()[..];
    let default_port = &DEFAULT_PORT.to_string()[..];

    let matches = App::new(DESCRIPTION)
        .version(VERSION)
        .author(AUTHOR)
        .arg(Arg::with_name("algorithm")
            .help("Algorithm to use")
            .short("a")
            .long("algorithm")
            .possible_values(&["aldous-broder", "binary", "sidewinder", "wilson"])
            .default_value("aldous-broder")
        )
        .arg(Arg::with_name("cell-size")
            .help("Size of Cell")
            .short("c")
            .long("cell-size")
            .default_value(default_cell_size)
        )
        .arg(Arg::with_name("color-cell")
            .help("Color of Cell")
            .long("color-cell")
            .default_value("#fff")
        )
        .arg(Arg::with_name("color-wall")
            .help("Color of Wall")
            .long("color-wall")
            .default_value("#000")
        )
        .arg(Arg::with_name("wall-size")
            .help("Size of Wall")
            .short("w")
            .long("wall-size")
            .default_value(default_wall_size)
        )
        .arg(Arg::with_name("format")
            .help("Output format to use")
            .short("f")
            .long("format")
            .possible_values(&["ascii", "json", "png"])
            .default_value("ascii")
        )
        .arg(Arg::with_name("height")
            .help("Height of Maze")
            .short("y")
            .long("height")
            .default_value(default_height)
        )
        .arg(Arg::with_name("width")
            .help("Width of Maze")
            .short("x")
            .long("width")
            .default_value(default_width)
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
            .default_value(default_port)
        )
        .get_matches();

    match matches.occurrences_of("verbose") {
        1 => env::set_var("RUST_LOG", "warn"),
        2 => env::set_var("RUST_LOG", "info"),
        3 => env::set_var("RUST_LOG", "debug"),
        _ => {}
    }

    env_logger::init().unwrap();

    let port: u16 = match matches.value_of("rest-port").unwrap().to_string().parse::<u16>() {
        Ok(val) => val,
        _ => DEFAULT_PORT
    };

    if matches.is_present("rest") {
        web::start_web(port);
        exit(0);
    }

    let height = match matches.value_of("height").unwrap().to_string().parse::<usize>() {
        Ok(val) => val,
        _ => DEFAULT_HEIGHT
    };

    let width = match matches.value_of("width").unwrap().to_string().parse::<usize>() {
        Ok(val) => val,
        _ => DEFAULT_WIDTH
    };

    let cell_size = match matches.value_of("cell-size").unwrap().to_string().parse::<u32>() {
        Ok(val) => val,
        _ => DEFAULT_CELL_SIZE
    };

    let wall_size = match matches.value_of("wall-size").unwrap().to_string().parse::<u32>() {
        Ok(val) => val,
        _ => DEFAULT_WALL_SIZE
    };

    let color_cell = match matches.value_of("color-cell").unwrap().parse::<CssColor>() {
        Ok(val) => [val.r, val.g, val.b],
        _ => DEFAULT_COLOR_CELL
    };

    let color_wall = match matches.value_of("color-wall").unwrap().parse::<CssColor>() {
        Ok(val) => [val.r, val.g, val.b],
        _ => DEFAULT_COLOR_WALL
    };

    let algorithm = Algorithm::from_str(matches.value_of("algorithm").unwrap());

    let mut grid: Grid<BaseCell> = Grid::new(width, height);
    match algorithm {
        Ok(Algorithm::AldousBroder) => {
            info!("Generating maze using Aldous-Broder algorithm");
            grid.generate_aldous_broder()
        },
        Ok(Algorithm::Binary) => {
            info!("Generating maze using Binary algorithm");
            grid.generate_binary()
        },
        Ok(Algorithm::Sidewinder) => {
            info!("Generating maze using Sidewinder algorithm");
            grid.generate_sidewinder()
        },
        Ok(Algorithm::Wilson) => {
            info!("Generating maze using Wilson's algorithm");
            grid.generate_wilson()
        },
        Err(_) => {
            info!("Invalid algorithm specified");
            exit(1);
        }
    }

    let format = Format::from_str(matches.value_of("format").unwrap());
    match format {
        Ok(Format::Ascii) => grid.print_ascii(),
        Ok(Format::Json) => grid.print_json(),
        Ok(Format::Png) => {
            let output_filename = "output.png";
            info!("Writing maze to {:?}", output_filename);
            grid.to_png(cell_size, wall_size, &color_cell, &color_wall, output_filename);
        },
        Err(_) => {
            println!("Invalid format specified");
            exit(1);
        }
    }

    let distances = distance::dijkstra::calculate(&grid);
    distances.print_ascii();
}
