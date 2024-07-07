use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Edge {
    pub to: String,
    pub relationship: String,
    pub weight: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    pub vertices: HashMap<String, Vec<Edge>>,
    pub directed: bool,
}

impl Graph {
    pub fn new(directed: bool) -> Self {
        Graph {
            vertices: HashMap::new(),
            directed,
        }
    }

    pub fn add_vertex(&mut self, vertex: &str) {
        self.vertices.entry(vertex.to_string()).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, from: &str, to: &str, relationship: &str, weight: f64) {
        self.vertices.entry(from.to_string()).or_insert(Vec::new()).push(Edge {
            to: to.to_string(),
            relationship: relationship.to_string(),
            weight,
        });
        if !self.directed {
            self.vertices.entry(to.to_string()).or_insert(Vec::new()).push(Edge {
                to: from.to_string(),
                relationship: relationship.to_string(),
                weight,
            });
        }
        self.vertices.entry(to.to_string()).or_insert(Vec::new());
    }

    pub fn get_neighbors(&self, vertex: &str) -> Option<&Vec<Edge>> {
        self.vertices.get(vertex)
    }

    pub fn get_neighbors_by_relationship(&self, vertex: &str, relationship: &str) -> Vec<&str> {
        self.vertices.get(vertex)
            .map(|edges| edges.iter()
                .filter(|edge| edge.relationship == relationship)
                .map(|edge| edge.to.as_str())
                .collect())
            .unwrap_or_else(Vec::new)
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let json = serde_json::to_string(self)?;
        fs::write(filename, json)
    }

    pub fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let json = fs::read_to_string(filename)?;
        let graph: Graph = serde_json::from_str(&json)?;
        Ok(graph)
    }
}