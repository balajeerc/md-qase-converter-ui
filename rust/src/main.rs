use std::fs;
use std::path::Path;

use clap::{App, Arg};

mod convert;
mod errors;
mod load;
mod qase;

fn convert_md_file_to_qase(markdown_file: &Path) {
    let markdown_text = fs::read_to_string(markdown_file.to_str().unwrap())
        .expect("file error loading markdown_file");
    let markdown_str = &markdown_text.to_string();
    let result = convert::convert_md_to_qase(markdown_str).unwrap();
    let json = serde_json::to_string(&result).unwrap();
    println!("{}", json);
}

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("Markdown to Qase Test Case Converter")
        .version("0.1")
        .author("Balajee.R.C <balajeerc@gmail.com>")
        .about("Converts a file containing test cases in Markdown format to Qase JSON")
        .subcommand(
            App::new("convert")
                .about("Convert specified markdown to Qase JSON")
                .version("0.1")
                .arg(
                    Arg::new("markdown")
                        .long("markdown")
                        .value_name("FILE")
                        .about("Converts specified markdown file")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("load")
                .about("Load test cases from Qase JSON")
                .version("0.1")
                .arg(
                    Arg::new("json")
                        .long("json")
                        .value_name("FILE")
                        .about("Loads test cases from Qase JSON file")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("load") {
        if let Some(json_file) = matches.value_of("json") {
            println!("Loading test cases from Qase JSON: {}", json_file);
            let result = load::load_test_cases_from_json(Path::new(json_file));
            println!("Result: {:#?}\n", result);
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("convert") {
        if let Some(md_file) = matches.value_of("markdown") {
            // println!("Converting Markdown to Qase from file: {}", md_file);
            convert_md_file_to_qase(Path::new(md_file));
        }
    }

    Ok(())
}
