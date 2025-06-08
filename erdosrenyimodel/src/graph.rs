use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Graph {
    // List of all nodes in graph
    pub nodes: Vec<Node>,
    // List of all edges in graph (tuples where both numbers are names of nodes)
    pub edges: Vec<(i32, i32)>,
}

pub struct Node {
    // Name identifies each node
    pub name: i32,
    // Time at which task becomes ready for execution and could possibly be executed
    pub arrival_time: i32,
    // Time at which task starts execution
    pub start_time: i32,
    // Latest acceptable completion time for a task
    pub deadline: i32,
    // Time required to compute a task, independently to when a task starts or ends
    pub computation_time: i32,
}

impl Graph {
    /// Adds n new nodes to the given vector and sets for each node a number as name and
    /// default values for start_time, arrival_time, deadline, computation_time
    fn initialize_nodes(nodes: &mut Vec<Node>, n: i32) {
        for node in 0..n {
            nodes.push(Node {
                name: node,
                start_time: 0,
                arrival_time: 0,
                deadline: 100,
                computation_time: 100,
            });
        }
    }

    /// Generates a graph with n nodes where each edge is included with independent probability p
    pub fn new_erdos_renyi_var1(n: i32, p: f64) -> Self {
        assert!(0.0 <= p && p <= 1.0, "The probability has to be between 0 and 1!");
        let mut edges = Vec::new();
        let mut nodes: Vec<Node> = Vec::new();
        let mut rng = rand::thread_rng();
        // In this adjacency list we store a list of reachable nodes from a key node
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();

        Self::initialize_nodes(&mut nodes, n);

        for i in 0..n {
            for j in 0..n {
                // Here we add each possible edge with probability p

                // Cycles would lead to non executable task arrangement
                // That is why we don't want to add edges that would close up a cycle
                // There should not be a path from j to i if we add the edge i to j
                if rng.r#gen::<f64>() < p && !Graph::has_path(&adj, j, i, &mut HashSet::new()) {
                    edges.push((i, j));
                    adj.entry(i).or_default().push(j);
                }
            }
        }

        Graph { nodes, edges }
    }

    /// Generates a graph with n nodes and m edges chosen uniformly at random
    pub fn new_erdos_renyi_var2(n: i32, m: i32) -> Self {
        // In directed graphes we can have twice the amount of edges in comparison to undirected graphes,
        // BUT as soon as we have more than an undirected complete graph there has to be a cycle,
        // see also https://de.wikipedia.org/wiki/Vollst%C3%A4ndiger_Graph
        assert!(m <= (n * (n - 1)) / 2, "The number of edges is higher than possible!");
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

        Graph { nodes, edges }
    }

    /// Distributes useful values for each node like deadline and results in a feasible task graph
    pub fn distribute_parameters(&mut self) {
        let sorted = self.topological_sort();
        for node_name in sorted {
            let latest_time = self.edges
                .iter()
                .filter(|(_, target)| *target == node_name)
                .map(|(source, _)| {
                    let src = &self.nodes[*source as usize];
                    src.start_time + src.computation_time // Time the task is finished
                })
                .max() // After the latest task we can start our current one
                .unwrap_or(0); // If there is no previous task we can start immediately

            // Find current node in nodes list
            let node = self.nodes
                .iter_mut()
                .find(|n| n.name == node_name)
                .unwrap();

            // Set start time and deadline
            node.start_time = latest_time;
            node.deadline = latest_time + node.computation_time;
        }
    }

    /// Sorts nodes based on dependencies and returns an ordered list
    fn topological_sort(&self) -> Vec<i32> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();

        /// Visits every node and saves order of dependencies in result
        fn dfs(
            node: i32,
            visited: &mut HashSet<i32>,
            result: &mut Vec<i32>,
            edges: &Vec<(i32, i32)>
        ) {
            // We only want to proceed if the node is not newly inserted
            if !visited.insert(node) {
                return;
            }
            // Then, every node that is a following node to the current node is visited
            for (_, following_node) in edges.iter().filter(|(src, _)| *src == node) {
                dfs(*following_node, visited, result, edges);
            }
            result.push(node); // All dependent nodes are before the current one
        }

        for node in &self.nodes {
            dfs(node.name, &mut visited, &mut result, &self.edges);
        }

        result.reverse(); // Now the result is in topological order
        result
    }

    /// Checks if there is a path from source to target and returns corresponding boolean
    /// As Input it takes an adjacency list of the graph and
    /// a visited HashSet that should be empty at the beginning
    fn has_path(
        adj: &HashMap<i32, Vec<i32>>,
        source: i32,
        target: i32,
        visited: &mut HashSet<i32>
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
