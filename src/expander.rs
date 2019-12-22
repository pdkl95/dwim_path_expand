use std::collections::HashSet;
use std::path::Path;

pub struct PathExpander {
    filter_ext: bool,
    pub included_ext: HashSet<String>,
    pub excluded_ext: HashSet<String>,
    pub hidden: bool,
    pub maxdepth: u32,
}

impl PathExpander {
    pub fn new() -> PathExpander {
        PathExpander {
            filter_ext: false,
            included_ext: HashSet::new(),
            excluded_ext: HashSet::new(),
            hidden: false,
            maxdepth: 1
        }
    }

    pub fn include_ext(&mut self, ext: &str) {
        self.filter_ext = true;
        self.included_ext.insert(ext.to_string());
    }

    pub fn exclude_ext(&mut self, ext: &str) {
        self.filter_ext = true;
        self.excluded_ext.insert(ext.to_string());
    }

    fn is_matching_ext(&self, ext: &str) -> bool {
        if !self.filter_ext {
            return true;
        }

        if self.excluded_ext.contains(ext) {
            return false;
        }

        if self.included_ext.contains(ext) {
            return true;
        }

        return false;
    }

    fn is_matching_file(&self, path: &Path) -> bool {
        let stem = path.file_stem().expect("Path.file_atem call failed")
            .to_str().expect("OsStr.to_str call failed");

        if stem.is_empty() {
            return false;
        }

        if self.hidden && stem.starts_with(".") {
            return false;
        }

        match path.extension() {
            Some(ext_osstr) => {
                let ext = ext_osstr.to_str().expect("OsStr.to_str call failed");
                return self.is_matching_ext(ext);
            },
            None => {
                return false;
            },
        }
    }

    fn expand(&self, expanded_paths: &mut Vec<String>, path: &Path, depth: u32) {
        //println!("expand(_, {:?}, {})", path, depth);

        if depth > self.maxdepth {
            //println!("DEPTH({}) > MAXDEPTH({})", depth, self.maxdepth);
            return;
        }

        if path.exists() {
            let md = path.metadata().expect("metadata call failed");

            if md.is_file() {
                if self.is_matching_file(path) {
                    let file_path = path.to_str().expect("to_str call failed");
                    expanded_paths.push(file_path.to_string());
                }
            } else if md.is_dir() {
                let newdepth = depth + 1;
                for entry in path.read_dir().expect("read_dir call failed") {
                    match entry {
                        Ok(e) => {
                            let entpath = e.path();
                            //println!("dirent: {:?}", entpath);
                            self.expand(expanded_paths, &entpath, newdepth);
                        },
                        _ => { }
                    }
                }
            } else {
                // not a file or dir - ignore
            }
        } else {
            // not in filesystem
        }
    }

    pub fn expand_input_path(&self, input_path: &str) -> Vec<String> {
        let mut expanded_paths: Vec<String> = Vec::new();

        let path = Path::new(input_path);
        println!("*** EXPANDING: {}", path.display());

        self.expand(&mut expanded_paths, &path, 0);

        for expath in &expanded_paths {
            println!("  - {}", expath);
        }

        return expanded_paths;
    }
}

