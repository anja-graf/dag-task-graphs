use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
//use std::fs::read;
use rand::seq::SliceRandom;

pub struct Graph {
    // List of all nodes in graph
    pub nodes: Vec<Node>,
    // List of all edges in graph (tuples where both numbers are names of nodes)
    pub edges: Vec<(i32, i32)>,
}

pub struct Node {
    // Name identifies each node and defines priority
    pub name: i32,
    // Time at which task becomes ready for execution and could possibly be executed
    //pub arrival_time: i32,
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
        let mut rng = rand::thread_rng();
        for node in 0..n {
            nodes.push(Node {
                name: node,
                start_time: 0,
                //arrival_time: 0,
                deadline: 100,
                computation_time: rng.gen_range(10..=100),
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
                    if rng.r#gen::<f64>() < p && !Graph::has_path(&adj, *j, *i, &mut HashSet::new()) {
                        edges.push((*i, *j));
                        adj.entry(*i).or_default().push(*j);
                    }
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
    // pub fn distribute_parameters(&mut self) {
    //     let sorted = self.topological_sort();
    //     for node_name in sorted {
    //         let latest_time = self.edges
    //             .iter()
    //             .filter(|(_, target)| *target == node_name)
    //             .map(|(source, _)| {
    //                 let src = &self.nodes[*source as usize];
    //                 src.start_time + src.computation_time // Time the task is finished
    //             })
    //             .max() // After the latest task we can start our current one
    //             .unwrap_or(0); // If there is no previous task we can start immediately

    //         // Find current node in nodes list
    //         let node = self.nodes
    //             .iter_mut()
    //             .find(|n| n.name == node_name)
    //             .unwrap();

    //         // Set start time and deadline
    //         node.start_time = latest_time;
    //         node.deadline = latest_time + node.computation_time;
    //     }
    // }

    /// Brings all tasks in feasible, sequential order and sets its parameters 
    /// If multiple tasks can be executed at one point, 
    /// the one with higher priority (number attribute) is taken.
    pub fn distribute_parameters_single_core(&mut self) -> Vec<i32> {
        let schedule = self.schedule_tasks();
        // find first executed task and set its deadline 
        let first = self.find_node_by_name(schedule[0]);
        first.deadline = first.start_time + first.computation_time;

        // the deadline will be the start time of the next node in the schedule
        let mut pre_time = first.deadline;

        for i in 1..schedule.len() { 
            let node = self.find_node_by_name(schedule[i]);
            // Set start time and deadline
            node.start_time = pre_time;
            node.deadline = node.start_time + node.computation_time;
            pre_time = node.deadline;
        }
        schedule
    }

    /// Gets a mutable node object by its number 
    fn find_node_by_name(&mut self,name:i32) -> &mut Node {
        self.nodes
            .iter_mut()
            .find(|n| n.name == name)
            .unwrap()
    }

    /// Finds a successively task arrangement, that does not allow parallel computation of tasks
    /// and returns an ordered list of the node numbers. If multiple tasks can be executed at one 
    /// point, the one with higher priority (number attribute) is taken.
    fn schedule_tasks(&self) -> Vec<i32> {
        // list of tasks in planned final order
        let mut schedule:Vec<i32> = Vec::new();
        // list of tasks that have no predecessor or all predecessor have been executed already
        let mut ready:HashSet<i32> = HashSet::new();
        // In this predecessor list we store all previous tasks that have to be executed before a key node
        let mut pre: HashMap<i32, HashSet<i32>> = HashMap::new();
        self.get_predecessor(&mut pre);

        // for key in pre.clone() {
        //     println!("{:?}",key);
        // }

        // Adding initial nodes to ready list
        // that affects tasks that have no previous tasks
        for (node, previous_keys) in &pre {
            if previous_keys.is_empty() {
                ready.insert(*node);
            }
        }

        if ready.is_empty() { // This should never happen as we ensured there are no cycles
            panic!("This task arrangement is not feasible! No node has no predecessor!");
        }

        while !ready.is_empty() {
            let highest_priority = ready.iter().min().cloned().unwrap(); // highest priority has minimum number
            
            // this task will now be included in schedule
            ready.remove(&highest_priority);
            pre.remove(&highest_priority);
            schedule.push(highest_priority);

            // create a copy of predecessor list and adapt it to updated schedule
            let mut new_pre: HashMap<i32, HashSet<i32>> = HashMap::new();
            for (node, predecessor) in &pre {
                let mut value = predecessor.clone();
                value.remove(&highest_priority);
                if value.len() == 0 { // this task can now also be executed, it has no more previous tasks it has to wait for
                    ready.insert(*node);
                }
                new_pre.insert(*node, value);
            }
            pre = new_pre;
        }
        //println!("Schedule: {:?}",schedule);
        schedule
    }

    /// Writes for every node (key) a list of predecessor nodes (value) in given pre hash map
    fn get_predecessor(&self,pre:&mut HashMap<i32,HashSet<i32>>) {
        // initialise empty entry for all nodes
        for node in &self.nodes {
            pre.insert(node.name, HashSet::new());
        }

        // add all predecessor for every node
        for &(from, to) in &self.edges {
            pre.entry(to).or_default().insert(from);
        }
    }

    /// Sorts nodes based on dependencies and returns an ordered list
    /// In this list a node comes always before all of its dependencies
    // fn topological_sort(&self) -> Vec<i32> {
    //     let mut visited = HashSet::new();
    //     let mut result = Vec::new();

    //     /// Visits every node and saves order of dependencies in result
    //     fn dfs(
    //         node: i32,
    //         visited: &mut HashSet<i32>,
    //         result: &mut Vec<i32>,
    //         edges: &Vec<(i32, i32)>
    //     ) {
    //         // We only want to proceed if the node is not newly inserted
    //         if !visited.insert(node) {
    //             return;
    //         }
    //         // Then, every node that is a following node to the current node is visited
    //         for (_, following_node) in edges.iter().filter(|(src, _)| *src == node) {
    //             dfs(*following_node, visited, result, edges);
    //         }
    //         result.push(node); // All dependent nodes are before the current one
    //     }

    //     for node in &self.nodes {
    //         dfs(node.name, &mut visited, &mut result, &self.edges);
    //     }

    //     result.reverse(); // Now the result is in topological order
    //     result
    // }

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
