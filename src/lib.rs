#![feature(test)]
#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate hyper;
extern crate iron;
extern crate rand;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate test;
extern crate time;
extern crate urlencoded;

pub mod types;
pub mod web;

