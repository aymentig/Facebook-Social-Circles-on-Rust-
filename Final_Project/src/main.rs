use petgraph::graph::UnGraph;
use petgraph::dot::{Dot, Config};
use petgraph::algo;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, Write}; 
use std::path::Path;

fn main() {
    // Create an undirected graph and a map to store node indices
    let mut graph = UnGraph::<usize, ()>::new_undirected();
    let mut node_indices = HashMap::new();

    // Read the edges file and build the graph
    if let Ok(lines) = read_lines("facebook/0.edges") {
        for line in lines {
            if let Ok(edge) = line {
                //here im looking through the edge information
                let nodes: Vec<&str> = edge.split_whitespace().collect();
                let node1 = nodes[0].parse::<usize>().unwrap();
                let node2 = nodes[1].parse::<usize>().unwrap();

                //i added node indices and edges to the graph
                let index1 = *node_indices.entry(node1).or_insert_with(|| graph.add_node(node1));
                let index2 = *node_indices.entry(node2).or_insert_with(|| graph.add_node(node2));

                graph.add_edge(index1, index2, ());
            }
        }
    }
// Generate the DOT representation of the graph
let dot_representation = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

// Write the DOT representation to a file
if let Ok(mut dot_file) = File::create("graph.dot") {
    if let Err(e) = dot_file.write_all(dot_representation.as_bytes()) {
        eprintln!("Error writing DOT file: {}", e);
    } else {
        println!("DOT file 'graph.dot' generated successfully.");
    }
} else {
    eprintln!("Error creating DOT file.");
}

// Use Graphviz to visualize the graph
if let Err(e) = std::process::Command::new("dot")
    .args(&["-Tpng", "graph.dot", "-o", "graph.png"])
    .output()
{
    eprintln!("Error executing Graphviz: {}", e);
} else {
    println!("Graph visualization 'graph.png' generated successfully.");
}
    //prints the graph in DOT format
    println!("Graph in DOT format:");
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    //detects communities in the dataset
    let communities = detect_communities(&graph);
    println!("Communities: {:?}", communities);

    //calculates the node degrees
    let node_degrees = calculate_node_degrees(&graph);
    println!("Node Degrees: {:?}", node_degrees);

    //counts triangles in the dataset
    let triangle_count = count_triangles(&graph);
    println!("Triangle Count: {:?}", triangle_count);

    //finds how many connected components in the dataset
    let connected_components = algo::connected_components(&graph);
    println!("Connected Components: {:?}", connected_components);

    // Calculate the density of the graph
    let graph_density = calculate_graph_density(&graph);
    println!("Graph Density: {:.4}", graph_density);

    // Calculate degree centrality for each node
    let centrality = calculate_degree_centrality(&graph);
    println!("Node Centrality: {:?}", centrality);

    // Additional analysis can be performed here
}

// Function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Function to detect communities in the graph
fn detect_communities(graph: &UnGraph<usize, ()>) -> HashMap<usize, usize> {
    let mut communities = HashMap::new();
    let mut current_community = 0;

    for node in graph.node_indices() {
        // If the node is not already assigned to a community, assign it to the current community
        if !communities.contains_key(&node.index()) {
            communities.insert(node.index(), current_community);
            // Update the community number for the next unassigned node
            current_community += 1;
        }
    }

    communities
}

// Function to calculate node degrees
fn calculate_node_degrees(graph: &UnGraph<usize, ()>) -> HashMap<usize, usize> {
    let mut degrees = HashMap::new();
    for node in graph.node_indices() {
        let degree = graph.neighbors(node).count();
        degrees.insert(node.index(), degree);
    }
    degrees
}

// Function to calculate the density of the graph
fn calculate_graph_density(graph: &UnGraph<usize, ()>) -> f64 {
    let node_count = graph.node_count() as f64;
    let edge_count = graph.edge_count() as f64;
    if node_count == 0.0 || node_count == 1.0 {
        0.0
    } else {
        (2.0 * edge_count) / (node_count * (node_count - 1.0))
    }
}

// Function to calculate degree centrality for each node
fn calculate_degree_centrality(graph: &UnGraph<usize, ()>) -> HashMap<usize, f64> {
    let node_count = graph.node_count() as f64;
    let mut centrality = HashMap::new();
    for node in graph.node_indices() {
        let degree = graph.neighbors(node).count() as f64;
        centrality.insert(node.index(), degree / (node_count - 1.0));
    }
    centrality
}

//the function of this is to count triangles in the dataset
fn count_triangles(graph: &UnGraph<usize, ()>) -> usize {
    let mut count = 0;
    for node in graph.node_indices() {
        let neighbors: HashSet<_> = graph.neighbors(node).collect();
        for neighbor in &neighbors {
            let common_neighbors: HashSet<_> = graph.neighbors(*neighbor).collect();
            let intersection: HashSet<_> = neighbors.intersection(&common_neighbors).collect();
            count += intersection.len();
        }
    }
    count / 2 //I divided by 2 to avoid double-counting
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_communities() {
        let mut graph = UnGraph::<usize, ()>::new_undirected();
        let mut node_indices = HashMap::new();

        //created a simple graph for testing
        let nodes = vec![1, 2, 3, 4];
        for node in &nodes {
            let index = *node_indices.entry(*node).or_insert_with(|| graph.add_node(*node));
        }

        //made a few edges to form communities
        graph.add_edge(node_indices[&1], node_indices[&2], ());
        graph.add_edge(node_indices[&2], node_indices[&3], ());
        graph.add_edge(node_indices[&3], node_indices[&4], ());

        let communities = detect_communities(&graph);

        //made it so nodes 1, 2, 3, and 4 are in the same community
        assert_eq!(communities[&0], 0);
        assert_eq!(communities[&1], 0);
        assert_eq!(communities[&2], 0);
        assert_eq!(communities[&3], 0);
    }

    #[test]
    fn test_node_degree_analysis() {
        let mut graph = UnGraph::<usize, ()>::new_undirected();
        let mut node_indices = HashMap::new();

        //i created a simple graph for testing
        let nodes = vec![1, 2, 3, 4];
        for node in &nodes {
            let index = *node_indices.entry(*node).or_insert_with(|| graph.add_node(*node));
        }

        //then i created a few edges to set degrees
        graph.add_edge(node_indices[&1], node_indices[&2], ());
        graph.add_edge(node_indices[&2], node_indices[&3], ());

        let node_degrees = calculate_node_degrees(&graph);

        // then set the degrees for nodes 1, 2, 3, and 4
        assert_eq!(node_degrees[&0], 1);
        assert_eq!(node_degrees[&1], 2);
        assert_eq!(node_degrees[&2], 2);
        assert_eq!(node_degrees[&3], 0);
    }

    #[test]
    fn test_connected_components_analysis() {
        let mut graph = UnGraph::<usize, ()>::new_undirected();
        let mut node_indices = HashMap::new();

        // Create two disconnected components
        let component1 = vec![1, 2, 3];
        let component2 = vec![4, 5, 6];

        for node in &component1 {
            let index = *node_indices.entry(*node).or_insert_with(|| graph.add_node(*node));
        }

        for node in &component2 {
            let index = *node_indices.entry(*node).or_insert_with(|| graph.add_node(*node));
        }

        let connected_components = algo::connected_components(&graph);

        // Count the number of connected components
        let num_components = *connected_components.iter().max().unwrap() + 1;

        // Assert that there are two connected components
        assert_eq!(num_components, 2);
    }
}
