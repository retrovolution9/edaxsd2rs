
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

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ct", namespace = "ct: http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20")]
pub struct AdditionalData {
    #[yaserde(attribute, rename = "Name")]
    pub name: AdditionalDataName,
}

impl Validate for AdditionalData {}


// Marktteilnehmer-Daten
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ct", namespace = "ct: http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20")]
pub struct MarketParticipantDirectory {
    #[yaserde(prefix = "ct", rename = "RoutingHeader")]
    pub routing_header: RoutingHeader,

    #[yaserde(prefix = "ct", rename = "Sector")]
    pub sector: Sector,

    #[yaserde(attribute, rename = "DocumentMode")]
    pub document_mode: DocumentMode,

    #[yaserde(attribute, rename = "Duplicate")]
    pub duplicate: bool,
}

impl Validate for MarketParticipantDirectory {}


// Prozessdaten
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ct", namespace = "ct: http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20")]
pub struct ProcessDirectory {
    #[yaserde(prefix = "ct", rename = "MessageId")]
    pub message_id: GroupingId,

    #[yaserde(prefix = "ct", rename = "ConversationId")]
    pub conversation_id: GroupingId,

    #[yaserde(prefix = "ct", rename = "ProcessDate")]
    pub process_date: xs::Date,

    #[yaserde(prefix = "ct", rename = "MeteringPoint")]
    pub metering_point: MeteringPoint,
}

impl Validate for ProcessDirectory {}


// Prozessdaten Kurz
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ct", namespace = "ct: http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20")]
pub struct ProcessDirectoryS {
    #[yaserde(prefix = "ct", rename = "MessageId")]
    pub message_id: GroupingId,

    #[yaserde(prefix = "ct", rename = "ConversationId")]
    pub conversation_id: GroupingId,
}

impl Validate for ProcessDirectoryS {}


// Routing-Adresse
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ct", namespace = "ct: http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20")]
pub struct RoutingAddress {
    #[yaserde(prefix = "ct", rename = "MessageAddress")]
    pub message_address: MessageAddress,

    #[yaserde(attribute, rename = "AddressType")]
    pub address_type: AddressType,
}

impl Validate for RoutingAddress {}


// Routing Header
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ct", namespace = "ct: http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20")]
pub struct RoutingHeader {
    #[yaserde(prefix = "ct", rename = "Sender")]
    pub sender: RoutingAddress,

    #[yaserde(prefix = "ct", rename = "Receiver")]
    pub receiver: RoutingAddress,

    #[yaserde(prefix = "ct", rename = "DocumentCreationDateTime")]
    pub document_creation_date_time: xs::DateTime,
}

impl Validate for RoutingHeader {}


// Nachweis-Dokument
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ct", namespace = "ct: http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20")]
pub struct VerificationDocument {
    #[yaserde(prefix = "ct", rename = "DOCNumber")]
    pub doc_number: Docnumber,
}

impl Validate for VerificationDocument {}


// Zusätzliche Daten - Name
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct AdditionalDataName (pub String);

impl Validate for AdditionalDataName {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 40 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 40 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Zusätzliche Daten - Inhalt
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct AdditionalDataValue (pub String);

impl Validate for AdditionalDataValue {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 120 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 120 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Adressierungsart (ECNumber, Other)
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ct", namespace = "ct: http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20")]

pub enum AddressType {
    #[yaserde(rename = "ECNumber")]
    Ecnumber,
    Other,
    __Unknown__(String),
}

impl Default for AddressType {
    fn default() -> AddressType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for AddressType {}



// Datum Uhrzeit Format JJJJ-MM-TT"T"HH:MM:SS
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DateTimeS (pub xs::DateTime);

impl Validate for DateTimeS {}
// Datum Uhrzeit (Sekunden immer 00) mit UNC 2001-12-17T09:30:00+01:00
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DateTimeU (pub xs::DateTime);

impl Validate for DateTimeU {}
// Dokumenten-ID
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Docnumber (pub String);

impl Validate for Docnumber {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 35 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 35 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Produktiv-/Testkennzeichen
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ct", namespace = "ct: http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20")]

pub enum DocumentMode {
    #[yaserde(rename = "PROD")]
    Prod,
    #[yaserde(rename = "SIMU")]
    Simu,
    __Unknown__(String),
}

impl Default for DocumentMode {
    fn default() -> DocumentMode {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DocumentMode {}



// Nachrichten- bzw. Prozessnummer
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct GroupingId (pub String);

impl Validate for GroupingId {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 35 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 35 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Adresse des Teilnehmers (Sender / Empfänger)
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct MessageAddress (pub String);

impl Validate for MessageAddress {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 35 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 35 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Nachrichten-Code
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct MessageCode (pub String);

impl Validate for MessageCode {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 20 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 20 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Zählpunkt
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct MeteringPoint (pub String);

impl Validate for MeteringPoint {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 33 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 33 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Sparte
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ct", namespace = "ct: http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20")]

pub enum Sector {
    #[yaserde(rename = "01")]
    _01,
    #[yaserde(rename = "02")]
    _02,
    #[yaserde(rename = "03")]
    _03,
    #[yaserde(rename = "04")]
    _04,
    #[yaserde(rename = "05")]
    _05,
    #[yaserde(rename = "06")]
    _06,
    #[yaserde(rename = "07")]
    _07,
    #[yaserde(rename = "08")]
    _08,
    #[yaserde(rename = "09")]
    _09,
    #[yaserde(rename = "99")]
    _99,
    __Unknown__(String),
}

impl Default for Sector {
    fn default() -> Sector {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Sector {}



