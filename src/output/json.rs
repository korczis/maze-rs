extern crate serde_json;

use serde_json::Map;

use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

pub fn format<T>(grid: &Grid<T>) -> String
    where T: Cell + Clone
{
    let mut map: Map<String, serde_json::Value> = Map::new();
    let mut links: Vec<serde_json::value::Value> = Vec::new();

    for (k, set) in grid.links.iter() {
        for v in set.iter() {
            let mut tuple: Vec<serde_json::value::Value> = Vec::new();
            tuple.push(serde_json::value::Value::U64(k.0 as u64));
            tuple.push(serde_json::value::Value::U64(k.1 as u64));

            tuple.push(serde_json::value::Value::U64(v.0 as u64));
            tuple.push(serde_json::value::Value::U64(v.1 as u64));
            links.push(serde_json::value::Value::Array(tuple));
        }
    }

    map.insert("x".to_string(), serde_json::value::Value::U64(grid.x() as u64));
    map.insert("y".to_string(), serde_json::value::Value::U64(grid.y() as u64));
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