extern crate rand;

use rand::Rng;
// use std::collections::BTreeSet;

use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

pub fn generate<T>(grid: &mut Grid<T>)
    where T: Cell + Clone
{
    let mut unvisited = Vec::new();
    for x in 0..grid.x() {
        for y in 0..grid.y() {
            unvisited.push((x, y));
        }
    }

    let first = rand::thread_rng().choose(&unvisited).unwrap().clone();
    unvisited.retain(|&x| x != first);
    println!("Starting cell: {:?}", first);

    while unvisited.len() > 0 {
        let mut cell: (usize, usize) = rand::thread_rng().choose(&unvisited).unwrap().clone();
        let mut path: Vec<(usize, usize)> = Vec::new();

        println!("Adding {:?}", cell);
        path.push(cell);

        while unvisited.contains(&cell) {
            let neighbors = grid.neighbors_indices(cell.0, cell.1);

            let r = rand::thread_rng().choose(&neighbors).unwrap();
            cell = (r.x(), r.y());

            println!("Looking for: {:?}", cell);
            match path.binary_search(&cell) {
                Ok(position) => {
                    path = path[0..position].to_vec();
                    println!("Found, position: {:?}, new path: {:?}", position, path);
                }
                _ => {
                    path.push(cell);
                    println!("Not found, new path {:?}", path);
                }
            }
        }

        println!("Unvisited does not contains: {:?}", cell);
        println!("Path: {:?}", path);

        // return;

        // println!("{:?}", path.len());
        let len = path.len() - 2;
        for i in 0..len {
            let a = path[i];
            let b = path[i + 1];
            grid.link_indices(a.0, a.1, b.0, b.1);
            unvisited.retain(|&x| x != a);
        }

        grid.print_ascii();
    }
}