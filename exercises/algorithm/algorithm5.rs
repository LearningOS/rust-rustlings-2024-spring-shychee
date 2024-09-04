/*
	bfs
	This problem requires you to implement a basic BFS algorithm
    广度优先搜索
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    // 在图上执行 bfs 广度优先搜索，返回访问节点的顺序
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // 首先定义了一个布尔类型的向量visited来记录每个节点是否被访问过。
        let mut visited = vec![false; self.adj.len()];
        // 创建一个空的向量visit_order来存储遍历的顺序。
        let mut visit_order = vec![];
        // 创建一个队列queue，用于存储待访问的节点。
        let mut queue = VecDeque::new();

        // Mark the start node as visited and enqueue it
        // 将起始节点标记为已访问，并将其放入队列中。
        visited[start] = true;
        queue.push_back(start);

        // 当队列不为空时，循环执行以下操作：
        while!queue.is_empty() {
            // Dequeue a vertex from queue and print it
            // 从队列中取出一个节点（当前节点）。
            let current = queue.pop_front().unwrap();
            // 将当前节点加入visit_order中。
            visit_order.push(current);

            // Get all adjacent vertices of the dequeued vertex current.
            // If an adjacent has not been visited, then mark it visited and enqueue it
            // 遍历当前节点的所有邻接节点，如果邻接节点未被访问过，则将其标记为已访问，并放入队列中。
            for &neighbor in &self.adj[current] {
                if!visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }
        // 最终返回visit_order，即遍历的顺序。
        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

