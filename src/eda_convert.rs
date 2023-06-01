use std::{path::{Path}, fs::{OpenOptions}};
use anyhow::Context;

use xsd_parser::generator::builder::GeneratorBuilder;
use xsd_parser::parser::parse;
use std::io::{prelude::*};


pub static PREPEND_XML : &str= r#"
// for deserialize xml files
use std::str::FromStr;
use xsd_parser::generator::validator::Validate;
use xsd_types::types as xs;
use xsd_macro_utils::{UtilsDefaultSerde,UtilsTupleIo};

// common types
use super::cpcommontypes_01p20 as ns1;

// for read/write functions
use yaserde::ser::Config;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader,BufWriter,Write};

"#;

pub static WRITE_PART_1_XML : &str= r#" 
    if let Ok(src_file) = File::create(file_write) {  
    let config: Config = Config {
        perform_indent: true,
        write_document_declaration: true,
        indent_string: None,
    };        
    if let Ok(mut content) = yaserde::ser::to_string_with_config(data, &config) {
"#;

pub static WRITE_PART_2_XML : &str= r#"
        let mut bw = BufWriter::new(src_file);
        if let Ok(_write_ok) = bw.write_all(content.as_bytes()) {
            return Ok(());
        }
    }        
    return Err("error serialize content".to_string());
}
Err("can't create file".to_string())

"#;

pub fn create_rs_from_schema(input_path: &Path, output_path: Option<&str>)
{
  if let Ok(text) = std::fs::read_to_string(input_path) {
    if let Ok(rs_file) = parse(text.as_str()) {
        let mut ext : String = Default::default();
        if let Some(file_ext) = input_path.extension() {
            ext = format!( ".{}", file_ext.to_string_lossy());
        }
        if let Some(file_name) = input_path.file_name() {
            let gen = GeneratorBuilder::default().build();

            println!("converting {}", file_name.to_string_lossy() );
            let function_name = file_name.to_string_lossy().replace(&ext,"").to_lowercase();
            let mut code : String = Default::default();
            let gen_code = gen.generate_rs_file(&rs_file);
            if function_name.starts_with("cpcommontypes") {
                code = PREPEND_XML.to_string() 
                    + &gen.generate_rs_file(&rs_file); 

                code = code.replace("\"ct\"", "\"ns1\"");
                code = code.replace("ct:", "ns1:");
            }
            else {

                let mut struct_name : String = Default::default();
            
                if let Some(found_start_idx) = gen_code.find("pub struct")
                {
                    let mut found_end_idx = found_start_idx;
                    while let Some(ch) = gen_code.chars().nth(found_end_idx) {
                        if ch == '{' {
                            struct_name = gen_code.get((found_start_idx + 10)..found_end_idx).unwrap().trim().into();
                            break;
                        }
                        found_end_idx = found_end_idx + 1;
                    }
                }
                
                if !struct_name.is_empty() {
                    code = PREPEND_XML.to_string() 
                    + gen.generate_rs_file(&rs_file).as_str() 
                    + &format!("pub fn read_{}(file_read : &Path) -> Option<{}>", 
                        function_name,
                        struct_name
                        )
                    + &format!("{{\n  if let Ok(src_file) = File::open(file_read){{\n    let _data: {} = yaserde::de::from_reader(BufReader::new(src_file)).unwrap();\n    return Some(_data)\n  }}\n  None\n}}"
                        ,struct_name );
                    
                    let content_path = function_name.to_lowercase().replace("_", "/");
                    let content_search : String = r#"xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/"#.to_string() + content_path.as_str();
                    let content_replace = content_search.clone() + r#"\" xmlns:ns1=\"http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20"#;
                    let content_replace_code = format!("    content = content.replace(\"{}\",\"{}\"); ", 
                        &content_search, 
                        content_replace
                    );
                    
                    code = code + &format!("\npub fn write_{}(file_write : &Path, data :&{}) -> Result<(),String>\n{{\n", 
                        function_name,
                        struct_name
                     )
                    + WRITE_PART_1_XML
                    + content_replace_code.as_str()
                    + WRITE_PART_2_XML
                    + &format!("\n}}\n");
                    
                    // HACK
                    code = code.replace("\"cp\"", "\"ns0\"");
                    code = code.replace("cp:", "ns0:");
                    code = code.replace("ct::", "ns1::");
                    // --> Generator has problems while setting up some Struct Name -> hack this
                    code = code.replace("MarketParticipantDirectoryMessageCodeType", "MessageCodeType");
                    code = code.replace("MeteringPointDataMeteringPointDataChoice", "MeteringPointDataChoice");
                    code = code.replace("MessageDataMessageTextType", "MessageTextType");
                }
            }
            if let Some(output_filename) = output_path {
                write_to_file(output_filename, &code).context("Error writing file").unwrap_or(());
            } else {
                println!("{}", code);
            }
        }
      }
    }
}


// 
// write to file
//
fn write_to_file(path: &str, text: &str) -> std::io::Result<()> {
    let mut file_path = path.to_string();
    file_path.make_ascii_lowercase();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)?;
    file.write_all(text.as_bytes())
}


