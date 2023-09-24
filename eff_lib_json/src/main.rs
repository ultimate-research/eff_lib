use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use eff_lib::EffFile;

/// Convert EFF files to and from JSON
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input EFF or JSON file path
    pub input: String,

    /// The output EFF or JSON file path
    pub output: Option<String>,

    /// The input or output PTCL file path
    pub ptcl: Option<String>,
}

fn read_data_write_json<P: AsRef<Path> + ToString>(
    input_path: P,
    output_path: Option<String>,
    ptcl_path: Option<String>,
) {
    let output_path = output_path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(&(input_path.to_string() + ".json")));
    let ptcl_path = ptcl_path
        .map(PathBuf::from)
        .unwrap_or_else(|| input_path.as_ref().with_extension("ptcl"));

    match EffFile::from_file(input_path) {
        Ok(eff) => {
            let json = serde_json::to_string_pretty(&eff).unwrap();

            fs::write(output_path, json).expect("failed to write JSON file");
            eff.write_resource_to_file(ptcl_path)
                .expect("failed to write PTCL file");
        }
        Err(error) => eprintln!("{error:?}"),
    }
}

fn read_json_write_data<P: AsRef<Path>>(
    input_path: P,
    output_path: Option<String>,
    ptcl_path: Option<String>,
) {
    let json = fs::read_to_string(&input_path).unwrap();

    match serde_json::from_str::<EffFile>(&json) {
        Ok(mut eff) => {
            let output_path = output_path
                .map(PathBuf::from)
                .unwrap_or_else(|| input_path.as_ref().with_extension("eff"));
            let ptcl_path = ptcl_path.map(PathBuf::from).unwrap_or_else(|| {
                input_path
                    .as_ref()
                    .with_extension("")
                    .with_extension("ptcl")
            });

            eff.resource_data = fs::read(ptcl_path).ok();
            eff.write_to_file(output_path)
                .expect("failed to write EFF file");
        }
        Err(error) => eprintln!("{error:?}"),
    }
}

fn main() {
    let args = Args::parse();

    match Path::new(&args.input)
        .extension()
        .expect("input file extension should exist")
        .to_str()
        .unwrap()
    {
        "json" => read_json_write_data(args.input, args.output, args.ptcl),
        _ => read_data_write_json(args.input, args.output, args.ptcl),
    }
}
