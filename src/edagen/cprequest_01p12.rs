
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
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]
pub struct Cprequest {
    #[yaserde(prefix = "ns0", rename = "MarketParticipantDirectory")]
    pub market_participant_directory: MarketParticipantDirectory,

    #[yaserde(prefix = "ns0", rename = "ProcessDirectory")]
    pub process_directory: ProcessDirectory,
}

impl Validate for Cprequest {}


// Request: Erweiterungen
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]
pub struct Extension {
    #[yaserde(prefix = "ns0", rename = "GridInvoiceRecipient")]
    pub grid_invoice_recipient: Option<GridInvoiceRecipient>,

    #[yaserde(prefix = "ns0", rename = "ConsumptionBillingCycle")]
    pub consumption_billing_cycle: Option<ConsumptionBillingCycle>,

    #[yaserde(prefix = "ns0", rename = "TransmissionCycle")]
    pub transmission_cycle: Option<TransmissionCycle>,

    #[yaserde(prefix = "ns0", rename = "MeteringIntervall")]
    pub metering_intervall: Option<MeteringIntervall>,

    #[yaserde(prefix = "ns0", rename = "LoadProfileType")]
    pub load_profile_type: Option<LoadProfileType>,

    #[yaserde(prefix = "ns0", rename = "DateTimeFrom")]
    pub date_time_from: Option<ns1::DateTimeU>,

    #[yaserde(prefix = "ns0", rename = "DateTimeTo")]
    pub date_time_to: Option<ns1::DateTimeU>,

    #[yaserde(prefix = "ns0", rename = "DisconnectionReason")]
    pub disconnection_reason: Option<DisconnectionReason>,

    #[yaserde(prefix = "ns0", rename = "EmailCustomer")]
    pub email_customer: Option<Email>,

    #[yaserde(prefix = "ns0", rename = "AssumptionOfCosts")]
    pub assumption_of_costs: bool,
}

impl Validate for Extension {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]
pub struct MarketParticipantDirectory {
    #[yaserde(prefix = "ns1", rename = "RoutingHeader")]
    pub routing_header: ns1::RoutingHeader,

    #[yaserde(prefix = "ns1", rename = "Sector")]
    pub sector: ns1::Sector,

    #[yaserde(attribute, rename = "DocumentMode")]
    pub document_mode: ns1::DocumentMode,

    #[yaserde(attribute, rename = "Duplicate")]
    pub duplicate: bool,
    
    #[yaserde(prefix = "ns0", rename = "MessageCode")]
    pub message_code: ns1::MessageCode,

    #[yaserde(attribute, rename = "SchemaVersion")]
    pub schema_version: String,
}

impl Validate for MarketParticipantDirectory {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]
pub struct ProcessDirectory {
    #[yaserde(prefix = "ns1", rename = "MessageId")]
    pub message_id: ns1::GroupingId,

    #[yaserde(prefix = "ns1", rename = "ConversationId")]
    pub conversation_id: ns1::GroupingId,

    #[yaserde(prefix = "ns1", rename = "ProcessDate")]
    pub process_date: xs::Date,

    #[yaserde(prefix = "ns1", rename = "MeteringPoint")]
    pub metering_point: ns1::MeteringPoint,

    #[yaserde(prefix = "ns0", rename = "Extension")]
    pub extension: Option<Extension>,

    #[yaserde(prefix = "ns0", rename = "AdditionalData")]
    pub additional_data: Vec<ns1::AdditionalData>,

    #[yaserde(prefix = "ns0", rename = "VerificationDocument")]
    pub verification_document: Option<ns1::VerificationDocument>,
}

impl Validate for ProcessDirectory {}


// Abrechnungszyklus
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]

pub enum ConsumptionBillingCycle {
    #[yaserde(rename = "01")]
    _01,
    #[yaserde(rename = "02")]
    _02,
    #[yaserde(rename = "03")]
    _03,
    #[yaserde(rename = "04")]
    _04,
    #[yaserde(rename = "06")]
    _06,
    #[yaserde(rename = "12")]
    _12,
    __Unknown__(String),
}

impl Default for ConsumptionBillingCycle {
    fn default() -> ConsumptionBillingCycle {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ConsumptionBillingCycle {}



// Messintervall
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]

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



// Sperrgrund
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]

pub enum DisconnectionReason {
    #[yaserde(rename = "01")]
    _01,
    #[yaserde(rename = "02")]
    _02,
    __Unknown__(String),
}

impl Default for DisconnectionReason {
    fn default() -> DisconnectionReason {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DisconnectionReason {}



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

// Netzrechnung an
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]

pub enum GridInvoiceRecipient {
    #[yaserde(rename = "CUSTOMER")]
    Customer,
    #[yaserde(rename = "SUPPLIER")]
    Supplier,
    __Unknown__(String),
}

impl Default for GridInvoiceRecipient {
    fn default() -> GridInvoiceRecipient {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for GridInvoiceRecipient {}



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

// Übertragungsintervall Verbrauchsdaten
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]

pub enum TransmissionCycle {
    D,
    M,
    __Unknown__(String),
}

impl Default for TransmissionCycle {
    fn default() -> TransmissionCycle {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for TransmissionCycle {}



pub fn read_cprequest_01p12(file_read : &Path) -> Option<Cprequest>{
  if let Ok(src_file) = File::open(file_read){
    let _data: Cprequest = yaserde::de::from_reader(BufReader::new(src_file)).unwrap();
    return Some(_data)
  }
  None
}
pub fn write_cprequest_01p12(file_write : &Path, data :&Cprequest) -> Result<(),String>
{
 
    if let Ok(src_file) = File::create(file_write) {  
    let config: Config = Config {
        perform_indent: true,
        write_document_declaration: true,
        indent_string: None,
    };        
    if let Ok(mut content) = yaserde::ser::to_string_with_config(data, &config) {
    content = content.replace("xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12"," xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12\" xmlns:ns1=\"http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20"); 
        let mut bw = BufWriter::new(src_file);
        if let Ok(_write_ok) = bw.write_all(content.as_bytes()) {
            return Ok(());
        }
    }        
    return Err("error serialize content".to_string());
}
Err("can't create file".to_string())


}
