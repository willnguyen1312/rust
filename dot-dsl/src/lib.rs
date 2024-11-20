use std::collections::HashMap;
type Attrs = HashMap<String, String>;

pub mod graph {
    pub mod graph_items {
        pub mod edge {
            use crate::Attrs;
            use crate::HashMap;

            #[derive(PartialEq, Eq, Clone, Debug)]
            pub struct Edge {
                pub start: String,
                pub end: String,
                pub attrs: Attrs,
            }

            impl Edge {
                pub fn new(start: &str, end: &str) -> Self {
                    return Self {
                        start: start.to_string(),
                        end: end.to_string(),
                        attrs: HashMap::new(),
                    };
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    let mut map: Attrs = HashMap::new();
                    for &(k, v) in attrs.iter() {
                        map.insert(k.to_string(), v.to_string());
                    }
                    self.attrs = map;
                    return self;
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    return self.attrs.get(attr).map(|v| v.as_str());
                }
            }
        }
        pub mod node {
            use crate::Attrs;
            use crate::HashMap;

            #[derive(PartialEq, Eq, Clone, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: Attrs,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    return Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    };
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    let mut map: Attrs = HashMap::new();
                    for &(k, v) in attrs.iter() {
                        map.insert(k.to_string(), v.to_string());
                    }
                    self.attrs = map;
                    return self;
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    return self.attrs.get(attr).map(|v| v.as_str());
                }
            }
        }
    }
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use crate::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            return Self {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            };
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.to_vec();
            return self;
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.to_vec();
            return self;
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            let mut map: HashMap<String, String> = HashMap::new();
            for &(k, v) in attrs.iter() {
                map.insert(k.to_string(), v.to_string());
            }
            self.attrs = map;
            return self;
        }

        pub fn node(&self, node: &str) -> Option<&Node> {
            return self.nodes.iter().find(|n| n.name == node);
        }
    }
}
