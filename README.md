# VertexVoyager: A Rust Graph Database Implementation

VertexVoyager is a simple, in-memory graph database implemented in Rust. It supports both directed and undirected graphs, weighted edges, and includes implementations of common graph algorithms.

## Features

- Support for both directed and undirected graphs
- Weighted edges
- Named relationships between vertices
- Basic graph operations (add vertex, add edge, get neighbors)
- Graph traversal algorithms (Depth-First Search, Breadth-First Search)
- Shortest path algorithm (Dijkstra's algorithm)
- Graph persistence (save to and load from JSON files)

## Project Structure

The project is organized into three main Rust files:

1. `src/main.rs`: Contains the main function demonstrating the usage of the graph database.
2. `src/graph.rs`: Defines the core `Graph` and `Edge` structures and their methods.
3. `src/algorithms.rs`: Implements graph algorithms (DFS, BFS, Dijkstra's).

## Core Components

### Graph Structure

The `Graph` struct is defined in `src/graph.rs`:

```rust
pub struct Graph {
    pub vertices: HashMap<String, Vec<Edge>>,
    pub directed: bool,
}
```

- vertices: A HashMap where keys are vertex names and values are vectors of edges.
- directed: A boolean indicating whether the graph is directed or undirected.

## Edge Structure
The Edge struct represents connections between vertices:

```
pub struct Edge {
    pub to: String,
    pub relationship: String,
    pub weight: f64,
}
```

- to: The name of the vertex this edge points to.
- relationship: A string describing the relationship between vertices.
- weight: A floating-point value representing the edge's weight.

## Key Functions
### Graph Operations

- Graph::new(directed: bool): Creates a new graph.
- add_vertex(&mut self, vertex: &str): Adds a new vertex to the graph.
- add_edge(&mut self, from: &str, to: &str, relationship: &str, weight: f64): Adds an edge between two vertices.
- get_neighbors(&self, vertex: &str): Returns all neighbors of a given vertex.
- get_neighbors_by_relationship(&self, vertex: &str,  relationship: &str): Returns neighbors connected by a specific relationship.

### Graph Algorithms

- dfs(graph: &Graph, start: &str, visit: F): Performs a depth-first search traversal.
- bfs(graph: &Graph, start: &str, visit: F): Performs a breadth-first search traversal.
- dijkstra(graph: &Graph, start: &str): Computes shortest paths from a starting vertex to all other vertices.

### Persistence

- save_to_file(&self, filename: &str): Saves the graph to a JSON file.
- load_from_file(filename: &str): Loads a graph from a JSON file.

## Usage Example
Here's a basic example of how to use VertexVoyager:

```
let mut graph = Graph::new(true);  // Create a directed graph

graph.add_vertex("A");
graph.add_vertex("B");
graph.add_edge("A", "B", "connects", 4.0);

if let Some(neighbors) = graph.get_neighbors("A") {
    for edge in neighbors {
        println!("A connects to {} with weight {}", edge.to, edge.weight);
    }
}

dfs(&graph, "A", &mut |vertex| println!("Visited: {}", vertex));

graph.save_to_file("my_graph.json").expect("Failed to save graph");
```

## Running the Project
To run the project:

- Ensure you have Rust installed on your system.
- Clone the repository.
- Navigate to the project directory.
- Run `cargo build` to compile the project.
- Run `cargo run` to execute the main function, which demonstrates various features of the graph database.

## Future Improvements

- Implement more advanced graph algorithms (e.g., A*, Bellman-Ford).
- Add support for property graphs (vertices and edges with arbitrary properties).
- Implement a query language for more complex graph operations.
- Add concurrency support for parallel graph operations.
- Implement a persistent storage solution for larger graphs.

## Contributing
Contributions to VertexVoyager are welcome! Please feel free to submit pull requests, create issues, or suggest new features.
## License
This project is open-source and available under the MIT License.