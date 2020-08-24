use std::collections::HashMap;

#[derive(PartialEq, Eq, Default, Clone, Debug)]
pub struct Edge {
    start: String,
    end: String,
    attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(s: &str, e: &str) -> Self {
        Edge {
            start: s.to_string(),
            end: e.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Edge {
        Edge {
            attrs: attrs
                .iter()
                .map(|(name, value)| (String::from(*name), String::from(*value)))
                .collect(),
            ..self
        }
    }

    pub fn get_attr(&self, name: &str) -> Option<&str> {
        self.attrs.get(name).map(|v| v.as_str())
    }
}
#[derive(PartialEq, Eq, Default, Clone, Debug)]
pub struct Node {
    name: String,
    attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Node {
        Node {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Node {
        Node {
            attrs: attrs
                .iter()
                .map(|(name, value)| (String::from(*name), String::from(*value)))
                .collect(),
            ..self
        }
    }

    pub fn get_attr(&self, name: &str) -> Option<&str> {
        self.attrs.get(name).map(|v| v.as_str())
    }
}
#[derive(Debug, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_nodes(self, nodes: &[Node]) -> Self {
        Graph {
            nodes: Vec::from(nodes),
            ..self
        }
    }

    pub fn with_edges(self, edges: &[Edge]) -> Self {
        Graph {
            edges: Vec::from(edges),
            ..self
        }
    }

    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Graph {
        Graph {
            attrs: attrs
                .iter()
                .map(|(name, value)| (String::from(*name), String::from(*value)))
                .collect(),
            ..self
        }
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| name == n.name)
    }

    pub fn get_attr(&self, name: &str) -> Option<&str> {
        self.attrs.get(name).map(|v| v.as_str())
    }
}

pub mod graph {
    pub use super::Graph;
    pub mod graph_items {
        pub mod edge {
            pub use super::super::super::Edge;
        }
        pub mod node {
            pub use super::super::super::Node;
        }
    }
}
