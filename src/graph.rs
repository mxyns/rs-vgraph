use num_traits::PrimInt;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug)]
pub struct Graph<N, C, D> {
    nodes: HashSet<N>,
    links: HashMap<(N, N), (C, D)>,
}

pub type CostComputer<N, C, D> = fn(&Graph<N, C, D>, N, N) -> C;

impl<N, C, D> Default for Graph<N, C, D> {
    fn default() -> Self {
        Graph {
            nodes: HashSet::new(),
            links: HashMap::new(),
        }
    }
}

impl<N: Eq + Hash + Copy + Display, C: Hash + PrimInt, D> Graph<N, C, D> {
    fn check_node(&self, node: &N) -> Result<(), String> {
        if self.nodes.contains(node) {
            Ok(())
        } else {
            Err(format!("Node {} is not in the graph", node))
        }
    }

    pub fn add_node(&mut self, node: N) {
        self.nodes.insert(node);
    }

    pub fn add_link(&mut self, a: N, b: N, cost_a2b: C, data: D) -> Result<(), String> {
        self.check_node(&a)?;
        self.check_node(&b)?;

        self.links.insert((a, b), (cost_a2b, data));

        Ok(())
    }

    pub fn compute_path(
        &self,
        start: N,
        goal: N,
        bypass: Option<CostComputer<N, C, D>>,
    ) -> Result<Option<(Vec<N>, C)>, String> {
        self.check_node(&start)?;
        self.check_node(&goal)?;

        let result = if let Some(bypasser) = bypass {
            pathfinding::directed::dijkstra::dijkstra(
                &start,
                |v: &N| {
                    self.nodes
                        .iter()
                        .map(|n: &N| {
                            if let Some((c, _d)) = self.links.get(&(*v, *n)) {
                                (*n, *c)
                            } else {
                                let bypass_cost = bypasser(self, *v, *n);
                                (*n, bypass_cost)
                            }
                        })
                        .collect::<Vec<(N, C)>>()
                },
                |v| *v == goal,
            )
        } else {
            pathfinding::directed::dijkstra::dijkstra(
                &start,
                |v: &N| {
                    self.links
                        .iter()
                        .filter(|((a, _b), (_c, _d))| *a == *v)
                        .map(|((_a, b), (c, _d))| (*b, *c))
                        .collect::<Vec<(N, C)>>()
                },
                |v| *v == goal,
            )
        };

        Ok(result)
    }

    pub fn get_data(&self, from: &N, to: &N) -> Result<&D, String> {
        self.get_link(from, to).map(|(_, data)| data)
    }

    #[allow(dead_code)]
    pub fn get_cost(&self, from: &N, to: &N) -> Result<&C, String> {
        self.get_link(from, to).map(|(cost, _)| cost)
    }

    pub fn get_link(&self, from: &N, to: &N) -> Result<&(C, D), String> {
        self.check_node(from)?;
        self.check_node(to)?;

        if let Some(link) = self.links.get(&(*from, *to)) {
            Ok(link)
        } else {
            Err(format!("Link {} -> {} does not exist", from, to))
        }
    }
}
