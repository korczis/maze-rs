extern crate iron;
extern crate router;
extern crate time;
extern crate urlencoded;

use hyper::header::Headers;
use iron::prelude::*;
use iron::status;
use router::Router;
use time::PreciseTime;
use urlencoded::UrlEncodedQuery;

use super::types::cell::BaseCell;
use super::types::grid::Grid;

pub fn start_web<'a>(port: u16) {
    println!("Starting REST Interface on port {}.", port);

    let mut router = Router::new();
    router.get("", move |req: &mut Request| -> IronResult<Response> {
        let mut count: usize = 1;
        let mut height: usize = 5;
        let mut width: usize = 5;
        match req.get_ref::<UrlEncodedQuery>() {
            Ok(ref hashmap) => {
                if hashmap.contains_key("count") && hashmap["count"].len() == 1 {
                    match hashmap["count"][0].parse::<usize>() {
                        Ok(val) => count = val,
                        _ => {}
                    }
                }

                if hashmap.contains_key("height") && hashmap["height"].len() == 1 {
                    match hashmap["height"][0].parse::<usize>() {
                        Ok(val) => height = val,
                        _ => {}
                    }
                }

                if hashmap.contains_key("width") && hashmap["width"].len() == 1 {
                    match hashmap["width"][0].parse::<usize>() {
                        Ok(val) => width = val,
                        _ => {}
                    }
                }
            },
            Err(ref e) => println!("{:?}", e)
        };

        let mut res = String::new();

        let start = PreciseTime::now();
        for _ in 0..count {
            let mut grid: Grid<BaseCell> = Grid::new(width, height);
            grid.generate_sidewinder();

            res += &grid.to_string()[..];
            res += "\n";
        }
        let end = PreciseTime::now();

        let mut headers = Headers::new();
        let diff = start.to(end);
        let diff_string = format!("{}", diff.num_seconds() as f32 + diff.num_milliseconds() as f32 * 0.001 + diff.num_microseconds().unwrap() as f32 * 1e-6);
        headers.set_raw("x-time-sec", vec![diff_string.into_bytes()]);

        let mut response = Response::with((status::Ok, res));
        response.headers = headers;
        Ok(response)
    }, "get");

    let address = format!("0.0.0.0:{}", port);
    match Iron::new(router).http(&address[..]) {
        Ok(_res) => {
            // println!("{:?}", res);
        },
        Err(res) => {
            println!("{:?}", res);
        }
    }
}