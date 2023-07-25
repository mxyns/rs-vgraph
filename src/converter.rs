use std::hash::{Hash, Hasher};
use crate::bypass_heuristics;
use crate::graph::Graph;

pub type Version = u32;

#[derive(Debug)]
pub struct ConverterFunction<F> {
    pub fname: &'static str,
    pub f: fn(&mut F)
}

#[macro_export]
macro_rules! converter {
    ($f: expr) => {
        {
            ConverterFunction { f: $f, fname: stringify!($f)}
        }
    }
}

pub trait Versioned {
    fn version(&self) -> Version;
}

pub type Cost = u32;
pub type ConversionGraph<F> = Graph<Version, Cost, ConverterFunction<F>>;

impl<F: Versioned> ConversionGraph<F> {
    pub fn convert(&self, file: &mut F, to: Version) -> Result<(), String> {
        let from = file.version();
        return if let Some((path, _cost)) = self.compute_path(from, to, None)? {
            for (idx, step) in path.iter().enumerate() {
                let prev = if idx == 0 { from } else { *path.get(idx - 1).unwrap() };
                if prev == *step {
                    continue
                }

                let converter = self.get_data(&prev, &step)?;
                (converter.f)(file);
            }

            Ok(())
        } else {
            Err(format!("Could not apply conversion {} -> {}", from, to))
        }
    }
}

pub fn print_result<F>(graph: &ConversionGraph<F>, start: Version, goal: Version, path: Vec<Version>, path_cost: Cost) {
    let len = path.len() - 1;
    println!("Converting from v{start} to v{goal} has {len} step(s) and costs {path_cost}:");
    print!("[{start}]");

    let mut bypass_count = 0;
    for (idx, intermediate) in path.iter().enumerate() {
        let prev = if idx == 0 { start } else { *path.get(idx - 1).unwrap() };
        if *intermediate == prev { continue }

        let link = graph.get_link(&prev, intermediate);
        if let Ok((step_cost, conv)) = link {
            print!("{}{{c={step_cost}}} -> [{}]", conv.fname, intermediate);
        } else if *intermediate != start {
            bypass_count += 1;
            let bypass_cost = bypass_heuristics::version_diff(graph, prev, *intermediate);
            print!("__bypass#{bypass_count}__{{c={bypass_cost}}} -> [{}]", intermediate)
        }

        if idx + 1 != path.len() {
            print!(" -> ");
        }
    }
    println!();
}
