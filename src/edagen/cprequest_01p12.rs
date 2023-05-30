
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
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]
pub struct Cprequest {
    #[yaserde(prefix = "cp", rename = "MarketParticipantDirectory")]
    pub market_participant_directory: MarketParticipantDirectory,

    #[yaserde(prefix = "cp", rename = "ProcessDirectory")]
    pub process_directory: ProcessDirectory,
}

impl Validate for Cprequest {}


// Request: Erweiterungen
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]
pub struct Extension {
    #[yaserde(prefix = "cp", rename = "GridInvoiceRecipient")]
    pub grid_invoice_recipient: Option<GridInvoiceRecipient>,

    #[yaserde(prefix = "cp", rename = "ConsumptionBillingCycle")]
    pub consumption_billing_cycle: Option<ConsumptionBillingCycle>,

    #[yaserde(prefix = "cp", rename = "TransmissionCycle")]
    pub transmission_cycle: Option<TransmissionCycle>,

    #[yaserde(prefix = "cp", rename = "MeteringIntervall")]
    pub metering_intervall: Option<MeteringIntervall>,

    #[yaserde(prefix = "cp", rename = "LoadProfileType")]
    pub load_profile_type: Option<LoadProfileType>,

    #[yaserde(prefix = "cp", rename = "DateTimeFrom")]
    pub date_time_from: Option<ct::DateTimeU>,

    #[yaserde(prefix = "cp", rename = "DateTimeTo")]
    pub date_time_to: Option<ct::DateTimeU>,

    #[yaserde(prefix = "cp", rename = "DisconnectionReason")]
    pub disconnection_reason: Option<DisconnectionReason>,

    #[yaserde(prefix = "cp", rename = "EmailCustomer")]
    pub email_customer: Option<Email>,

    #[yaserde(prefix = "cp", rename = "AssumptionOfCosts")]
    pub assumption_of_costs: bool,
}

impl Validate for Extension {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]
pub struct MarketParticipantDirectory {
    #[yaserde(prefix = "cp", rename = "MessageCode")]
    pub message_code: ct::MessageCode,

    #[yaserde(attribute, rename = "SchemaVersion")]
    pub schema_version: String,
}

impl Validate for MarketParticipantDirectory {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]
pub struct ProcessDirectory {
    #[yaserde(prefix = "cp", rename = "Extension")]
    pub extension: Option<Extension>,

    #[yaserde(prefix = "cp", rename = "AdditionalData")]
    pub additional_data: Vec<ct::AdditionalData>,

    #[yaserde(prefix = "cp", rename = "VerificationDocument")]
    pub verification_document: Option<ct::VerificationDocument>,
}

impl Validate for ProcessDirectory {}


// Abrechnungszyklus
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]

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
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]

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
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]

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
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]

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
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "cp", namespace = "cp: http://www.ebutilities.at/schemata/customerprocesses/cprequest/01p12")]

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