extern crate serde_json;

use serde_json::Map;
use serde_json::json;

use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

pub fn format<T>(grid: &Grid<T>) -> String
    where T: Cell + Clone
{
    let mut map: Map<String, serde_json::Value> = Map::new();
    let mut links: Vec<serde_json::value::Value> = Vec::new();

    // TODO A lot more of this can be moved to the json! syntax
    // Read: https://docs.serde.rs/serde_json/index.html

    for (k, set) in grid.links.iter() {
        for v in set.iter() {
            let mut first: Vec<serde_json::value::Value> = Vec::new();
            first.push(json!(k.0 as u64));
            first.push(json!(k.1 as u64));

            let mut second: Vec<serde_json::value::Value> = Vec::new();
            second.push(json!(v.0 as u64));
            second.push(json!(v.1 as u64));

            let mut tuple: Vec<serde_json::value::Value> = Vec::new();
            tuple.push(serde_json::value::Value::Array(first));
            tuple.push(serde_json::value::Value::Array(second));

            links.push(serde_json::value::Value::Array(tuple));
        }
    }

    map.insert("x".to_string(), json!(grid.x() as u64));
    map.insert("y".to_string(), json!(grid.y() as u64));
    map.insert("links".to_string(), serde_json::value::Value::Array(links));

    match serde_json::to_string(&map) {
        Ok(json) => {
            return json;
        },
        Err(_) => {
            return String::new()
        }
    }
}