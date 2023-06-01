
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
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerconsent/cmrevoke/01p00")]
pub struct Cmrevoke {
    #[yaserde(prefix = "ns0", rename = "MarketParticipantDirectory")]
    pub market_participant_directory: MarketParticipantDirectory,

    #[yaserde(prefix = "ns0", rename = "ProcessDirectory")]
    pub process_directory: ProcessDirectory,
}

impl Validate for Cmrevoke {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerconsent/cmrevoke/01p00")]
pub struct MarketParticipantDirectory {
    #[yaserde(prefix = "ns0", rename = "MessageCode")]
    pub message_code: market_participant_directory::MessageCodeType,

    #[yaserde(attribute, rename = "SchemaVersion")]
    pub schema_version: String,
}

impl Validate for MarketParticipantDirectory {}

pub mod market_participant_directory {
    use super::*;
    
    #[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
    pub struct MessageCodeType (pub ns1::MessageCode);

    impl Validate for MessageCodeType {}

}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerconsent/cmrevoke/01p00")]
pub struct ProcessDirectory {
    // Zustimmungs-ID
    #[yaserde(prefix = "ns0", rename = "ConsentId")]
    pub consent_id: ns1::GroupingId,

    #[yaserde(prefix = "ns0", rename = "MeteringPoint")]
    pub metering_point: ns1::MeteringPoint,

    // Ende-Datum der Zustimmung
    #[yaserde(prefix = "ns0", rename = "ConsentEnd")]
    pub consent_end: xs::Date,

    // Grund der Bendigung
    #[yaserde(prefix = "ns0", rename = "Reason")]
    pub reason: Option<ReasonType>,
}

impl Validate for ProcessDirectory {}


#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ReasonType (pub String);

impl Validate for ReasonType {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 50 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 50 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

pub fn read_cmrevoke_01p00(file_read : &Path) -> Option<Cmrevoke>{
  if let Ok(src_file) = File::open(file_read){
    let _data: Cmrevoke = yaserde::de::from_reader(BufReader::new(src_file)).unwrap();
    return Some(_data)
  }
  None
}
pub fn write_cmrevoke_01p00(file_write : &Path, data :&Cmrevoke) -> Result<(),String>
{
 
    if let Ok(src_file) = File::create(file_write) {  
    let config: Config = Config {
        perform_indent: true,
        write_document_declaration: true,
        indent_string: None,
    };        
    if let Ok(mut content) = yaserde::ser::to_string_with_config(data, &config) {
    content = content.replace("xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/cmrevoke/01p00","xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/cmrevoke/01p00xmlns:ns1=\"http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20\""); 
        let mut bw = BufWriter::new(src_file);
        if let Ok(_write_ok) = bw.write_all(content.as_bytes()) {
            return Ok(());
        }
    }        
    return Err("error serialize content".to_string());
}
Err("can't create file".to_string())


}
