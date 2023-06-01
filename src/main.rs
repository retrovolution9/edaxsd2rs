use clap::{Arg, Command, ArgAction, ValueHint, value_parser};
use eda_info::EdaRecordTyp;

use std::fs::{self};
use std::path::{Path, PathBuf};

mod eda;
use eda::{identify_eda_xml, rename_eda_file, print_eda_identity, test_readwrite_eda_file};

mod edagen;
mod eda_info;
mod eda_convert;
use eda_convert::create_rs_from_schema;

#[macro_use]
extern crate yaserde_derive;

enum EMode {
    CONVERT,
    RENAME,
    IDENTIFY,
    TESTREADWRITE,
}

fn main() {
    let m = Command::new("edaxsd2rs")
    .author("retrovolution, office@retrovolution.at")
    .version("0.1.0")
    .about("Convert, Analyse & Identify EDA schema files")
    .arg(
        Arg::new("convert")
        .long("convert")
        .short('c')
        .required(false)
        .action(ArgAction::SetTrue)
        .help("Convert XSD schema to rust rs"))
    .arg(
        Arg::new("rename")
        .long("rename")
        .short('r')
        .required(false)
        .action(ArgAction::SetTrue)
        .help("rename eda xml file(s)"))
    .arg(
        Arg::new("identify")
        .long("identify")
        .short('i')
        .required(false)
        .action(ArgAction::SetTrue)
        .help("identify eda xml file(s)")
    )
    .arg(
        Arg::new("test")
        .long("test")
        .short('t')
        .required(false)
        .action(ArgAction::SetTrue)
        .help("test readwrite eda xml file(s)")
    )
    .arg(
        Arg::new("folder")
        .long("folder")
        .short('o')
        .value_name("FOLDER")
        .required(false)
        .value_hint(ValueHint::DirPath)
        .value_parser(value_parser!(PathBuf))
        .help("Folder"))
    .arg(
        Arg::new("file")
        .long("file")
        .short('f')
        .value_name("FILE")
        .value_hint(ValueHint::FilePath)
        .value_parser(value_parser!(PathBuf))
        .required(false)
        .help("File")
    )
    .after_help("Longer explanation to appear after the options when \
                 displaying the help information from --help or -h")
    .get_matches();

    let mut cmd_v : Vec<EMode> = vec![];
    
    if *m.get_one::<bool>("convert").unwrap() {
        cmd_v.push(EMode::CONVERT)
    }

    if *m.get_one::<bool>("rename").unwrap() {
        cmd_v.push(EMode::RENAME)
    }
    
    if *m.get_one::<bool>("identify").unwrap() {
        cmd_v.push(EMode::IDENTIFY)
    }
    
    if let Some(folder) = m.get_one::<PathBuf>("folder")
    {
        process_dir( &cmd_v, &folder, &folder).unwrap_or(());
    }
    if let Some(file) = m.get_one::<PathBuf>("file")
    {
        let output_file_path = PathBuf::from(file.file_name().unwrap()).with_extension("rs");
        for m in cmd_v 
        {
            process_single_file(&m, &file, output_file_path.to_str()).unwrap_or(());
        }
    }
}

// 
// Process directory
//
fn process_dir(cmd_v: &Vec<EMode>, input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    }
    for m in cmd_v {
        for entry in fs::read_dir(input_path)? {
            let path = entry?.path();
            if path.is_dir() {
                process_dir(&cmd_v, &path, &output_path.join(path.file_name().unwrap()))?;
            } else {
                let output_file = PathBuf::from(path.file_name().unwrap()).with_extension("rs");
                let output_file_path = output_path.join(output_file);
                process_single_file(&m, &path, output_file_path.to_str())?;
            }
        }
    }
    Ok(())
}

// 
// Process single file
//
// EMode::CONVERT       ... Convert from XSD ebutilities schema file to Rust source file(s)
// EMode::IDENTIFY      ... Identify ebutilities XML data file 
// EMode::RENAME        ... Rename ebutilities XML data file to human readable name
//
fn process_single_file(mode: &EMode, input_path: &Path, output_path: Option<&str>) -> anyhow::Result<()> 
{
    if let Some(ext) = input_path.extension() {
        match mode {
            EMode::CONVERT => {  
                // convert only xsd files 
                if ext == "xsd" {
                    create_rs_from_schema(&input_path, output_path);
                }
            },
            EMode::RENAME => { 
                if ext == "xml" {
                    rename_eda_file(&input_path);
                }
            },
            EMode::IDENTIFY => {
                if ext == "xml" {
                    println!("identify {}", input_path.as_os_str().to_string_lossy());
                    let ident = &identify_eda_xml(&input_path).unwrap_or(EdaRecordTyp::Invalid);    
                    print_eda_identity( ident, &input_path);
               }
            },
            EMode::TESTREADWRITE => {  
                // convert only xsd files 
                if ext == "xsd" {
                    println!("test_readwrite {}", input_path.as_os_str().to_string_lossy());
                    test_readwrite_eda_file(&input_path);
                }
            },
        }
    }
    Ok(())
}
