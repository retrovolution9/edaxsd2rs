use std::{path::{Path, PathBuf}, io::BufReader, fs::{File, self, OpenOptions}};
use anyhow::Context;
use xml::{EventReader, reader::XmlEvent};

use xsd_parser::generator::builder::GeneratorBuilder;
use xsd_parser::parser::parse;
use std::io::{prelude::*};

use super::eda_typ::{EdaRecordTyp};
use crate::eda_info;
use super::eda_info::{get_eda_info, test_readwrite};
use super::eda_convert::create_rs_from_schema;

// 
// Identify EDA xml files
//
pub fn identify_eda_xml(file_read : &Path) -> Result<EdaRecordTyp,std::io::Error>
{
    let file = match File::open(file_read) {
        Ok(f) => f,
        Err(e) => return Err(e)
    };
    let filereader = BufReader::new(file);
    
    let event_reader = EventReader::new(filereader);
    for e in event_reader {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                match name.local_name.as_str() {
                    "ConsumptionRecord" => {
                        if name.namespace.is_some() {
                            return Ok(match name.namespace.unwrap().as_str() {
                                "http://www.ebutilities.at/schemata/customerprocesses/consumptionrecord/01p30" => EdaRecordTyp::ConsumptionRecord1p30,
                                "http://www.ebutilities.at/schemata/customerprocesses/consumptionrecord/01p31" => EdaRecordTyp::ConsumptionRecord1p31,
                                &_ => EdaRecordTyp::Unknown,
                            });
                        }
                        return Ok(EdaRecordTyp::ConsumptionRecord1p30)
                    },
                    "CPNotification" => {
                        if name.namespace.is_some() {
                            return Ok(match name.namespace.unwrap().as_str() {
                                "http://www.ebutilities.at/schemata/customerprocesses/cpnotification/01p13" => EdaRecordTyp::CpNotification1p13,
                                &_ => EdaRecordTyp::Unknown,
                            });
                        }
                        return Ok(EdaRecordTyp::CpNotification1p13)
                    },
                    "CPRequest" => {
                        if name.namespace.is_some() {
                            return Ok(match name.namespace.unwrap().as_str() {
                                "http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12" => EdaRecordTyp::CpRequest1p12,
                                &_ => EdaRecordTyp::Unknown,
                            });
                        }
                        return Ok(EdaRecordTyp::CpRequest1p12)
                    },
                    _ => return Ok(EdaRecordTyp::Unknown)
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Ok(EdaRecordTyp::Invalid);
            }
            _ => {}
        }
    };
    return Ok(EdaRecordTyp::Unknown)
}


// 
// Rename EDA xml files
//
pub fn rename_eda_file(file_path:&Path)
{
  let edatyp = match identify_eda_xml(&file_path) {
      Ok(edatype) => edatype,
      Err(e) => return,
  };
  rename_eda_xml_file(edatyp, &file_path);
}

// 
// Rename EDA xml files in folders
//
pub fn rename_eda_xml_folder(folder:&Path)
{    
    let mut readfiles : Vec<PathBuf> = vec![];
    for file in fs::read_dir(folder).unwrap() {
        if file.is_ok()
        {          
            let strfile = file.unwrap().path();
            readfiles.push(strfile);
        }
    }
   
    let ret = true;
    for readfile in &readfiles{
        let p = readfile;

        let edatyp = match identify_eda_xml(p) {
            Ok(edatype) => edatype,
            Err(e) => return,
        };

        let ret = rename_eda_xml_file(edatyp, p);        
    }
}

// 
// Rename EDA xml file
//
pub fn rename_eda_xml_file(edatyp: EdaRecordTyp, p:&Path) -> bool
{    
    let ret = match edatyp {
        EdaRecordTyp::Invalid => false,
        EdaRecordTyp::Unknown => false,
        EdaRecordTyp::ConsumptionRecord1p30 => false,
        EdaRecordTyp::ConsumptionRecord1p31 => false,
        EdaRecordTyp::Cmrequest1p10 => false,
        EdaRecordTyp::Cmrevoke1p00 => false,
        EdaRecordTyp::Message1p10 => false,
        EdaRecordTyp::CpNotification1p13 => false,
        EdaRecordTyp::EcmpList1p00 => false,
        EdaRecordTyp::MasterData1p31 => false,
        EdaRecordTyp::CpRequest1p12 => false,
    };
    ret
}

pub fn print_eda_identity( edatyp: &EdaRecordTyp, file_path: &Path)
{
    match edatyp
    {
        EdaRecordTyp::Invalid => {println!( "error: invalid" ); return },
        EdaRecordTyp::Unknown => {println!( "unknown" ); return},
        _ => (),
    }

    let info = get_eda_info( edatyp, &file_path );
    let typ = match edatyp
    {
        EdaRecordTyp::ConsumptionRecord1p30 => "ConsumptionRecord1p30",
        EdaRecordTyp::ConsumptionRecord1p31 => "ConsumptionRecord1p31",
        EdaRecordTyp::Cmrequest1p10 => "Cmrequest1p10",
        EdaRecordTyp::Cmrevoke1p00 => "Cmrevoke1p00",
        EdaRecordTyp::CpNotification1p13 => "CpNotification1p13",
        EdaRecordTyp::CpRequest1p12 => "CpRequest1p12",
        EdaRecordTyp::EcmpList1p00 => "EcmpList1p00",
        EdaRecordTyp::MasterData1p31 => "MasterData1p31",
        EdaRecordTyp::Message1p10 => "Message1p10",       
        _ => "",
    };


    println!( "found: {}", typ);
    println!( "info: {}", info);
}



// 
// Testing EDA xml files
//
pub fn test_readwrite_eda_file(file_path:&Path)
{
  let edatyp = match identify_eda_xml(&file_path) {
      Ok(edatype) => edatype,
      Err(e) => return,
  };
  eda_info::test_readwrite(&edatyp, &file_path);
}
