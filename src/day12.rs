//! <https://adventofcode.com/2021/day/12>
//!
//! Part 2 takes a few seconds to run (without optimizations, with them it's OK).
//! Obvious improvements to increase performance would be:
//! * Don't produce a vector of all paths, just calculate the number of paths (as asked).
//! * Don't even have a vector of paths - one path is enough, it can be used globally and pushed to/
//!   popped from through the recursive calls. (But then you have a global mutable variable :/.)

use std::collections::HashMap;

const INPUT: [&str; 25] = [
    "fw-ll", "end-dy", "tx-fw", "tx-tr", "dy-jb", "ZD-dy", "dy-BL", "dy-tr", "dy-KX", "KX-start",
    "KX-tx", "fw-ZD", "tr-end", "fw-jb", "fw-yi", "ZD-nr", "start-fw", "tx-ll", "ll-jb", "yi-jb",
    "yi-ll", "yi-start", "ZD-end", "ZD-jb", "tx-ZD",
];

#[derive(Clone, Eq, PartialEq, Hash)]
struct Cave(String);

struct CaveGraph {
    adjacency_list: HashMap<Cave, Vec<Cave>>,
}

struct CavePath {
    path: Vec<Cave>,
    contains_one_small_cave_twice: bool,
}

impl Cave {
    fn is_small(&self) -> bool {
        self.0.chars().all(|c| c.is_lowercase())
    }
}

impl CavePath {
    fn clone_and_extend(
        &self,
        cave: Cave,
        contains_one_small_cave_twice: Option<bool>,
    ) -> CavePath {
        CavePath {
            path: [self.path.clone(), vec![cave]].concat(),
            contains_one_small_cave_twice: match contains_one_small_cave_twice {
                Some(b) => b,
                None => self.contains_one_small_cave_twice,
            },
        }
    }
}

impl CaveGraph {
    fn from_paths(paths: &[&str]) -> CaveGraph {
        let mut adjacency_list: HashMap<Cave, Vec<Cave>> = HashMap::new();
        for &path in paths {
            let path_split: Vec<&str> = path.split('-').collect();
            let cave_1 = Cave(String::from(path_split[0]));
            let cave_2 = Cave(String::from(path_split[1]));

            adjacency_list
                .entry(cave_1.clone())
                .or_insert_with(Vec::new)
                .push(cave_2.clone());
            adjacency_list
                .entry(cave_2)
                .or_insert_with(Vec::new)
                .push(cave_1);
        }
        CaveGraph { adjacency_list }
    }

    fn paths_through(&self, allow_one_small_cave_twice: bool) -> Vec<CavePath> {
        /// Given a valid path prefix, returns all valid complete paths that begin with this prefix.
        fn dfs_with_path(
            path_from_start: CavePath,
            graph: &CaveGraph,
            allow_one_small_cave_twice: bool,
        ) -> Vec<CavePath> {
            let current_cave = path_from_start.path.last().unwrap();
            if current_cave.0 == "end" {
                vec![path_from_start]
            } else {
                let mut paths: Vec<CavePath> = Vec::new();
                for successor in &graph.adjacency_list[current_cave] {
                    let mut paths_through_successor = {
                        if successor.is_small() && path_from_start.path.contains(successor) {
                            // We're about to visit a small cave that is already on the path:
                            if !allow_one_small_cave_twice {
                                // ...if `allow_one_small_cave_twice` is false, we can't.
                                vec![]
                            } else if successor.0 == "start" {
                                // ...even if `allow_one_small_cave_twice` is true, we can't visit
                                // the start cave twice.
                                vec![]
                            } else if path_from_start.contains_one_small_cave_twice {
                                // ...even if `allow_one_small_cave_twice` is true, we can't visit
                                // a small cave twice when we already did that earlier on the path.
                                vec![]
                            } else {
                                // ...if `allow_one_small_cave_twice` is true and we did not visit
                                // a small cave twice yet, we can do that now, marking resulting
                                // paths using the flag `contains_one_small_cave_twice`.
                                dfs_with_path(
                                    path_from_start.clone_and_extend(successor.clone(), Some(true)),
                                    graph,
                                    allow_one_small_cave_twice,
                                )
                            }
                        } else {
                            // No small cave shenanigans here.
                            dfs_with_path(
                                path_from_start.clone_and_extend(successor.clone(), None),
                                graph,
                                allow_one_small_cave_twice,
                            )
                        }
                    };
                    paths.append(&mut paths_through_successor);
                }
                paths
            }
        }

        dfs_with_path(
            CavePath {
                path: vec![Cave(String::from("start"))],
                contains_one_small_cave_twice: false,
            },
            self,
            allow_one_small_cave_twice,
        )
    }
}

pub fn solution_1() -> String {
    let cave_graph = CaveGraph::from_paths(&INPUT);
    cave_graph.paths_through(false).len().to_string()
}

pub fn solution_2() -> String {
    let cave_graph = CaveGraph::from_paths(&INPUT);
    cave_graph.paths_through(true).len().to_string()
}
