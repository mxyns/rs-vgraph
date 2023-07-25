mod graph;
mod converter;
mod bypass_heuristics;

use graph::Graph;
use crate::converter::{ConversionGraph, ConverterFunction, Version, Versioned};

fn fn_1_to_10(file: &mut u32) {
    *file = 10;
}

fn fn_10_to_1(file: &mut u32) {
    *file = 1;
}

fn fn_10_to_18(file: &mut u32) {
    *file = 18;
}

impl Versioned for u32 {
    fn version(&self) -> Version {
        return *self;
    }
}

fn main() {
    let mut g: ConversionGraph<u32> = Graph::default();
    vec![1, 10, 15, 18].iter().for_each(|v| g.add_node(*v));
    g.add_link(1, 10, 1, converter!(fn_1_to_10)).unwrap();
    g.add_link(10, 1, 1, converter!(fn_10_to_1)).unwrap();
    g.add_link(10, 18, 1, converter!(fn_10_to_18)).unwrap();

    let start = 1;
    let goal = 18;
    let result = g.compute_path(start, goal, None).unwrap();

    println!("{:#?}", g);
    println!("{} -> {} = {:#?}", start, goal, &result);

    if let Some((path, cost)) = result {
        converter::print_result(&g, start, goal, path, cost);
    } else {
        println!("Trying bypass");
        let result = g.compute_path(start, goal, Some(bypass_heuristics::version_diff)).unwrap();

        if let Some((path, cost)) = result {
            println!("{:#?}", path);
            converter::print_result(&g, start, goal, path, cost)
        } else {
            println!("Could not find a path even with bypassing");
        }
    }

    let mut file = 1;
    let result = g.convert(&mut file, 18);
    println!("{:#?}, {:#?}", result, file);
}
