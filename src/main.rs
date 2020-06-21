use std::fs;
use std::path::Path;

use rustympkglib::pkgdata::PkgData;

fn main() {
    let matches = clap::App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .arg(
            clap::Arg::with_name("pkgbuild")
                .short("p")
                .value_name("file")
                .help("Use an alternate build script")
                .takes_value(true)
                .default_value("PKGBUILD"),
        )
        .subcommand(
            clap::SubCommand::with_name("json")
                .about("Print the parsed JSON representation of the PKGBUILD"),
        )
        .get_matches();

    let source_path = Path::new(matches.value_of("pkgbuild").unwrap());
    let source_code = fs::read_to_string(source_path).unwrap();

    if matches.subcommand_matches("json").is_some() {
        let pkgdata = PkgData::from_source(&source_code).unwrap();
        println!("{}", serde_json::to_string(&pkgdata).unwrap());
    }
}
