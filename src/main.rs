#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(
    all(
        not(debug_assertions),
        feature = "gui"
    ),
    windows_subsystem = "windows"
)]

use std::fs::{read_to_string, File};
use std::io;
use std::io::{BufRead, BufReader};
#[cfg(feature = "cli")]
use clap::Parser;
use colored::Colorize;
use utils::cache;

fn main() {
    #[cfg(feature = "cli")]
    {
        let mut args = reborn::cli::RebornCliArgs::parse();

        if args.show_versions || args.input.is_empty() {
            if args.input.is_empty() {
                println!("\
                No input files / directories specified, just showing available mappings!\n\
                To get help about usage run with '--help' flag")
            }
            let versions = utils::cache::read_versions_json();

            let mut versions_iter = versions
                .iter()
                .filter(|(version, _)| **version == args.game_version || args.game_version == "*")
                .enumerate()
                .peekable();

            while let Some((i, (game_version, mappings))) = versions_iter.next() {
                let game_version_text = game_version.cyan();

                let (repr_prefix, map_type_prefix) = if i == 0 {
                    if let Some(_) = versions_iter.peek() {
                        ("┌─┬─", "│ ")
                    } else {
                        ("──┬─", "  ")
                    }
                } else if i == versions.len() - 1 {
                    ("└─┬─", "  ")
                } else {
                    ("├─┬─", "│ ")
                };

                println!("{} {}", repr_prefix, game_version_text);

                for (i, (map_type, map_versions)) in mappings.iter().enumerate() {
                    let map_type_text = map_type.bright_magenta();

                    let (repr_prefix, map_version_prefix) = if i == (mappings.len() - 1) {
                        ("└─┬─", "  ")
                    } else {
                        ("├─┬─", "│ ")
                    };

                    let prefix = map_type_prefix.to_owned() + repr_prefix;
                    println!("{} {}", prefix, map_type_text);

                    for (i, map_version) in map_versions.iter().enumerate() {
                        let map_version_text = if utils::cache::mappings_exists(game_version, map_type, map_version) {
                            map_version.green()
                        } else {
                            map_version.red()
                        };

                        let repr_prefix = if i == (map_versions.len() - 1) {
                            "└──"
                        } else {
                            "├──"
                        };

                        let prefix = map_type_prefix.to_owned() + map_version_prefix + repr_prefix;

                        println!("{} {}", prefix, map_version_text);
                    }
                }
            }
            return;
        }

        if args.debug {
            args.print_cpool = true;
            args.print_class = true;
            args.print_code = true;
        }

        if args.input.len() > 0 {
            let mut input_files: Vec<String> = vec![];
            if !cache::mappings_exists(args.game_version.as_str(),
                                       args.mappings_channel.as_str(),
                                       args.mappings_version.as_str()) {
                reborn::remapper::prepare_mappings(&args);
            }
            reborn::remapper::gather_input_files(&mut input_files, &args.input);
            reborn::remapper::remap_files(&args, &input_files);
            return;
        }
    }

    #[cfg(feature = "gui")]
    {
        gui::app::App::run().expect("Unable to run GUI app");
    }
    tsrg_trie::tsrg_parser::read_tsrg("./joined.tsrg");
}
