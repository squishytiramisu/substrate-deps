#![warn(clippy::all)]


mod error;
mod graph;
mod manifest;
mod metadata;
mod registry;

#[macro_use]
extern crate lazy_static;

use crate::manifest::find_manifest_file;
use clap::{crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand};
use log::{warn, LevelFilter};
use std::env;

fn parse_cli<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("manifest-path")
                .long("manifest-path")
                .value_name("path")
                .help("Path to the manifest of the runtime.")
                .takes_value(true)
                .global(true)
                .default_value("Cargo.toml"),
        )
        .arg(
            Arg::with_name("quiet")
                .long("quiet")
                .short("q")
                .global(true)
                .help("No output printed to stdout"),
        )
        .arg(
            Arg::with_name("v")
                .long("verbose")
                .short("v")
                .multiple(true)
                .global(true)
                .help("Use verbose output"),
        )
        .subcommand(
            SubCommand::with_name("graph")
                .about("Generate a graph of the Substrate runtime pallet dependencies.")
                .arg(
                    Arg::with_name("include-versions")
                    .long("include-versions")
                    .short("I")
                    .help("Include the dependency version on nodes")
                )
        )
        .get_matches()
}

fn config_log(m: &ArgMatches) {
    let log_level = if m.is_present("quiet") {
        LevelFilter::Error
    } else {
        match m.occurrences_of("v") {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            2 | _ => LevelFilter::Trace,
        }
    };
    env_logger::from_env(env_logger::Env::default().default_filter_or(format!(
        "{}={}",
        crate_name!().replace("-", "_"),
        log_level
    )))
    .format_timestamp(None)
    .format_level(false)
    .format_module_path(false)
    .init();
}

fn main() {
    let m = parse_cli();
    config_log(&m);

    let manifest = m.value_of("manifest-path").unwrap(); // manifest-path has a default value so we can safely unwrap
    let manifest_path = find_manifest_file(manifest).unwrap(); // -> Stop on error, if any

    if let Err(err) = match m.subcommand() {
        ("graph", Some(m)) => graph::execute_graph(&m),
        _ => Ok(()),
    } {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
