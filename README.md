## Project Overview

The core of the project involves constructing an undirected graph from a dataset of Facebook connections, represented as edges between nodes (users). This is achieved by parsing an edge list from the file `"facebook/0.edges"`. To facilitate a more tangible understanding of the network's structure, the program generates a DOT file of the graph, which is further visualized as a PNG image using Graphviz.

## Analytical Components

### Community Detection
A fundamental part of the analysis is identifying communities within the network. Currently, the implementation is rudimentary, placing each node in a separate community, but it lays the groundwork for integrating more advanced community detection algorithms in the future.

### Node Degree Analysis
The program calculates the degree of each node, providing insights into the most connected individuals in the network.

### Graph Density
Computing the graph's density offers a perspective on how interconnected the network is compared to a complete graph.

### Degree Centrality
The degree centrality measure for each node is calculated, highlighting the importance or influence of individual nodes.

### Triangle Count
Counting the number of triangles in the network can reveal the presence of tightly knit groups.

### Connected Components Analysis
The program determines the number of connected components, shedding light on the network's segmentation.

## Technical Details

### File Operations
Rust's standard library is employed for reading the edge data and writing the DOT file.

### Graph Data Structure
The `petgraph` library's `UnGraph` is used, suitable for representing undirected graphs.

### Error Handling
The code includes robust error handling for file operations and the execution of Graphviz for visualization.

## Objectives and Applications

The primary objective of this project is to provide a foundational tool for social network analysis. It is especially pertinent for researchers and analysts interested in exploring the structural properties of social networks, identifying key influencers, or detecting community clusters for targeted studies. 

## Write - UP

In my write-up, I basically go over the reasoning behing what i did and analyze the outputs and what they mean in terms of the dataset.
