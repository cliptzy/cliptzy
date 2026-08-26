pub struct FilterNode {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub input_pads: Vec<String>,
    pub output_pads: Vec<String>,
}

impl FilterNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            params: Vec::new(),
            input_pads: Vec::new(),
            output_pads: Vec::new(),
        }
    }

    pub fn param(mut self, key: &str, val: &str) -> Self {
        self.params.push((key.to_string(), val.to_string()));
        self
    }

    pub fn inputs(mut self, pads: &[&str]) -> Self {
        self.input_pads = pads.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn outputs(mut self, pads: &[&str]) -> Self {
        self.output_pads = pads.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for pad in &self.input_pads {
            s.push_str(&format!("[{}]", pad));
        }
        s.push_str(&self.name);

        if !self.params.is_empty() {
            s.push('=');
            let params_str: Vec<String> = self
                .params
                .iter()
                .map(|(k, v)| {
                    if k.is_empty() {
                        v.to_string() // Anonymous param
                    } else {
                        format!("{}={}", k, v)
                    }
                })
                .collect();
            s.push_str(&params_str.join(":"));
        }

        for pad in &self.output_pads {
            s.push_str(&format!("[{}]", pad));
        }
        s
    }
}

pub struct FilterGraph {
    pub nodes: Vec<FilterNode>,
}

impl FilterGraph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: FilterNode) {
        self.nodes.push(node);
    }

    pub fn to_string(&self) -> String {
        let node_strs: Vec<String> = self.nodes.iter().map(|n| n.to_string()).collect();
        node_strs.join(";")
    }
}

impl Default for FilterGraph {
    fn default() -> Self {
        Self::new()
    }
}
