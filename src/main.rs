use std::fs;
use std::path::Path;

use clap::{Parser, Subcommand};

use rustympkglib::pkgdata::PkgData;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[clap(short, long, default_value_t=String::from("PKGBUILD"))]
    /// Use an alternate build script path
    pkgbuild: String,

    #[clap(subcommand)]
    export: ExportType,
}

#[derive(Subcommand, Debug)]
enum ExportType {
    /// Export the parsed JSON representation of the PKGBUILD
    Json,
}

fn main() {
    let args = Arguments::parse();
    let source_path = Path::new(&args.pkgbuild);
    let source_code = fs::read_to_string(source_path).unwrap();

    match args.export {
        ExportType::Json {} => {
            let pkgdata = PkgData::from_source(&source_code).unwrap();
            println!("{}", serde_json::to_string(&pkgdata).unwrap());
        }
    }
}
