use std::collections::{HashSet, VecDeque, HashMap};
use super::graph::Graph;

pub fn dfs<F>(graph: &Graph, start: &str, mut visit: F)
where
    F: FnMut(&str),
{
    let mut visited = HashSet::new();
    dfs_recursive(graph, start, &mut visited, &mut visit);
}

fn dfs_recursive<F>(graph: &Graph, vertex: &str, visited: &mut HashSet<String>, visit: &mut F)
where
    F: FnMut(&str),
{
    if visited.insert(vertex.to_string()) {
        visit(vertex);
        if let Some(neighbors) = graph.get_neighbors(vertex) {
            for edge in neighbors {
                dfs_recursive(graph, &edge.to, visited, visit);
            }
        }
    }
}

pub fn bfs<F>(graph: &Graph, start: &str, mut visit: F)
where
    F: FnMut(&str),
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start.to_string());
    queue.push_back(start.to_string());

    while let Some(vertex) = queue.pop_front() {
        visit(&vertex);

        if let Some(neighbors) = graph.get_neighbors(&vertex) {
            for edge in neighbors {
                if visited.insert(edge.to.clone()) {
                    queue.push_back(edge.to.clone());
                }
            }
        }
    }
}

pub fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, f64> {
    let mut distances = HashMap::new();
    let mut unvisited = graph.vertices.keys().cloned().collect::<HashSet<_>>();

    for vertex in graph.vertices.keys() {
        distances.insert(vertex.clone(), f64::INFINITY);
    }
    distances.insert(start.to_string(), 0.0);

    while let Some(current) = unvisited.iter()
        .min_by(|a, b| distances[*a].partial_cmp(&distances[*b]).unwrap())
        .cloned() 
    {
        unvisited.remove(&current);

        if let Some(neighbors) = graph.get_neighbors(&current) {
            for edge in neighbors {
                let distance = distances[&current] + edge.weight;
                if distance < distances[&edge.to] {
                    distances.insert(edge.to.clone(), distance);
                }
            }
        }
    }

    distances
}