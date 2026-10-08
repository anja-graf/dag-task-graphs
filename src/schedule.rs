use crate::graph::Graph;
use crate::graph::Node;
use std::collections::HashMap;
use std::collections::HashSet;

impl Graph {
    /// Brings all tasks in feasible, sequential order and sets its parameters
    /// If multiple tasks can be executed at one point,
    /// the one with higher priority (number attribute) is taken.
    ///
    /// If utilization is given, it will be distributed among the nodes of the graph
    /// For this the computation time of each task i is set to utilization[i] * period
    /// and finishing time, response time and lateness are set accordingly.
    /// If now the computation time is 0 or higher than 100, a warning or error is printed out
    pub fn distribute_parameters_uniprocessor(&mut self, utilization: &Vec<f64>) -> Vec<i32> {
        let schedule = self.schedule_tasks();

        // the finishing time will be the start time of the next node in the schedule
        // the first task can begin immediately
        let mut pre_time = 0;

        for i in 0..schedule.len() {
            let node = self.find_node_by_name(schedule[i]);
            if !utilization.is_empty() {
                // This ensures utilitaion is distributed correctly
                node.computation_time =
                    (node.period as f64 * utilization[node.name as usize]).round() as i32;
            }
            // Set start time and deadline
            node.start_time = pre_time;
            node.abs_deadline = node.period;
            node.rel_deadline = node.abs_deadline - node.arrival_time;
            node.finishing_time = node.start_time + node.computation_time;
            node.response_time = node.finishing_time - node.arrival_time;
            node.lateness = node.finishing_time - node.abs_deadline;
            pre_time = node.finishing_time;

            if node.computation_time > 100 || node.computation_time < 10 {
                println!(
                    "[WARNING] Computation time of task {} was set to {}!",
                    node.name, node.computation_time
                );
            }
            if node.abs_deadline < node.computation_time + node.start_time {
                println!(
                    "[ERROR] Computation time of task {} is too high to be executable before its deadline!",
                    node.name
                );
            }
        }
        schedule
    }

    /// Gets a mutable node object by its number
    fn find_node_by_name(&mut self, name: i32) -> &mut Node {
        self.nodes.iter_mut().find(|n| n.name == name).unwrap()
    }

    /// Finds a successively task arrangement, that does not allow parallel computation of tasks
    /// and returns an ordered list of the node numbers. If multiple tasks can be executed at one
    /// point, the one with higher priority (number attribute) is taken.
    fn schedule_tasks(&self) -> Vec<i32> {
        // list of tasks in planned final order
        let mut schedule: Vec<i32> = Vec::new();
        // list of tasks that have no predecessor or all predecessor have been executed already
        let mut ready: HashSet<i32> = HashSet::new();
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

        if ready.is_empty() {
            // This should never happen as we ensured there are no cycles
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
                if value.len() == 0 {
                    // this task can now also be executed, it has no more previous tasks it has to wait for
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
    fn get_predecessor(&self, pre: &mut HashMap<i32, HashSet<i32>>) {
        // initialise empty entry for all nodes
        for node in &self.nodes {
            pre.insert(node.name, HashSet::new());
        }

        // add all predecessor for every node
        for &(from, to) in &self.edges {
            pre.entry(to).or_default().insert(from);
        }
    }
}
