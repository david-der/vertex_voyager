use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Edge {
    to: String,
    relationship: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Graph {
    vertices: HashMap<String, Vec<Edge>>,
}

#[derive(Debug, Error)]
enum GraphError {
    #[error("Failed to read from file")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse JSON")]
    ParseError(#[from] serde_json::Error),
}

impl Graph {
    fn new() -> Self {
        Graph {
            vertices: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: &str) {
        self.vertices.entry(vertex.to_string()).or_insert(Vec::new());
    }

    fn add_edge(&mut self, from: &str, to: &str, relationship: &str) {
        self.vertices.entry(from.to_string()).or_insert(Vec::new()).push(Edge {
            to: to.to_string(),
            relationship: relationship.to_string(),
        });
        self.vertices.entry(to.to_string()).or_insert(Vec::new());
    }

    fn get_neighbors(&self, vertex: &str) -> Option<&Vec<Edge>> {
        self.vertices.get(vertex)
    }

    fn get_neighbors_by_relationship(&self, vertex: &str, relationship: &str) -> Vec<&str> {
        self.vertices.get(vertex)
            .map(|edges| edges.iter()
                .filter(|edge| edge.relationship == relationship)
                .map(|edge| edge.to.as_str())
                .collect())
            .unwrap_or_else(Vec::new)
    }

    fn save_to_file(&self, filename: &str) -> Result<(), GraphError> {
        let json = serde_json::to_string(self)?;
        fs::write(filename, json)?;
        Ok(())
    }

    fn load_from_file(filename: &str) -> Result<Self, GraphError> {
        let json = fs::read_to_string(filename)?;
        let graph: Graph = serde_json::from_str(&json)?;
        Ok(graph)
    }
}

fn main() {
    let mut graph = Graph::new();

    // Add vertices
    graph.add_vertex("Abby");
    graph.add_vertex("Ben");
    graph.add_vertex("Charlie");

    // Add edges with relationships
    graph.add_edge("Abby", "Ben", "mother");
    graph.add_edge("Ben", "Charlie", "father");
    graph.add_edge("Abby", "Charlie", "grandmother");

    // Print the graph
    println!("Graph: {:?}", graph);

    // Get all neighbors of Abby
    if let Some(neighbors) = graph.get_neighbors("Abby") {
        println!("All relationships of Abby: {:?}", neighbors);
    }

    // Get "mother" relationships of Abby
    let mother_relationships = graph.get_neighbors_by_relationship("Abby", "mother");
    println!("Mother relationships of Abby: {:?}", mother_relationships);

    // Save the graph to a file
    graph.save_to_file("graph.json").expect("Failed to save graph");

    // Load the graph from the file
    let loaded_graph = Graph::load_from_file("graph.json").expect("Failed to load graph");
    println!("Loaded graph: {:?}", loaded_graph);
}
