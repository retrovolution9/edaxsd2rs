
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

//use CPCommonTypes_01p20.xsd  http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cpnotification/01p13")]
pub struct Cpnotification {
    #[yaserde(prefix = "ns0", rename = "MarketParticipantDirectory")]
    pub market_participant_directory: MarketParticipantDirectory,

    #[yaserde(prefix = "ns0", rename = "ProcessDirectory")]
    pub process_directory: ProcessDirectory,
}

impl Validate for Cpnotification {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cpnotification/01p13")]
pub struct MarketParticipantDirectory {
    #[yaserde(prefix = "ns0", rename = "MessageCode")]
    pub message_code: ns1::MessageCode,

    #[yaserde(attribute, rename = "SchemaVersion")]
    pub schema_version: String,
}

impl Validate for MarketParticipantDirectory {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cpnotification/01p13")]
pub struct ProcessDirectory {
    #[yaserde(prefix = "ns0", rename = "ResponseData")]
    pub response_data: ResponseData,

    #[yaserde(prefix = "ns0", rename = "AdditionalData")]
    pub additional_data: Vec<ns1::AdditionalData>,
}

impl Validate for ProcessDirectory {}


// Fehlermeldungsdaten
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cpnotification/01p13")]
pub struct ResponseData {
    #[yaserde(prefix = "ns0", rename = "OriginalMessageID")]
    pub original_message_id: ns1::GroupingId,

    #[yaserde(prefix = "ns0", rename = "ResponseCode")]
    pub response_code: Vec<ResponseCode>,
}

impl Validate for ResponseData {}


// MessageCode innerhalb des Verfahrensschritts
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ResponseCode (pub xs::Integer);

impl Validate for ResponseCode {
    fn validate(&self) -> Result<(), String> { 
        if self.0 < "1".parse::<xs::Integer>().unwrap() {
            return Err(format!("MinInclusive validation error: invalid value of 0! \nExpected: 0 >= 1.\nActual: 0 == {}", self.0));
        }
        if self.0 > "999".parse::<xs::Integer>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 999.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

pub fn read_cpnotification_01p13(file_read : &Path) -> Option<Cpnotification>{
  if let Ok(src_file) = File::open(file_read){
    let _data: Cpnotification = yaserde::de::from_reader(BufReader::new(src_file)).unwrap();
    return Some(_data)
  }
  None
}
pub fn write_cpnotification_01p13(file_write : &Path, data :&Cpnotification) -> Result<(),String>
{
 
    if let Ok(src_file) = File::create(file_write) {  
    let config: Config = Config {
        perform_indent: true,
        write_document_declaration: true,
        indent_string: None,
    };        
    if let Ok(mut content) = yaserde::ser::to_string_with_config(data, &config) {
    content = content.replace("xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/cpnotification/01p13","xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/cpnotification/01p13xmlns:ns1=\"http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20\""); 
        let mut bw = BufWriter::new(src_file);
        if let Ok(_write_ok) = bw.write_all(content.as_bytes()) {
            return Ok(());
        }
    }        
    return Err("error serialize content".to_string());
}
Err("can't create file".to_string())


}
