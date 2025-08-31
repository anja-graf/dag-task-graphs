use crate::graph::Graph;
use crate::graph::Node;
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::collections::HashSet;

impl Graph {
    /// Adds n new nodes to the given vector and sets for each node a number as name and
    /// default values for all other node parameters
    fn initialize_nodes(nodes: &mut Vec<Node>, n: i32) {
        let mut rng = rand::thread_rng();
        for node in 0..n {
            nodes.push(Node {
                name: node,
                start_time: 0,
                abs_deadline: 0,
                computation_time: rng.gen_range(10..=100),
                arrival_time: 0,
                rel_deadline: 0,
                finishing_time: 0,
                response_time: 0,
                lateness: 0,
                period: 0,
            });
        }

        // The period is set to sum of all computation times
        let period: i32 = nodes.iter().map(|node| node.computation_time).sum();
        for node in nodes.iter_mut() {
            node.period = period;
            node.abs_deadline = period;
            node.rel_deadline = node.abs_deadline - node.arrival_time;
        }
    }

    /// Generates a directed acyclic graph with n nodes where each edge is included with independent probability p
    pub fn new_erdos_renyi_var1(n: i32, p: f64) -> Self {
        assert!(
            0.0 <= p && p <= 1.0,
            "The probability has to be between 0 and 1!"
        );
        let mut edges = Vec::new();
        let mut nodes: Vec<Node> = Vec::new();
        let mut rng = rand::thread_rng();
        // In this adjacency list we store a list of reachable nodes from a key node
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();

        Self::initialize_nodes(&mut nodes, n);

        let mut vec1: Vec<i32> = (0..n).collect();
        let mut vec2: Vec<i32> = (0..n).collect();
        vec1.shuffle(&mut rng);
        vec2.shuffle(&mut rng);

        for i in &vec1 {
            for j in &vec2 {
                // Here we add each possible edge with probability p

                // Cycles would lead to non executable task arrangement
                // That is why we don't want to add edges that would close up a cycle
                // There should not be a path from j to i if we add the edge i to j
                if *i != *j && !(edges.contains(&(*i, *j)) || edges.contains(&(*j, *i))) {
                    if rng.r#gen::<f64>() < p && !Graph::has_path(&adj, *j, *i, &mut HashSet::new())
                    {
                        edges.push((*i, *j));
                        adj.entry(*i).or_default().push(*j);
                    }
                }
            }
        }

        Graph { nodes, edges }
    }

    /// Generates a directed acyclic graph with n nodes and m edges chosen uniformly at random
    pub fn new_erdos_renyi_var2(n: i32, m: i32) -> Self {
        // In directed graphes we can have twice the amount of edges in comparison to undirected graphes,
        // BUT as soon as we have more than an undirected complete graph there has to be a cycle,
        // see also https://de.wikipedia.org/wiki/Vollst%C3%A4ndiger_Graph
        assert!(
            m <= (n * (n - 1)) / 2,
            "The number of edges is higher than possible!"
        );
        let mut edges = Vec::new();
        let mut nodes: Vec<Node> = Vec::new();
        let mut rng = rand::thread_rng();
        // In this adjacency list we store a list of reachable nodes from a key node
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();

        Self::initialize_nodes(&mut nodes, n);
        let mut counter = 0; // Current number of edges in graph
        while counter < m {
            let i = rng.gen_range(0..n);
            let j = rng.gen_range(0..n);
            //println!("{:?}",edges);
            if i != j && !(edges.contains(&(i, j)) || edges.contains(&(j, i))) {
                // Cycles would lead to non executable task arrangement
                // That is why we don't want to add edges that would close up a cycle
                // There should not be a path from j to i if we add the edge i to j
                if !Graph::has_path(&adj, j, i, &mut HashSet::new()) {
                    edges.push((i, j));
                    adj.entry(i).or_default().push(j);
                    counter += 1;
                }
            }
        }
        assert!(m == edges.len() as i32);
        Graph { nodes, edges }
    }

    /// Checks if there is a path from source to target and returns corresponding boolean
    /// As Input it takes an adjacency list of the graph and
    /// a visited HashSet that should be empty at the beginning
    fn has_path(
        adj: &HashMap<i32, Vec<i32>>,
        source: i32,
        target: i32,
        visited: &mut HashSet<i32>,
    ) -> bool {
        if source == target {
            // Source is target so we have a (trivial) path
            return true;
        }
        if visited.contains(&source) {
            // We already visited our source which would lead to a cycle
            return false;
        }
        visited.insert(source); // Add current source node, so we won't check it again
        if let Some(neighbors) = adj.get(&source) {
            for &next in neighbors {
                // Check for each neighbor if we can reach target node
                if Graph::has_path(adj, next, target, visited) {
                    return true;
                }
            }
        }
        false // There is no path between source and target
    }
}
