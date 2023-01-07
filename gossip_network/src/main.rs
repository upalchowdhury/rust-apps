use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct GossipNetwork {
    nodes: HashMap<String, Arc<Mutex<Node>>>,
}

struct Node {
    name: String,
    gossip: Vec<String>,
    connections: Vec<Arc<Mutex<Node>>>,
}

impl GossipNetwork {
    fn new() -> Self {
        GossipNetwork {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, name: &str) {
        let node = Arc::new(Mutex::new(Node {
            name: name.to_string(),
            gossip: Vec::new(),
            connections: Vec::new(),
        }));
        self.nodes.insert(name.to_string(), node);
    }

    fn add_connection(&mut self, a: &str, b: &str) {
        let a_node = self.nodes.get(a).unwrap();
        let b_node = self.nodes.get(b).unwrap();
        a_node.lock().unwrap().connections.push(b_node.clone());
        b_node.lock().unwrap().connections.push(a_node.clone());
    }

    fn spread_gossip(&self, source: &str, message: &str) {
        let source_node = self.nodes.get(source).unwrap();
        source_node.lock().unwrap().gossip.push(message.to_string());
        for connection in source_node.lock().unwrap().connections.iter() {
            connection.lock().unwrap().gossip.push(message.to_string());
        }
    }
}

fn main() {
    let mut network = GossipNetwork::new();

    // Add some nodes to the network
    network.add_node("Alice");
    network.add_node("Bob");
    network.add_node("Eve");
    network.add_node("Mallory");

    // Connect the nodes in a loop
    network.add_connection("Alice", "Bob");
    network.add_connection("Bob", "Eve");
    network.add_connection("Eve", "Mallory");
    network.add_connection("Mallory", "Alice");

    // Spread some gossip
    network.spread_gossip("Alice", "I heard that Eve is planning a surprise party!");
}
