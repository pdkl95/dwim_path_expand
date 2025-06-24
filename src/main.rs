#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches, AppSettings};

mod expander;
use expander::PathExpander;

mod rng;
use rng::RNG;

arg_enum! {
    #[derive(Debug)]
    enum OutputOrder {
        PRESERVE,
        SORT,
        REVERSE,
        RANDOM
    }
}

fn find_output_order(matches: &ArgMatches) -> OutputOrder {
    if matches.is_present("sort_order") {
        return OutputOrder::SORT;
    } else if matches.is_present("reverse_order") {
        return OutputOrder::REVERSE;
    } else if matches.is_present("rand_order") {
        return OutputOrder::RANDOM;
    } else {
        return value_t!(matches, "order", OutputOrder).unwrap_or_else(|e| e.exit());
    }
}

fn find_app_options() -> App<'static> {
    return App::new("dwim_path_expand")
        .version(crate_version!()) 
        .author(crate_authors!())
        .about(crate_description!())
        .term_width(70)
        .global_setting(AppSettings::NextLineHelp)
        .arg(Arg::with_name("maxdepth")
             .short('d')
             .long("maxdepth")
             .help("Maximum directory recursion depth")
             .default_value("1")
             .help_heading("FILENAME MATCHING")
             .display_order(100)
        )
        .arg(Arg::with_name("show_hidden")
             .short('a')
             .long("all")
             .help("Output includes hidden files (\"hidden\" filenames start with \".\")")
             .help_heading("FILENAME MATCHING")
             .display_order(101)
        )
        .arg(Arg::with_name("match_prefix")
             .short('p')
             .long("match-prefix")
             .help("Match all files with an <input_path> prefix.")
             .help_heading("FILENAME MATCHING")
             .display_order(102)
        )
        .arg(Arg::with_name("match_concat")
             .short('c')
             .long("match-concat")
             .help("Match multiple filenames concatenated into a single <input_path> string.")
             .help_heading("FILENAME MATCHING")
             .display_order(103)
        )
        .arg(Arg::with_name("included_ext")
             .short('i')
             .long("include")
             .help("Only match files with these extensions")
             .takes_value(true)
             .help_heading("FILENAME MATCHING")
             .display_order(104)
        )
        .arg(Arg::with_name("excluded_ext")
             .short('e')
             .long("exclude")
             .help("Never match files with these extensions")
             .takes_value(true)
             .help_heading("FILENAME MATCHING")
             .display_order(105)
        )
        .arg(Arg::with_name("extra_suffix")
             .short('x')
             .long("extra-suffix")
             .help("Also include files with these extensions appended to the --include extensions")
             .takes_value(true)
             .help_heading("FILENAME MATCHING")
             .display_order(106)
        )
        .arg(Arg::with_name("zero")
             .short('0')
             .long("zero-terminated")
             .help("Separate output paths with a NULL char '\\0', similar to \"find ... -print0\"")
             .help_heading("OUTPUT")
             .display_order(201)
        )
        .arg(Arg::with_name("order")
             .short('o')
             .long("order")
             .possible_values(OutputOrder::variants())
             .default_value("PRESERVE")
             .case_insensitive(true)
             .help_heading("OUTPUT")
             .display_order(202)
        )
        .arg(Arg::with_name("sort_order")
             .short('s')
             .long("sort")
             .help("Shortcut for --order=SORT")
             .help_heading("OUTPUT")
             .display_order(203)
        )
        .arg(Arg::with_name("reverse_order")
             .short('R')
             .long("reverse")
             .help("Shortcut for --order=REVERSE")
             .help_heading("OUTPUT")
             .display_order(204)
        )
        .arg(Arg::with_name("rand_order")
             .short('r')
             .long("random")
             .help("Shortcut for --order=RANDOM")
             .help_heading("OUTPUT")
             .display_order(205)
        )
        .arg(Arg::with_name("input_paths")
             .help("Partial paths to expand")
             .takes_value(true)
             .multiple(true)
        );
}

fn main() {
    let app_options = find_app_options();
    let matches = app_options.get_matches();
    let output_order = find_output_order(&matches);
    let mut expander = PathExpander::new();

    if matches.is_present("show_hidden") {
        expander.show_hidden = true;
    }

    if matches.is_present("match_prefix") {
        expander.match_prefix = true;
    }

    if matches.is_present("match_concat") {
        expander.match_concat = true;
    }

    if matches.is_present("maxdepth") {
        let md_str = matches.value_of("maxdepth").unwrap();
        match md_str.parse::<u32>() {
            Ok(n) => expander.maxdepth = n,
            Err(e) => panic!("--maxdepth \"{}\" is not an integer: {}",
                             md_str, e)
        }
    }

    if matches.is_present("extra_suffix") {
        let extstr = matches.value_of("extra_suffix").unwrap();
        let extlist: Vec<&str> = extstr.split(',').collect();
        for ext in extlist {
            expander.add_extra_suffix(ext);
        }
    }

    if matches.is_present("included_ext") {
        let extstr = matches.value_of("included_ext").unwrap();
        let extlist: Vec<&str> = extstr.split(',').collect();
        for ext in extlist {
            expander.add_included_ext(ext);
        }
    }

    if matches.is_present("excluded_ext") {
        let extstr = matches.value_of("excluded_ext").unwrap();
        let extlist: Vec<&str> = extstr.split(',').collect();
        for ext in extlist {
            expander.add_excluded_ext(ext);
        }
    }

    let mut paths: Vec<String> = Vec::new();

    let input_paths: Vec<&str> = if matches.is_present("input_paths") {
        matches.values_of("input_paths").unwrap().collect()
    } else {
        vec![]
    };

    for input_path in input_paths {
        let mut expanded_paths = expander.expand_input_path(input_path);
        paths.append(&mut expanded_paths);
    }

    match output_order {
        OutputOrder::PRESERVE => {},
        OutputOrder::SORT => {
            paths.sort();
        },
        OutputOrder::REVERSE => {
            paths.sort();
            paths.reverse();
        },
        OutputOrder::RANDOM => {
            let mut rand = RNG::new();
            rand.seed_from_current_time();
            rand.shuffle(&mut paths);
        },
    };

    for path in paths {
        println!("{}", path);
    }
}
