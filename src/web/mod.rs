extern crate iron;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedQuery;

use super::types::grid::Grid;

pub fn start_web<'a>(port: u16) {
    println!("Starting REST Interface on port {}.", port);

    let mut router = Router::new();
    router.get("", move |req: &mut Request| -> IronResult<Response> {
        let mut height: usize = 5;
        let mut width: usize = 5;
        match req.get_ref::<UrlEncodedQuery>() {
            Ok(ref hashmap) => {
                if hashmap["height"].len() == 1 {
                    match hashmap["height"][0].parse::<usize>() {
                        Ok(val) => height = val,
                        _ => {}
                    }
                }

                if hashmap["width"].len() == 1 {
                    match hashmap["width"][0].parse::<usize>() {
                        Ok(val) => width = val,
                        _ => {}
                    }
                }
            },
            Err(ref e) => println!("{:?}", e)
        };

        let mut grid = Grid::new(width, height);
        grid.generate_sidewinder();

        let response = Response::with((status::Ok, grid.to_string()));
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