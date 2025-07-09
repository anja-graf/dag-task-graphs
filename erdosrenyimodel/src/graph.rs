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
    // Time at which task starts execution
    pub start_time: i32,
    // Latest acceptable completion time for a task
    pub abs_deadline: i32,
    // Time required to compute a task, independently to when a task starts or ends
    pub computation_time: i32,
    // Time at which task becomes ready for execution and could possibly be executed
    pub arrival_time: i32,
    // Relative Deadline = abs_deadline - arrival_time
    pub rel_deadline: i32,
    // Time the task execution ends
    pub finishing_time: i32,
    // Response Time = finishing_time - arrival_time
    pub response_time: i32,
    // Lateness = finishing_time - abs_deadline
    pub lateness: i32
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
                abs_deadline: 100,
                computation_time: rng.gen_range(10..=100),
                arrival_time: 0,
                rel_deadline: 100,
                finishing_time: 100,
                response_time: 100,
                lateness: 0
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
        assert!(m == edges.len() as i32);
        Graph { nodes, edges }
    }

    /// Brings all tasks in feasible, sequential order and sets its parameters 
    /// If multiple tasks can be executed at one point, 
    /// the one with higher priority (number attribute) is taken.
    pub fn distribute_parameters_uniprocessor(&mut self) -> Vec<i32> {
        let schedule = self.schedule_tasks();

        // the deadline will be the start time of the next node in the schedule
        // the first task can begin immediately 
        let mut pre_time = 0;

        for i in 0..schedule.len() { 
            let node = self.find_node_by_name(schedule[i]);
            // Set start time and deadline
            node.start_time = pre_time;
            node.abs_deadline = node.start_time + node.computation_time;
            node.rel_deadline = node.abs_deadline - node.arrival_time;
            node.finishing_time = node.abs_deadline;
            node.response_time = node.finishing_time - node.arrival_time;
            node.lateness = node.finishing_time - node.abs_deadline;
            pre_time = node.abs_deadline;
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

    /// Calls uunifast algorithm and assigns new computation times
    pub fn distribute_uunifast(&mut self,u:f64) {
        // Slide 7 on VL04_SchedulingPeriodic
        // U_i = C_i / T_i and sum of all U_i must be <= 1 (else not schedulable)
        let utilization:Vec<f64> = self.uunifast(u);
        assert!(utilization.len() == self.nodes.len());
        for i in 0..self.nodes.len() {
            self.nodes[i].computation_time = 
            (self.nodes[i].abs_deadline as f64 * utilization[i]).round() as i32;

            if self.nodes[i].computation_time == 0 {
                println!("[WARNING] Computation time of task {} was set to 0!", self.nodes[i].name);
            } else if self.nodes[i].computation_time > self.nodes[i].abs_deadline + self.nodes[i].start_time {
                println!("[ERROR] Computation time of task {} is too high to be executable before its deadline!", self.nodes[i].name);
            }

        }
    }

    /// UuniFast algorithm like described in IEEE paper https://ieeexplore.ieee.org/abstract/document/1311021
    /// Page 6, upper right before '4. Simulation results'
    /// To efficiently generate task sets with uniform distribution and with O(n) complexity
    fn uunifast(&self,u:f64) -> Vec<f64> {
        assert!(u <= 1.0 && u >= 0.0,"The processor utilization should be percentage between 0 and 1");
        let mut sum_u = u;
        let mut rng = rand::thread_rng();
        let mut vect_u:Vec<f64> = Vec::new();
        for i in 0..self.nodes.len() - 1 {
            let next_sum_u = sum_u * rng.r#gen::<f64>().powf(1.0/(self.nodes.len() - i) as f64);
            vect_u.push(sum_u - next_sum_u);
            sum_u = next_sum_u;
        }
        vect_u.push(sum_u); 
        println!("{:?}",vect_u);
        assert!(u - vect_u.iter().sum::<f64>().abs() < 1e-6); // floating point arithmetic
        vect_u
    }
}
