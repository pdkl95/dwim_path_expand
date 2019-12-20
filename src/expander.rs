pub struct PathExpander {
    pub hidden: bool,
    pub maxdepth: u32,
}

impl PathExpander {
    pub fn new() -> PathExpander {
        PathExpander {
            hidden: false,
            maxdepth: 1
        }
    }

    pub fn expand_input_path(&self, input_path: &str) -> Vec<String> {
        let file = input_path.to_string();

        let mut expanded_paths: Vec<String> = Vec::new();
        
        expanded_paths.push(file.clone());

        return expanded_paths;
    }
}

