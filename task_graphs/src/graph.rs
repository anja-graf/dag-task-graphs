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
    pub lateness: i32,
    // Time between executions if task is repeated
    pub period: i32
}