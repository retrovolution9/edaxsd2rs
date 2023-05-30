
// for deserialize xml files
use std::str::FromStr;
use xsd_parser::generator::validator::Validate;
use xsd_types::types as xs;
use xsd_macro_utils::{UtilsDefaultSerde,UtilsTupleIo};

// common types
use super::cpcommontypes_01p20 as ct;

// for read/write functions
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

//use CPCommonTypes_01p20.xsd  http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerconsent/cmrevoke/01p00")]
pub struct Cmrevoke {
    #[yaserde(prefix = "cp", rename = "MarketParticipantDirectory")]
    pub market_participant_directory: MarketParticipantDirectory,

    #[yaserde(prefix = "cp", rename = "ProcessDirectory")]
    pub process_directory: ProcessDirectory,
}

impl Validate for Cmrevoke {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerconsent/cmrevoke/01p00")]
pub struct MarketParticipantDirectory {
    #[yaserde(prefix = "cp", rename = "MessageCode")]
    pub message_code: market_participant_directory::MessageCodeType,

    #[yaserde(attribute, rename = "SchemaVersion")]
    pub schema_version: String,
}

impl Validate for MarketParticipantDirectory {}

pub mod market_participant_directory {
    use super::*;
    
    #[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
    pub struct MessageCodeType (pub ct::MessageCode);

    impl Validate for MessageCodeType {}

}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerconsent/cmrevoke/01p00")]
pub struct ProcessDirectory {
    // Zustimmungs-ID
    #[yaserde(prefix = "cp", rename = "ConsentId")]
    pub consent_id: ct::GroupingId,

    #[yaserde(prefix = "cp", rename = "MeteringPoint")]
    pub metering_point: ct::MeteringPoint,

    // Ende-Datum der Zustimmung
    #[yaserde(prefix = "cp", rename = "ConsentEnd")]
    pub consent_end: xs::Date,

    // Grund der Bendigung
    #[yaserde(prefix = "cp", rename = "Reason")]
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