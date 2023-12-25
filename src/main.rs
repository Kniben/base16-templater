use std::{io::{self}, error::Error, fs::{self, File}, path::Path};

use clap::{arg, Arg};
use glob::glob;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SchemeInput {
    scheme: String,
    author: String,
    base00: String,
    base01: String,
    base02: String,
    base03: String,
    base04: String,
    base05: String,
    base06: String,
    base07: String,
    base08: String,
    base09: String,
    #[serde(rename = "base0A")]
    base0a: String,
    #[serde(rename = "base0B")]
    base0b: String,
    #[serde(rename = "base0C")]
    base0c: String,
    #[serde(rename = "base0D")]
    base0d: String,
    #[serde(rename = "base0E")]
    base0e: String,
    #[serde(rename = "base0F")]
    base0f: String,
}

#[derive(Serialize)]
struct SchemeOutput {
    #[serde(rename = "scheme-name")]
    scheme_name: String,
    #[serde(rename = "scheme-author")]
    scheme_author: String,
    #[serde(rename = "base00-hex")]
    base00_hex: String,
    #[serde(rename = "base01-hex")]
    base01_hex: String,
    #[serde(rename = "base02-hex")]
    base02_hex: String,
    #[serde(rename = "base03-hex")]
    base03_hex: String,
    #[serde(rename = "base04-hex")]
    base04_hex: String,
    #[serde(rename = "base05-hex")]
    base05_hex: String,
    #[serde(rename = "base06-hex")]
    base06_hex: String,
    #[serde(rename = "base07-hex")]
    base07_hex: String,
    #[serde(rename = "base08-hex")]
    base08_hex: String,
    #[serde(rename = "base09-hex")]
    base09_hex: String,
    #[serde(rename = "base0A-hex")]
    base0a_hex: String,
    #[serde(rename = "base0B-hex")]
    base0b_hex: String,
    #[serde(rename = "base0C-hex")]
    base0c_hex: String,
    #[serde(rename = "base0D-hex")]
    base0d_hex: String,
    #[serde(rename = "base0E-hex")]
    base0e_hex: String,
    #[serde(rename = "base0F-hex")]
    base0f_hex: String,
}

impl From<&SchemeInput> for SchemeOutput {
    fn from(value: &SchemeInput) -> Self {
        Self {
            scheme_name: value.scheme.clone(),
            scheme_author: value.author.clone(),
            base00_hex: value.base00.clone(),
            base01_hex: value.base01.clone(),
            base02_hex: value.base02.clone(),
            base03_hex: value.base03.clone(),
            base04_hex: value.base04.clone(),
            base05_hex: value.base05.clone(),
            base06_hex: value.base06.clone(),
            base07_hex: value.base07.clone(),
            base08_hex: value.base08.clone(),
            base09_hex: value.base09.clone(),
            base0a_hex: value.base0a.clone(),
            base0b_hex: value.base0b.clone(),
            base0c_hex: value.base0c.clone(),
            base0d_hex: value.base0d.clone(),
            base0e_hex: value.base0e.clone(),
            base0f_hex: value.base0f.clone(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = clap::Command::new("base16-templater")
        .arg(Arg::new("DRY_RUN").long("dry-run").num_args(0))
        .arg(arg!(--scheme <SCHEME_FILE>).required(true))
        .arg(arg!(<DIR>))
        .get_matches();

    let scheme = fs::read(args.get_one::<String>("scheme").unwrap())?;
    let content = serde_yaml::from_slice::<SchemeInput>(&scheme)?;

    let dir = args.get_one::<String>("DIR").unwrap();
    for path_result in glob(&format!("{}/**/base16-templater*.mustache", dir))? {
        let Ok(path) = path_result else {
            continue;
        };

        let template_string = fs::read_to_string(path.as_path())?;
        let template = mustache::compile_str(&template_string)?;

        let parent = path.parent().expect("Path had no parent dir");
        let file_name = path.file_stem().expect("Path had no file name");
        let output_path = parent.join(file_name);
        
        let output = template.render_to_string(&SchemeOutput::from(&content))?;

        println!("{}", output_path.to_string_lossy());

        if !*args.get_one::<bool>("DRY_RUN").unwrap_or(&false) {
            fs::write(output_path, output)?;
        }
    }

        Ok(())
}
