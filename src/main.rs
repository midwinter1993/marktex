use std::fs::File;
use std::io::Read;
use std::path::Path;

use clap::{Arg, App};

mod converter;
use converter::Converter;

fn main() -> std::io::Result<()> {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Dongjie Chen <midwinter1993@gmail.com>")
        .about("Writer paper and notes in simple markdown.")
        .arg(Arg::new("md_file")
                 .long("in-markdown")
                 .takes_value(true)
                 .required(true)
                 .about("Input markdown file path"))
        .arg(Arg::new("output")
                 .long("out-tex")
                 .takes_value(true)
                 .required(true)
                 .default_value("./output.tex")
                 .about("Output tex file path"))
        .get_matches();

    let md_file = matches.value_of("md_file").unwrap();
    let output= matches.value_of("output").unwrap();
    println!("The markdown file passed is: {}", md_file);

    if !Path::new(md_file).exists() {
        println!("{} not exist!", md_file);
        return Ok(());
    }

    let mut file = File::open(md_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut convert = Converter::new(output);

    convert.parse_markdown(contents)?;

    Ok(())
}
