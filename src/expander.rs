use glob_match::glob_match;

use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::env::current_dir;

use walkdir::WalkDir;

pub struct PathExpander {
    filter_ext: bool,
    pub extra_suffix: HashSet<String>,
    pub included_ext: HashSet<String>,
    pub excluded_ext: HashSet<String>,
    pub show_hidden: bool,
    pub match_prefix: bool,
    pub maxdepth: u32,
}

macro_rules! add_ext_method {
    ($var:ident, $method:ident) => {
        pub fn $method(&mut self, ext: &str) {
            self.filter_ext = true;
            self.$var.insert(ext.to_string());
        }
    };
}

impl PathExpander {
    pub fn new() -> PathExpander {
        PathExpander {
            filter_ext: false,
            extra_suffix: HashSet::new(),
            included_ext: HashSet::new(),
            excluded_ext: HashSet::new(),
            show_hidden: false,
            match_prefix: false,
            maxdepth: 1
        }
    }

    add_ext_method!(extra_suffix, add_extra_suffix);
    add_ext_method!(included_ext, add_included_ext);
    add_ext_method!(excluded_ext, add_excluded_ext);

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

        if !self.show_hidden {
            if stem.starts_with(".") {
                return false;
            }
        }

        match path.extension() {
            Some(ext_osstr) => {
                let ext = ext_osstr.to_str().expect("OsStr.to_str call failed");

                if self.is_matching_ext(ext) {
                    return true;
                }

                if self.extra_suffix.contains(ext) {
                    let mut noext = path.to_path_buf();
                    noext.set_extension("");
                    let noext_str = noext.to_str().expect("PathBuf.to_str call failed");
                    let path_noext = Path::new(noext_str);
                    match path_noext.extension() {
                        Some(_) => {
                            return self.is_matching_file(path_noext);
                        },
                        _ => {}
                    }
                }

                return false;
            },
            None => {
                return false;
            },
        }
    }

    fn dir_iter<P>(&self, path: P) -> WalkDir where P: AsRef<Path> {
        return WalkDir::new(path).min_depth(1).max_depth(1).follow_links(true).sort_by_file_name();
    }

    fn expand(&self, expanded_paths: &mut Vec<String>, path: &Path, depth: u32, strip_prefix: Option<&PathBuf>) {
        //println!("expand(_, {:?}, {})", path, depth);

        if depth > self.maxdepth {
            //println!("DEPTH({}) > MAXDEPTH({})", depth, self.maxdepth);
            return;
        }

        if path.exists() {
            let md = path.metadata().expect("metadata call failed");

            if md.is_file() {
                if self.is_matching_file(path) {
                    let striped_path = match strip_prefix {
                        Some(p) => path.strip_prefix(p).unwrap(),
                        None    => path
                    };
                    let file_path = striped_path.to_str().expect("to_str call failed");
                    expanded_paths.push(file_path.to_string());
                }
            } else if md.is_dir() {
                let newdepth = depth + 1;
                for entry in self.dir_iter(path) {
                    match entry {
                        Ok(e) => {
                            let entpath = e.path();
                            //println!("dirent: {:?}", entpath);
                            self.expand(expanded_paths, &entpath, newdepth, strip_prefix);
                        },
                        _ => { }
                    }
                }
            } else {
                // not a file or dir - ignore
            }
        } else {
            // not in filesystem
            if depth == 0 && self.match_prefix {
                self.expand_matching_prefix(expanded_paths, &path, 0);
            }
        }
    }

    fn get_parent_dir<P>(&self, path: P) -> Option<PathBuf> where P: AsRef<Path> {
        let path_val = path.as_ref();
        let parent = path_val.parent()?;
        if parent.components().next().is_none() {
            return current_dir().ok();
        }
        Some(parent.to_owned())
    }

    fn expand_matching_prefix(&self, expanded_paths: &mut Vec<String>, path: &Path, depth: u32) {
        let is_rel = path.is_relative();
        let parent = self.get_parent_dir(path).unwrap();
        let name = path.file_name().unwrap().to_str().unwrap();
        let name_wildcard = format!("{}*", name);
        let pattern_path = parent.join(name_wildcard);
        let pattern_str = pattern_path.to_str().unwrap();
        //println!("*** PATTERN: \"{}\"", pattern_str);

        for entry in self.dir_iter(parent) {
            match entry {
                Ok(e) => {
                    let entpath = e.path();
                    if glob_match(pattern_str, entpath.to_str().unwrap()) {
                        if is_rel {
                            self.expand(expanded_paths, &entpath, depth, current_dir().ok().as_ref());
                        } else {
                            self.expand(expanded_paths, &entpath, depth, None);
                        }
                    }
                },
                _ => { }
            }
        }
    }

    pub fn expand_input_path(&self, input_path: &str) -> Vec<String> {
        let mut expanded_paths: Vec<String> = Vec::new();

        let path = Path::new(input_path);
        // println!("*** EXPANDING: {}", path.display());

        self.expand(&mut expanded_paths, &path, 0, None);

        // for expath in &expanded_paths {
        //     println!("  - {}", expath);
        // }

        return expanded_paths;
    }
}

