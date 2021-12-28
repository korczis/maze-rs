#![feature(test)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate  ansi_term;
extern crate css_color_parser;
extern crate hyper;
extern crate image;
extern crate imageproc;
extern crate iron;
extern crate rand;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate svg;
extern crate test;
extern crate time;
extern crate urlencoded;

pub mod distance;
pub mod generator;
pub mod output;
pub mod types;
pub mod web;

