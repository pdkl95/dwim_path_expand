#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches};

arg_enum! {
    #[derive(Debug)]
    enum OutputOrder {
        PRESERVE,
        SORT,
        RANDOM
    }
}

fn find_output_order(matches: &ArgMatches) -> OutputOrder {
    if matches.is_present("sort_order") {
        return OutputOrder::SORT;
    } else if matches.is_present("rand_order") {
        return OutputOrder::RANDOM;
    } else {
        return value_t!(matches, "order", OutputOrder).unwrap_or_else(|e| e.exit());
    }
}

fn find_arg_matches() -> ArgMatches<'static> {
    return App::new("dwim_path_expand")
        .version(crate_version!()) 
        .author(crate_authors!())
        .about(crate_description!()) 
        .arg(Arg::with_name("hidden")
             .short("a")
             .long("all")
             .help("Output includes hidden files (\"hidden\" filenames start with \".\"")
        )
        .arg(Arg::with_name("maxdepth")
             .short("d")
             .long("maxdepth")
             .help("Maximum directory recursion depth")
             .default_value("1")
        )
        .arg(Arg::with_name("include_ext")
             .short("i")
             .long("include")
             .help("Only match files with these extensions")
             .takes_value(true)
             .multiple(true)
        )
        .arg(Arg::with_name("exclude_ext")
             .short("e")
             .long("exclude")
             .help("Never match files with these extensions")
             .takes_value(true)
             .multiple(true)
        )
        .arg(Arg::with_name("zero")
             .short("0")
             .long("zero-terminated")
             .help("Separate output paths with \0, similar to \"find ... -print0\"")
        )
        .arg(Arg::with_name("order")
             .short("o")
             .long("order")
             .possible_values(&OutputOrder::variants())
             .default_value("PRESERVE")
             .case_insensitive(true)
        )
        .arg(Arg::with_name("sort_order")
             .short("s")
             .long("sort")
             .help("Shortcut for --order=SORT")
        )
        .arg(Arg::with_name("rand_order")
             .short("r")
             .long("random")
             .help("Shortcut for --order=RANDOM")
        )
        .arg(Arg::with_name("input_paths")
             .help("Partial paths to expand")
             .takes_value(true)
             .multiple(true)
        )
        .get_matches();
}

fn main() {
    let matches = find_arg_matches();
    let output_order = find_output_order(&matches);

    println!("Output Order: {}", output_order);

    if let Some(in_v) = matches.values_of("input_paths") {
        for input_path in in_v {
            println!("An input path: \"{}\"", input_path);
        }
    }
}
