
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
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]
pub struct Ecmplist {
    #[yaserde(prefix = "ns0", rename = "MarketParticipantDirectory")]
    pub market_participant_directory: MarketParticipantDirectory,

    #[yaserde(prefix = "ns0", rename = "ProcessDirectory")]
    pub process_directory: ProcessDirectory,
}

impl Validate for Ecmplist {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]
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
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]
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

    // Gemeinschafts-ID
    #[yaserde(prefix = "ns0", rename = "ECID")]
    pub ecid: ns1::MeteringPoint,

    // Typ der Energiegemeinschaft
    #[yaserde(prefix = "ns0", rename = "ECType")]
    pub ec_type: Ectype,

    // Verteilmodell (dynamisch/statisch)
    #[yaserde(prefix = "ns0", rename = "ECDisModel")]
    pub ec_dis_model: EcdisModel,

    // Zählpunktsdaten
    #[yaserde(prefix = "ns0", rename = "MPListData")]
    pub mp_list_data: Vec<MplistData>,
}

impl Validate for ProcessDirectory {}


// Daten der Zählpunkte
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]
pub struct MplistData {
    // Zählpunkt
    #[yaserde(prefix = "ns0", rename = "MeteringPoint")]
    pub metering_point: ns1::MeteringPoint,

    // zeitabhängige Daten
    #[yaserde(prefix = "ns0", rename = "MPTimeData")]
    pub mp_time_data: Vec<MptimeData>,
}

impl Validate for MplistData {}


// Daten der Zählpunkte
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]
pub struct MptimeData {
    // gültig seit
    #[yaserde(prefix = "ns0", rename = "DateFrom")]
    pub date_from: xs::Date,

    // gültig bis
    #[yaserde(prefix = "ns0", rename = "DateTo")]
    pub date_to: xs::Date,

    // Energierichtung
    #[yaserde(prefix = "ns0", rename = "EnergyDirection")]
    pub energy_direction: EnergyDirection,

    // Energieträger
    #[yaserde(prefix = "ns0", rename = "PlantCategory")]
    pub plant_category: Option<PlantCategory>,

    // Versorgt seit
    #[yaserde(prefix = "ns0", rename = "DateActivate")]
    pub date_activate: xs::Date,

    // Versorgt bis
    #[yaserde(prefix = "ns0", rename = "DateDeactivate")]
    pub date_deactivate: Option<xs::Date>,

    // gemeldeter Prozentanteil bei statischem Modell
    #[yaserde(prefix = "ns0", rename = "ECShare")]
    pub ec_share: Option<Ecshare>,

    // berechneter Prozentanteil bei statischem Modell bei Gesamt-Übersteigung
    // von 100%
    #[yaserde(prefix = "ns0", rename = "ECShC")]
    pub ec_sh_c: Vec<ShareCalc>,
}

impl Validate for MptimeData {}


// Daten der Zählpunkte
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]
pub struct ShareCalc {
    // gültig seit
    #[yaserde(prefix = "ns0", rename = "DateFrom")]
    pub date_from: xs::Date,

    // gültig bis
    #[yaserde(prefix = "ns0", rename = "DateTo")]
    pub date_to: xs::Date,

    // berechneter Prozentanteil
    #[yaserde(prefix = "ns0", rename = "ECShareCalc")]
    pub ec_share_calc: Ecshare,
}

impl Validate for ShareCalc {}


// Verteilmodell (dynamisch/statisch)
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]

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
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00")]

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
pub fn write_ecmplist_01p00(file_write : &Path, data :&Ecmplist) -> Result<(),String>
{
 
    if let Ok(src_file) = File::create(file_write) {  
    let config: Config = Config {
        perform_indent: true,
        write_document_declaration: true,
        indent_string: None,
    };        
    if let Ok(mut content) = yaserde::ser::to_string_with_config(data, &config) {
    content = content.replace("xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00","xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/ecmplist/01p00xmlns:ns1=\"http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20\""); 
        let mut bw = BufWriter::new(src_file);
        if let Ok(_write_ok) = bw.write_all(content.as_bytes()) {
            return Ok(());
        }
    }        
    return Err("error serialize content".to_string());
}
Err("can't create file".to_string())


}
