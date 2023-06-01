
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
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/meteringpointlist/01p20")]
pub struct MeteringPointList {
    #[yaserde(prefix = "ns0", rename = "MarketParticipantDirectory")]
    pub market_participant_directory: MarketParticipantDirectory,

    #[yaserde(prefix = "ns0", rename = "ProcessDirectory")]
    pub process_directory: ProcessDirectory,
}

impl Validate for MeteringPointList {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/meteringpointlist/01p20")]
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
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/meteringpointlist/01p20")]
pub struct ProcessDirectory {
    // Nachrichtennummer
    #[yaserde(prefix = "ns0", rename = "MessageId")]
    pub message_id: ns1::GroupingId,

    // Prozessnummer
    #[yaserde(prefix = "ns0", rename = "ConversationId")]
    pub conversation_id: ns1::GroupingId,

    // Prozessdatum
    #[yaserde(prefix = "ns0", rename = "ProcessDate")]
    pub process_date: xs::Date,

    // Anzahl der (Teil)Meldungen innerhalb der gleichen ConversationId
    #[yaserde(prefix = "ns0", rename = "NumberOfMessages")]
    pub number_of_messages: NumberOfMessages,

    // Nummer der Meldung innerhalb der gleichen ConversationId
    #[yaserde(prefix = "ns0", rename = "CurrentMessageNumber")]
    pub current_message_number: CurrentMessageNumber,

    // Zählpunktsdaten
    #[yaserde(prefix = "ns0", rename = "MeteringPointListData")]
    pub metering_point_list_data: Vec<MeteringPointListData>,
}

impl Validate for ProcessDirectory {}


// Daten der Zählpunkte
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/meteringpointlist/01p20")]
pub struct MeteringPointListData {
    // Zählpunkt
    #[yaserde(prefix = "ns0", rename = "MeteringPoint")]
    pub metering_point: ns1::MeteringPoint,

    // Prognostizierter Jahresverbrauch
    #[yaserde(prefix = "ns0", rename = "ForecastConsumption")]
    pub forecast_consumption: Option<ForecastConsumption>,

    // Lastprofiltyp
    #[yaserde(prefix = "ns0", rename = "LoadProfileType")]
    pub load_profile_type: LoadProfileType,

    // Typ des Zählers
    #[yaserde(prefix = "ns0", rename = "DeviceType")]
    pub device_type: DeviceType,

    // Versorgt seit
    #[yaserde(prefix = "ns0", rename = "DateFrom")]
    pub date_from: xs::Date,

    // Versorgt bis
    #[yaserde(prefix = "ns0", rename = "DateTo")]
    pub date_to: xs::Date,
}

impl Validate for MeteringPointListData {}


// Nummer der Meldung innerhalb einer ConversionId
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct CurrentMessageNumber (pub xs::Integer);

impl Validate for CurrentMessageNumber {}
// Zählertyp
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/meteringpointlist/01p20")]

pub enum DeviceType {
    #[yaserde(rename = "NONSMART")]
    Nonsmart,
    #[yaserde(rename = "IMN")]
    Imn,
    #[yaserde(rename = "DSZ")]
    Dsz,
    #[yaserde(rename = "IMS")]
    Ims,
    #[yaserde(rename = "IME")]
    Ime,
    #[yaserde(rename = "LPZ")]
    Lpz,
    #[yaserde(rename = "PAUSCHAL")]
    Pauschal,
    __Unknown__(String),
}

impl Default for DeviceType {
    fn default() -> DeviceType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DeviceType {}



// Prognostizierter Jahresverbrauch
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ForecastConsumption (pub xs::Decimal);

impl Validate for ForecastConsumption {}
// Lastprofiltyp (inkl. Temperaturzone bei Gas)
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct LoadProfileType (pub String);

impl Validate for LoadProfileType {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 10 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 10 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Anzahl der Meldungen innerhalb einer ConversionId
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct NumberOfMessages (pub xs::Integer);

impl Validate for NumberOfMessages {}
pub fn read_meteringpointlist_01p20(file_read : &Path) -> Option<MeteringPointList>{
  if let Ok(src_file) = File::open(file_read){
    let _data: MeteringPointList = yaserde::de::from_reader(BufReader::new(src_file)).unwrap();
    return Some(_data)
  }
  None
}
pub fn write_meteringpointlist_01p20(file_write : &Path, data :&MeteringPointList) -> Result<(),String>
{
 
    if let Ok(src_file) = File::create(file_write) {  
    let config: Config = Config {
        perform_indent: true,
        write_document_declaration: true,
        indent_string: None,
    };        
    if let Ok(mut content) = yaserde::ser::to_string_with_config(data, &config) {
    content = content.replace("xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/meteringpointlist/01p20","xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/meteringpointlist/01p20xmlns:ns1=\"http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20\""); 
        let mut bw = BufWriter::new(src_file);
        if let Ok(_write_ok) = bw.write_all(content.as_bytes()) {
            return Ok(());
        }
    }        
    return Err("error serialize content".to_string());
}
Err("can't create file".to_string())


}
