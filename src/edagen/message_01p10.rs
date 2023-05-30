
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
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/message/01p10")]
pub struct Message {
    #[yaserde(prefix = "cp", rename = "MarketParticipantDirectory")]
    pub market_participant_directory: MarketParticipantDirectory,

    #[yaserde(prefix = "cp", rename = "ProcessDirectory")]
    pub process_directory: ProcessDirectory,
}

impl Validate for Message {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/message/01p10")]
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
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/message/01p10")]
pub struct ProcessDirectory {
    #[yaserde(prefix = "cp", rename = "ProcessDate")]
    pub process_date: xs::Date,

    #[yaserde(prefix = "cp", rename = "MessageData")]
    pub message_data: MessageData,
}

impl Validate for ProcessDirectory {}


// Nachricht
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/message/01p10")]
pub struct MessageData {
    #[yaserde(prefix = "cp", rename = "Subject")]
    pub subject: SubjectType,

    #[yaserde(prefix = "cp", rename = "InfoType")]
    pub info_type: InfoTypeType,

    #[yaserde(prefix = "cp", rename = "MessageText")]
    pub message_text: message_data::MessageTextType,
}

impl Validate for MessageData {}

pub mod message_data {
    use super::*;
    
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/message/01p10")]
    pub struct MessageTextType {
        #[yaserde(attribute, rename = "Tag")]
        pub tag: Option<MessageTextTag>,
    }

    impl Validate for MessageTextType {}

}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/message/01p10")]

pub enum SubjectTypeChoice {
    ConversationId(ct::GroupingId),
    MeteringPoint(ct::MeteringPoint),
    InvoiceNumber(InvoiceNumberType),
    __Unknown__(String),
}

impl Default for SubjectTypeChoice {
    fn default() -> SubjectTypeChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for SubjectTypeChoice {}

// Bezugsobjekt
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/message/01p10")]
pub struct SubjectType {
    #[yaserde(flatten)]
    pub subject_type_choice: SubjectTypeChoice,
}

impl Validate for SubjectType {}




// Prozessrolle
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct InfoTypeType (pub String);

impl Validate for InfoTypeType {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 20 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 20 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Netz-Rechnungsnumer
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct InvoiceNumberType (pub String);

impl Validate for InvoiceNumberType {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 20 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 20 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Schlagwort
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct MessageTextTag (pub String);

impl Validate for MessageTextTag {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 100 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 100 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

pub fn read_message_01p10(file_read : &Path) -> Option<Message>{
  if let Ok(src_file) = File::open(file_read){
    let _data: Message = yaserde::de::from_reader(BufReader::new(src_file)).unwrap();
    return Some(_data)
  }
  None
}