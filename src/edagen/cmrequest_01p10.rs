
// for deserialize xml files
use std::str::FromStr;
use xsd_parser::generator::validator::Validate;
use xsd_types::types as xs;
use xsd_macro_utils::{UtilsDefaultSerde,UtilsTupleIo};
use yaserde::ser::Config;

// common types
use super::cpcommontypes_01p20 as ct;

// for read/write functions
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufWriter};

//use CPCommonTypes_01p20.xsd  http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerconsent/cmrequest/01p10")]
pub struct Cmrequest {
    #[yaserde(prefix = "cp", rename = "MarketParticipantDirectory")]
    pub market_participant_directory: MarketParticipantDirectory,

    #[yaserde(prefix = "cp", rename = "ProcessDirectory")]
    pub process_directory: ProcessDirectory,
}

impl Validate for Cmrequest {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerconsent/cmrequest/01p10")]
pub struct MarketParticipantDirectory {
    // Nachricht (nicht im Schema geprüft)
    #[yaserde(prefix = "cp", rename = "MessageCode")]
    pub message_code: ct::MessageCode,

    #[yaserde(attribute, rename = "SchemaVersion")]
    pub schema_version: String,
}

impl Validate for MarketParticipantDirectory {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerconsent/cmrequest/01p10")]
pub struct ProcessDirectory {
    #[yaserde(prefix = "cp", rename = "ProcessDate")]
    pub process_date: xs::Date,

    #[yaserde(prefix = "cp", rename = "MeteringPoint")]
    pub metering_point: Option<ct::MeteringPoint>,

    // Anforderungs-ID
    #[yaserde(prefix = "cp", rename = "CMRequestId")]
    pub cm_request_id: ct::GroupingId,

    // Zustimmungs-ID
    #[yaserde(prefix = "cp", rename = "ConsentId")]
    pub consent_id: Option<ct::GroupingId>,

    // Anforderungs-Details
    #[yaserde(prefix = "cp", rename = "CMRequest")]
    pub cm_request: ReqType,
}

impl Validate for ProcessDirectory {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerconsent/cmrequest/01p10")]
pub struct ReqType {
    // Datentypen der Anfrage
    #[yaserde(prefix = "cp", rename = "ReqDatType")]
    pub req_dat_type: ReqDatType,

    // Beginndatum für Versand/Datenzugriff
    #[yaserde(prefix = "cp", rename = "DateFrom")]
    pub date_from: xs::Date,

    // Endedatum für Versand/Datenzugriff
    #[yaserde(prefix = "cp", rename = "DateTo")]
    pub date_to: Option<xs::Date>,

    // Messintervall/Granularität
    #[yaserde(prefix = "cp", rename = "MeteringIntervall")]
    pub metering_intervall: Option<MeteringIntervallType>,

    // Übertragungsintervall
    #[yaserde(prefix = "cp", rename = "TransmissionCycle")]
    pub transmission_cycle: Option<TransmissionCycle>,

    // Energiegemeinschafts-ID
    #[yaserde(prefix = "cp", rename = "ECID")]
    pub ecid: Option<ct::MeteringPoint>,

    // Anteil bei statischem Modell von Energiegemeinschaften in Prozent
    #[yaserde(prefix = "cp", rename = "ECShare")]
    pub ec_share: Option<Ecshare>,

    // Energierichtung
    #[yaserde(prefix = "cp", rename = "EnergyDirection")]
    pub energy_direction: Option<EnergyDirection>,
}

impl Validate for ReqType {}


#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerconsent/cmrequest/01p10")]

pub enum ReqDatParamTypeChoice {
    ParamCyc(ParamCycType),
    ParamHist(ParamHistType),
    __Unknown__(String),
}

impl Default for ReqDatParamTypeChoice {
    fn default() -> ReqDatParamTypeChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ReqDatParamTypeChoice {}

// Anforderungsdaten
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerconsent/cmrequest/01p10")]
pub struct ReqDatParamType {
    #[yaserde(flatten)]
    pub req_dat_param_type_choice: ReqDatParamTypeChoice,
}

impl Validate for ReqDatParamType {}




// Parameter für Historischen Versand
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerconsent/cmrequest/01p10")]
pub struct ParamHistType {
    // Messintervall/Granularität
    #[yaserde(prefix = "cp", rename = "MeteringIntervall")]
    pub metering_intervall: MeteringIntervallType,
}

impl Validate for ParamHistType {}


// Parameter für Zyklischen Versand
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerconsent/cmrequest/01p10")]
pub struct ParamCycType {
    // Messintervall/Granularität
    #[yaserde(prefix = "cp", rename = "MeteringIntervall")]
    pub metering_intervall: MeteringIntervallType,

    // Übertragungsintervall
    #[yaserde(prefix = "cp", rename = "TransmissionCycle")]
    pub transmission_cycle: TransmissionCycle,
}

impl Validate for ParamCycType {}


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

// Energierichtung (Erzeuger/Verbraucher)
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerconsent/cmrequest/01p10")]

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



// Messintervall
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerconsent/cmrequest/01p10")]

pub enum MeteringIntervallType {
    #[yaserde(rename = "QH")]
    Qh,
    H,
    D,
    V,
    __Unknown__(String),
}

impl Default for MeteringIntervallType {
    fn default() -> MeteringIntervallType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for MeteringIntervallType {}



// Datentypen der Anfrage
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ReqDatType (pub String);

impl Validate for ReqDatType {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 30 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 30 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Übertragungsintervall Verbrauchsdaten
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerconsent/cmrequest/01p10")]

pub enum TransmissionCycle {
    D,
    M,
    V,
    __Unknown__(String),
}

impl Default for TransmissionCycle {
    fn default() -> TransmissionCycle {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for TransmissionCycle {}



pub fn read_cmrequest_01p10(file_read : &Path) -> Option<Cmrequest>{
  if let Ok(src_file) = File::open(file_read){
    let _data: Cmrequest = yaserde::de::from_reader(BufReader::new(src_file)).unwrap();
    return Some(_data)
  }
  None
}

pub fn write_cm_request_01p10( file_write: &Path, cm_equest : &Cmrequest ) -> Result<(),String>
{
    if let Ok(src_file) = File::open(file_write) {  
        let config: Config = Config {
            perform_indent: true,
            write_document_declaration: true,
            indent_string: None,
        };
        match yaserde::ser::serialize_with_writer(cm_equest, BufWriter::new(src_file), &config) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err),
        }
    }
    Err("can't create file".to_string())
}