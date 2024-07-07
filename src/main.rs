mod graph;
mod algorithms;

use graph::Graph;
use algorithms::{dfs, bfs, dijkstra};

fn main() {
    // Create a directed graph
    println!("Creating a directed graph...");
    let mut directed_graph = Graph::new(true);

    // Add vertices
    directed_graph.add_vertex("A");
    directed_graph.add_vertex("B");
    directed_graph.add_vertex("C");
    directed_graph.add_vertex("D");

    // Add edges with weights
    directed_graph.add_edge("A", "B", "connects", 4.0);
    directed_graph.add_edge("B", "C", "connects", 3.0);
    directed_graph.add_edge("C", "D", "connects", 2.0);
    directed_graph.add_edge("A", "D", "connects", 10.0);

    // Test get_neighbors
    println!("\nNeighbors of A:");
    if let Some(neighbors) = directed_graph.get_neighbors("A") {
        for edge in neighbors {
            println!("  To: {}, Relationship: {}, Weight: {}", edge.to, edge.relationship, edge.weight);
        }
    }

    // Test get_neighbors_by_relationship
    println!("\nNeighbors of A with 'connects' relationship:");
    let connects_neighbors = directed_graph.get_neighbors_by_relationship("A", "connects");
    println!("{:?}", connects_neighbors);

    // Perform DFS
    println!("\nDFS traversal:");
    dfs(&directed_graph, "A", &mut |vertex: &str| print!("{} ", vertex));
    println!();

    // Perform BFS
    println!("\nBFS traversal:");
    bfs(&directed_graph, "A", &mut |vertex: &str| print!("{} ", vertex));
    println!();

    // Find shortest paths using Dijkstra's algorithm
    println!("\nShortest paths from A:");
    let distances = dijkstra(&directed_graph, "A");
    for (vertex, distance) in distances {
        println!("To {}: {}", vertex, distance);
    }

    // Save and load the graph
    directed_graph.save_to_file("directed_graph.json").expect("Failed to save directed graph");
    let loaded_directed_graph = Graph::load_from_file("directed_graph.json").expect("Failed to load directed graph");
    println!("\nLoaded directed graph: {:?}", loaded_directed_graph);

    // Create an undirected graph
    println!("\nCreating an undirected graph...");
    let mut undirected_graph = Graph::new(false);

    // Add vertices and edges
    undirected_graph.add_vertex("X");
    undirected_graph.add_vertex("Y");
    undirected_graph.add_vertex("Z");
    undirected_graph.add_edge("X", "Y", "friends", 1.0);
    undirected_graph.add_edge("Y", "Z", "friends", 1.0);

    // Test get_neighbors for undirected graph
    println!("\nNeighbors of Y in undirected graph:");
    if let Some(neighbors) = undirected_graph.get_neighbors("Y") {
        for edge in neighbors {
            println!("  To: {}, Relationship: {}, Weight: {}", edge.to, edge.relationship, edge.weight);
        }
    }

    // Save and load the undirected graph
    undirected_graph.save_to_file("undirected_graph.json").expect("Failed to save undirected graph");
    let loaded_undirected_graph = Graph::load_from_file("undirected_graph.json").expect("Failed to load undirected graph");
    println!("\nLoaded undirected graph: {:?}", loaded_undirected_graph);

}