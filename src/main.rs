#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;


fn main() {
    let mut args = reborn::cli::RebornCliArgs::parse();

    if args.debug {
        args.print_cpool = true;
        args.print_class = true;
        args.print_code = true;
    }

    if args.input.len() > 0 {
        let mut input_files: Vec<String> = vec![];
        reborn::deobfuscator::gather_input_files(&mut input_files, &args.input);
        reborn::deobfuscator::deobf_many(&args, &input_files);
        return;
    }
    #[cfg(feature = "gui")]
    gui::app::App::run().expect("Unable to run GUI app");
}
