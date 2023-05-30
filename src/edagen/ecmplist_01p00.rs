
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
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]
pub struct Ecmplist {
    #[yaserde(prefix = "cp", rename = "MarketParticipantDirectory")]
    pub market_participant_directory: MarketParticipantDirectory,

    #[yaserde(prefix = "cp", rename = "ProcessDirectory")]
    pub process_directory: ProcessDirectory,
}

impl Validate for Ecmplist {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]
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
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]
pub struct ProcessDirectory {
    // Nachrichtennummer
    #[yaserde(prefix = "cp", rename = "MessageId")]
    pub message_id: ct::GroupingId,

    // Prozessnummer
    #[yaserde(prefix = "cp", rename = "ConversationId")]
    pub conversation_id: ct::GroupingId,

    // Prozessdatum
    #[yaserde(prefix = "cp", rename = "ProcessDate")]
    pub process_date: xs::Date,

    // Gemeinschafts-ID
    #[yaserde(prefix = "cp", rename = "ECID")]
    pub ecid: ct::MeteringPoint,

    // Typ der Energiegemeinschaft
    #[yaserde(prefix = "cp", rename = "ECType")]
    pub ec_type: Ectype,

    // Verteilmodell (dynamisch/statisch)
    #[yaserde(prefix = "cp", rename = "ECDisModel")]
    pub ec_dis_model: EcdisModel,

    // Zählpunktsdaten
    #[yaserde(prefix = "cp", rename = "MPListData")]
    pub mp_list_data: Vec<MplistData>,
}

impl Validate for ProcessDirectory {}


// Daten der Zählpunkte
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]
pub struct MplistData {
    // Zählpunkt
    #[yaserde(prefix = "cp", rename = "MeteringPoint")]
    pub metering_point: ct::MeteringPoint,

    // zeitabhängige Daten
    #[yaserde(prefix = "cp", rename = "MPTimeData")]
    pub mp_time_data: Vec<MptimeData>,
}

impl Validate for MplistData {}


// Daten der Zählpunkte
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]
pub struct MptimeData {
    // gültig seit
    #[yaserde(prefix = "cp", rename = "DateFrom")]
    pub date_from: xs::Date,

    // gültig bis
    #[yaserde(prefix = "cp", rename = "DateTo")]
    pub date_to: xs::Date,

    // Energierichtung
    #[yaserde(prefix = "cp", rename = "EnergyDirection")]
    pub energy_direction: EnergyDirection,

    // Energieträger
    #[yaserde(prefix = "cp", rename = "PlantCategory")]
    pub plant_category: Option<PlantCategory>,

    // Versorgt seit
    #[yaserde(prefix = "cp", rename = "DateActivate")]
    pub date_activate: xs::Date,

    // Versorgt bis
    #[yaserde(prefix = "cp", rename = "DateDeactivate")]
    pub date_deactivate: Option<xs::Date>,

    // gemeldeter Prozentanteil bei statischem Modell
    #[yaserde(prefix = "cp", rename = "ECShare")]
    pub ec_share: Option<Ecshare>,

    // berechneter Prozentanteil bei statischem Modell bei Gesamt-Übersteigung
    // von 100%
    #[yaserde(prefix = "cp", rename = "ECShC")]
    pub ec_sh_c: Vec<ShareCalc>,
}

impl Validate for MptimeData {}


// Daten der Zählpunkte
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]
pub struct ShareCalc {
    // gültig seit
    #[yaserde(prefix = "cp", rename = "DateFrom")]
    pub date_from: xs::Date,

    // gültig bis
    #[yaserde(prefix = "cp", rename = "DateTo")]
    pub date_to: xs::Date,

    // berechneter Prozentanteil
    #[yaserde(prefix = "cp", rename = "ECShareCalc")]
    pub ec_share_calc: Ecshare,
}

impl Validate for ShareCalc {}


// Verteilmodell (dynamisch/statisch)
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]

pub enum EcdisModel {
    D,
    S,
    __Unknown__(String),
}

impl Default for EcdisModel {
    fn default() -> EcdisModel {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for EcdisModel {}



// Anteil bei statischem Modell in Prozent
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Ecshare (pub xs::Decimal);

impl Validate for Ecshare {
    fn validate(&self) -> Result<(), String> { 
        if self.0 < "0".parse::<xs::Decimal>().unwrap() {
            return Err(format!("MinInclusive validation error: invalid value of 0! \nExpected: 0 >= 0.\nActual: 0 == {}", self.0));
        }
        if self.0 > "100".parse::<xs::Decimal>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 100.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

// Zählertyp
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Ectype (pub String);

impl Validate for Ectype {}
// Energierichtung (Erzeuger/Verbraucher)
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]

pub enum EnergyDirection {
    #[yaserde(rename = "CONSUMPTION")]
    Consumption,
    #[yaserde(rename = "GENERATION")]
    Generation,
    __Unknown__(String),
}

impl Default for EnergyDirection {
    fn default() -> EnergyDirection {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for EnergyDirection {}



// Energieträger
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct PlantCategory (pub String);

impl Validate for PlantCategory {
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

pub fn read_ecmplist_01p00(file_read : &Path) -> Option<Ecmplist>{
  if let Ok(src_file) = File::open(file_read){
    let _data: Ecmplist = yaserde::de::from_reader(BufReader::new(src_file)).unwrap();
    return Some(_data)
  }
  None
}