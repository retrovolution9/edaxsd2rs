
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
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/consumptionrecord/01p30")]
pub struct ConsumptionRecord {
    #[yaserde(prefix = "cp", rename = "MarketParticipantDirectory")]
    pub market_participant_directory: MarketParticipantDirectory,

    #[yaserde(prefix = "cp", rename = "ProcessDirectory")]
    pub process_directory: ProcessDirectory,
}

impl Validate for ConsumptionRecord {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/consumptionrecord/01p30")]
pub struct MarketParticipantDirectory {
    // DATEN_CRMSG
    #[yaserde(prefix = "cp", rename = "MessageCode")]
    pub message_code: market_participant_directory::MessageCodeType,

    #[yaserde(prefix = "ct", rename = "RoutingHeader")]
    pub routing_header: ct::RoutingHeader,

    #[yaserde(prefix = "ct", rename = "Sector")]
    pub sector: ct::Sector,

    #[yaserde(attribute, rename = "DocumentMode")]
    pub document_mode: ct::DocumentMode,

    #[yaserde(attribute, rename = "Duplicate")]
    pub duplicate: bool,

    #[yaserde(attribute, rename = "SchemaVersion")]
    pub schema_version: ConsumptionRecordVersion,
}

impl Validate for MarketParticipantDirectory {}

pub mod market_participant_directory {
    use super::*;
    
    #[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
    pub struct MessageCodeType (pub ct::MessageCode);

    impl Validate for MessageCodeType {}

}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/consumptionrecord/01p30")]
pub struct ProcessDirectory {
    // Lieferanschrift (EC-Nummer)
    #[yaserde(prefix = "cp", rename = "DeliveryPoint")]
    pub delivery_point: Option<ct::MessageAddress>,

    #[yaserde(prefix = "ct", rename = "MessageId")]
    pub message_id: ct::GroupingId,

    #[yaserde(prefix = "ct", rename = "ConversationId")]
    pub conversation_id: ct::GroupingId,

    #[yaserde(prefix = "ct", rename = "ProcessDate")]
    pub process_date: xs::Date,

    #[yaserde(prefix = "ct", rename = "MeteringPoint")]
    pub metering_point: ct::MeteringPoint,

    #[yaserde(prefix = "cp", rename = "Energy")]
    pub energy: Vec<Energy>,
}

impl Validate for ProcessDirectory {}


// Energie-Mengen
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/consumptionrecord/01p30")]
pub struct Energy {
    #[yaserde(prefix = "cp", rename = "MeteringReason")]
    pub metering_reason: MeteringReason,

    #[yaserde(prefix = "cp", rename = "MeteringPeriodStart")]
    pub metering_period_start: DateTimeU,

    #[yaserde(prefix = "cp", rename = "MeteringPeriodEnd")]
    pub metering_period_end: DateTimeU,

    #[yaserde(prefix = "cp", rename = "MeteringIntervall")]
    pub metering_intervall: MeteringIntervall,

    #[yaserde(prefix = "cp", rename = "NumberOfMeteringIntervall")]
    pub number_of_metering_intervall: xs::Integer,

    #[yaserde(prefix = "cp", rename = "EnergyData")]
    pub energy_data: Vec<EnergyData>,
}

impl Validate for Energy {}


// Energie-Positionen
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/consumptionrecord/01p30")]
pub struct EnergyData {
    #[yaserde(prefix = "cp", rename = "EP")]
    pub ep: Vec<EnergyPosition>,

    // OBIS-Code
    #[yaserde(attribute, rename = "MeterCode")]
    pub meter_code: String,

    // Einheit
    #[yaserde(attribute, rename = "UOM")]
    pub uom: Uomtype,
}

impl Validate for EnergyData {}


// Energie-Position
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/consumptionrecord/01p30")]
pub struct EnergyPosition {
    // Datum/Zeit von
    #[yaserde(prefix = "cp", rename = "DTF")]
    pub dtf: DateTimeU,

    // Datum/Zeit bis
    #[yaserde(prefix = "cp", rename = "DTT")]
    pub dtt: Option<DateTimeU>,

    // Ableseart
    #[yaserde(prefix = "cp", rename = "MM")]
    pub mm: Option<MeteringMethod>,

    // Menge
    #[yaserde(prefix = "cp", rename = "BQ")]
    pub bq: BillingQuantity,
}

impl Validate for EnergyPosition {}


// Verbrauchsmenge Dezimalzahl -10.6
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct BillingQuantity (pub xs::Decimal);

impl Validate for BillingQuantity {}
// Zuständigkeit
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Competence (pub String);

impl Validate for Competence {
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

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ConsumptionRecordMessageCode (pub MessageCode);

impl Validate for ConsumptionRecordMessageCode {}
// Schema-Version
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ConsumptionRecordVersion (pub String);

impl Validate for ConsumptionRecordVersion {}
// Email-Adresse
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Email (pub String);

impl Validate for Email {
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

// Faxnummer
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Fax (pub String);

impl Validate for Fax {
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

// Messintervall
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/consumptionrecord/01p30")]

pub enum MeteringIntervall {
    #[yaserde(rename = "QH")]
    Qh,
    H,
    D,
    V,
    __Unknown__(String),
}

impl Default for MeteringIntervall {
    fn default() -> MeteringIntervall {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for MeteringIntervall {}



// Ableseart
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/consumptionrecord/01p30")]

pub enum MeteringMethod {
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
    L1,
    L2,
    L3,
    __Unknown__(String),
}

impl Default for MeteringMethod {
    fn default() -> MeteringMethod {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for MeteringMethod {}



// Ablesegrund
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/consumptionrecord/01p30")]

pub enum MeteringReason {
    #[yaserde(rename = "00")]
    _00,
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
    __Unknown__(String),
}

impl Default for MeteringReason {
    fn default() -> MeteringReason {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for MeteringReason {}



// Name
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Name (pub String);

impl Validate for Name {
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

// Telefonnummer
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Phone (pub String);

impl Validate for Phone {
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

// Referenznummer (z.B: Debitorenkontnummer, Vertragskontonummer)
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ReferenceNumber (pub String);

impl Validate for ReferenceNumber {
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

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/consumptionrecord/01p30")]

pub enum Uomtype {
    #[yaserde(rename = "PROZ")]
    Proz,
    #[yaserde(rename = "CELS")]
    Cels,
    #[yaserde(rename = "PCE")]
    Pce,
    #[yaserde(rename = "EUR")]
    Eur,
    #[yaserde(rename = "MB")]
    Mb,
    #[yaserde(rename = "GB")]
    Gb,
    #[yaserde(rename = "TB")]
    Tb,
    H,
    #[yaserde(rename = "TAG")]
    Tag,
    #[yaserde(rename = "MIN")]
    Min,
    #[yaserde(rename = "MON")]
    Mon,
    #[yaserde(rename = "KVARH")]
    Kvarh,
    #[yaserde(rename = "MVARH")]
    Mvarh,
    #[yaserde(rename = "KWT")]
    Kwt,
    #[yaserde(rename = "MWT")]
    Mwt,
    #[yaserde(rename = "GWT")]
    Gwt,
    #[yaserde(rename = "KWH")]
    Kwh,
    #[yaserde(rename = "MWH")]
    Mwh,
    #[yaserde(rename = "GWH")]
    Gwh,
    #[yaserde(rename = "LE")]
    Le,
    M2,
    M3,
    #[yaserde(rename = "BM3")]
    Bm3,
    #[yaserde(rename = "NM3")]
    Nm3,
    #[yaserde(rename = "BM3H")]
    Bm3H,
    #[yaserde(rename = "NM3H")]
    Nm3H,
    #[yaserde(rename = "KWHH")]
    Kwhh,
    #[yaserde(rename = "PAU")]
    Pau,
    __Unknown__(String),
}

impl Default for Uomtype {
    fn default() -> Uomtype {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Uomtype {}



// Datum Uhrzeit Format JJJJ-MM-TT"T"HH:MM:SS
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DateTimeS (pub xs::DateTime);

impl Validate for DateTimeS {}
// Datum Uhrzeit (Sekunden immer 00) mit UNC 2001-12-17T09:30:00+01:00
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DateTimeU (pub xs::DateTime);

impl Validate for DateTimeU {}
// Produktiv-/Testkennzeichen
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/consumptionrecord/01p30")]

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

pub fn read_consumptionrecord_01p30(file_read : &Path) -> Option<ConsumptionRecord>{
  if let Ok(src_file) = File::open(file_read){
    let _data: ConsumptionRecord = yaserde::de::from_reader(BufReader::new(src_file)).unwrap();
    return Some(_data)
  }
  None
}


pub fn write_consumptionrecord_01p30( file_write: &Path, data : &ConsumptionRecord ) -> Result<(),String>
{
    if let Ok(src_file) = File::create(file_write) {  
        let config: Config = Config {
            perform_indent: true,
            write_document_declaration: true,
            indent_string: None,
        };
        match yaserde::ser::serialize_with_writer(data, BufWriter::new(src_file), &config) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err),
        }
    }
    Err("can't create file".to_string())
}