
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
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct MasterData {
    #[yaserde(prefix = "ns0", rename = "MarketParticipantDirectory")]
    pub market_participant_directory: MarketParticipantDirectory,

    #[yaserde(prefix = "ns0", rename = "ProcessDirectory")]
    pub process_directory: ProcessDirectory,
}

impl Validate for MasterData {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct MarketParticipantDirectory {
    #[yaserde(prefix = "ns0", rename = "MessageCode")]
    pub message_code: ns1::MessageCode,

    #[yaserde(attribute, rename = "SchemaVersion")]
    pub schema_version: String,
}

impl Validate for MarketParticipantDirectory {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct ProcessDirectory {
    #[yaserde(prefix = "ns0", rename = "ContractPartner")]
    pub contract_partner: Option<ContractPartner>,

    #[yaserde(prefix = "ns0", rename = "DeliveryAddress")]
    pub delivery_address: Option<DeliveryAddress>,

    #[yaserde(prefix = "ns0", rename = "BillingData")]
    pub billing_data: Option<BillingData>,

    #[yaserde(prefix = "ns0", rename = "MeteringPointData")]
    pub metering_point_data: Option<MeteringPointData>,

    #[yaserde(prefix = "ns0", rename = "InvoiceRecipient")]
    pub invoice_recipient: Option<InvoiceRecipient>,

    #[yaserde(prefix = "ns0", rename = "AdditionalData")]
    pub additional_data: Vec<ns1::AdditionalData>,

    #[yaserde(prefix = "ns0", rename = "VerificationDocument")]
    pub verification_document: Option<ns1::VerificationDocument>,
}

impl Validate for ProcessDirectory {}


// Adresse
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct Address {
    #[yaserde(prefix = "ns0", rename = "ZIP")]
    pub zip: Zipc,

    #[yaserde(prefix = "ns0", rename = "City")]
    pub city: CityC,

    #[yaserde(prefix = "ns0", rename = "POBoxNo")]
    pub po_box_no: Option<PoboxNoC>,

    #[yaserde(prefix = "ns0", rename = "Street")]
    pub street: Option<StreetC>,

    #[yaserde(prefix = "ns0", rename = "StreetNo")]
    pub street_no: Option<StreetNoC>,

    #[yaserde(prefix = "ns0", rename = "Staircase")]
    pub staircase: Option<StaircaseC>,

    #[yaserde(prefix = "ns0", rename = "Floor")]
    pub floor: Option<FloorC>,

    #[yaserde(prefix = "ns0", rename = "DoorNumber")]
    pub door_number: Option<DoorNumberC>,
}

impl Validate for Address {}


// Abrechnungsdaten
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct BillingData {
    #[yaserde(prefix = "ns0", rename = "ReferenceNumber")]
    pub reference_number: Option<ReferenceNumber>,

    #[yaserde(prefix = "ns0", rename = "GridInvoiceRecipient")]
    pub grid_invoice_recipient: GridInvoiceRecipientC,

    #[yaserde(prefix = "ns0", rename = "BudgetBillingCycle")]
    pub budget_billing_cycle: Option<BudgetBillingCycleC>,

    #[yaserde(prefix = "ns0", rename = "MeterReadingMonth")]
    pub meter_reading_month: Option<Months0C>,

    #[yaserde(prefix = "ns0", rename = "ConsumptionBillingCycle")]
    pub consumption_billing_cycle: Option<ConsumptionBillingCycleC>,

    #[yaserde(prefix = "ns0", rename = "ConsumptionBillingMonth")]
    pub consumption_billing_month: Option<Months0C>,

    #[yaserde(prefix = "ns0", rename = "YearMonthOfNextBill")]
    pub year_month_of_next_bill: Option<YearMonth>,
}

impl Validate for BillingData {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct BudgetBillingCycleC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for BudgetBillingCycleC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct CityC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for CityC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct ConsumptionBillingCycleC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for ConsumptionBillingCycleC {}


// Kundendaten
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct ContractPartner {
    #[yaserde(prefix = "ns0", rename = "Salutation")]
    pub salutation: Option<Salutation>,

    #[yaserde(prefix = "ns0", rename = "Name1")]
    pub name_1: NameC,

    #[yaserde(prefix = "ns0", rename = "Name2")]
    pub name_2: Option<NameC>,

    #[yaserde(prefix = "ns0", rename = "Name3")]
    pub name_3: Option<NameC>,

    #[yaserde(prefix = "ns0", rename = "Name4")]
    pub name_4: Option<NameC>,

    #[yaserde(prefix = "ns0", rename = "ContractPartnerNumber")]
    pub contract_partner_number: Option<ContractPartnerNumber>,

    #[yaserde(prefix = "ns0", rename = "DateOfBirth")]
    pub date_of_birth: Option<xs::Date>,

    #[yaserde(prefix = "ns0", rename = "DateOfDeath")]
    pub date_of_death: Option<xs::Date>,

    #[yaserde(prefix = "ns0", rename = "CompanyRegistryNo")]
    pub company_registry_no: Option<CompanyRegistryNo>,

    #[yaserde(prefix = "ns0", rename = "VATNumber")]
    pub vat_number: Option<Vatnumber>,

    #[yaserde(prefix = "ns0", rename = "Email")]
    pub email: Option<Email>,
}

impl Validate for ContractPartner {}


// Lieferadresse
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct DeliveryAddress {
    #[yaserde(prefix = "ns0", rename = "ZIP")]
    pub zip: Zipc,

    #[yaserde(prefix = "ns0", rename = "City")]
    pub city: CityC,

    #[yaserde(prefix = "ns0", rename = "Street")]
    pub street: StreetC,

    #[yaserde(prefix = "ns0", rename = "StreetNo")]
    pub street_no: StreetNoC,

    #[yaserde(prefix = "ns0", rename = "Staircase")]
    pub staircase: Option<StaircaseC>,

    #[yaserde(prefix = "ns0", rename = "Floor")]
    pub floor: Option<FloorC>,

    #[yaserde(prefix = "ns0", rename = "DoorNumber")]
    pub door_number: Option<DoorNumberC>,

    #[yaserde(prefix = "ns0", rename = "DeliveryAddressData")]
    pub delivery_address_data: Option<DeliveryAddressDataC>,
}

impl Validate for DeliveryAddress {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct DeliveryAddressDataC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for DeliveryAddressDataC {}


// Gerät
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct Device {
    #[yaserde(prefix = "ns0", rename = "DeviceNumber")]
    pub device_number: DeviceNumberC,

    #[yaserde(prefix = "ns0", rename = "MeterCode")]
    pub meter_code: Vec<MeterCode>,
}

impl Validate for Device {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct DeviceNumberC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for DeviceNumberC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct DeviceTypeC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for DeviceTypeC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct DoorNumberC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for DoorNumberC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct ElectricityGridLevelC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for ElectricityGridLevelC {}


// stromspezifische ZP-Daten
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct ElectricitySpecificData {
    #[yaserde(prefix = "ns0", rename = "GridUsageLevel")]
    pub grid_usage_level: ElectricityGridLevelC,

    #[yaserde(prefix = "ns0", rename = "GridLossLevel")]
    pub grid_loss_level: ElectricityGridLevelC,
}

impl Validate for ElectricitySpecificData {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct FloorC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for FloorC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct GasGridLevelC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for GasGridLevelC {}


// gasspezifische ZP-Daten
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct GasSpecificData {
    #[yaserde(prefix = "ns0", rename = "PeakPower")]
    pub peak_power: PeakPowerC,

    #[yaserde(prefix = "ns0", rename = "GridUsageLevel")]
    pub grid_usage_level: GasGridLevelC,
}

impl Validate for GasSpecificData {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct GridInvoiceRecipientC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for GridInvoiceRecipientC {}


// Rechnungsempfänger
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct InvoiceRecipient {
    #[yaserde(prefix = "ns0", rename = "PartnerData")]
    pub partner_data: ContractPartner,

    #[yaserde(prefix = "ns0", rename = "AddressData")]
    pub address_data: Address,
}

impl Validate for InvoiceRecipient {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct LoadProfileTypeC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for LoadProfileTypeC {}


// ZP-Daten
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct MeteringPointData {
    #[yaserde(prefix = "ns0", rename = "DeviceType")]
    pub device_type: DeviceTypeC,

    #[yaserde(prefix = "ns0", rename = "TransmissionCycle")]
    pub transmission_cycle: Option<TransmissionCycleC>,

    #[yaserde(prefix = "ns0", rename = "Device")]
    pub device: Vec<Device>,

    #[yaserde(prefix = "ns0", rename = "SupStatus")]
    pub sup_status: SupStatus,

    #[yaserde(prefix = "ns0", rename = "DSOTariffClass")]
    pub dso_tariff_class: DsotariffClassC,

    #[yaserde(prefix = "ns0", rename = "EnergyDirection")]
    pub energy_direction: EnergyDirection,

    #[yaserde(prefix = "ns0", rename = "EnergyCommunity")]
    pub energy_community: EnergyCommunityC,

    #[yaserde(prefix = "ns0", rename = "TypeOfGeneration")]
    pub type_of_generation: TypeOfGenerationC,

    #[yaserde(prefix = "ns0", rename = "ShortageCapacity")]
    pub shortage_capacity: Option<ShortageCapacityC>,

    #[yaserde(prefix = "ns0", rename = "ForecastConsumption")]
    pub forecast_consumption: ForecastConsumption,

    #[yaserde(prefix = "ns0", rename = "SupplyOfLastResort")]
    pub supply_of_last_resort: SupplyOfLastResort,

    #[yaserde(prefix = "ns0", rename = "LoadProfileType")]
    pub load_profile_type: LoadProfileTypeC,

    #[yaserde(prefix = "ns0", rename = "MeteringPointDataChoice")]
    pub metering_point_data_choice: metering_point_data::MeteringPointDataChoice,
}

impl Validate for MeteringPointData {}

pub mod metering_point_data {
    use super::*;
    
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]

    pub enum MeteringPointDataChoice {
        ElectricitySpecificData(Option<ElectricitySpecificData>),
        GasSpecificData(Option<GasSpecificData>),
        __Unknown__(String),
    }

    impl Default for MeteringPointDataChoice {
        fn default() -> MeteringPointDataChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    impl Validate for MeteringPointDataChoice {}


}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct DsotariffClassC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for DsotariffClassC {}


// Daten einer Energiegemeinschaft
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct Ecdata {
    // Energiegemeinschafts-ID
    #[yaserde(prefix = "ns0", rename = "ECID")]
    pub ecid: ns1::MeteringPoint,

    // Verteilungsmodell
    #[yaserde(prefix = "ns0", rename = "ECPartitionModell")]
    pub ec_partition_modell: PartitionModell,

    // Anteil bei statischem Modell von Energiegemeinschaften in Prozent
    #[yaserde(prefix = "ns0", rename = "ECShare")]
    pub ec_share: Option<Ecshare>,
}

impl Validate for Ecdata {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct EnergyCommunityC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for EnergyCommunityC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct Months0C {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for Months0C {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct NameC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for NameC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct PeakPowerC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for PeakPowerC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct PoboxNoC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for PoboxNoC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct ShortageCapacityC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for ShortageCapacityC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct StaircaseC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for StaircaseC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct StreetC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for StreetC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct StreetNoC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for StreetNoC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct TransmissionCycleC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for TransmissionCycleC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct TypeOfGenerationC {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for TypeOfGenerationC {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]
pub struct Zipc {
    #[yaserde(attribute, rename = "Changed")]
    pub changed: bool,
}

impl Validate for Zipc {}


// Abschlagszyklus
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]

pub enum BudgetBillingCycle {
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
    #[yaserde(rename = "06")]
    _06,
    #[yaserde(rename = "12")]
    _12,
    __Unknown__(String),
}

impl Default for BudgetBillingCycle {
    fn default() -> BudgetBillingCycle {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for BudgetBillingCycle {}



// Ort
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct City (pub String);

impl Validate for City {
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

// Firmenbuchnummer
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct CompanyRegistryNo (pub String);

impl Validate for CompanyRegistryNo {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 14 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 14 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Abrechnungszyklus
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]

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



// Kundennummer
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ContractPartnerNumber (pub String);

impl Validate for ContractPartnerNumber {
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

// Adresszusatz
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DeliveryAddressData (pub String);

impl Validate for DeliveryAddressData {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 255 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 255 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Zählernummer
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DeviceNumber (pub String);

impl Validate for DeviceNumber {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 18 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 18 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Zählertyp
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]

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



// Türnummer
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DoorNumber (pub String);

impl Validate for DoorNumber {
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

// Tarifklasse Netzbetreiber (lt. SNVo)
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]

pub enum DsotariffClass {
    G,
    #[yaserde(rename = "GD")]
    Gd,
    N,
    #[yaserde(rename = "ND")]
    Nd,
    U,
    #[yaserde(rename = "UD")]
    Ud,
    E,
    __Unknown__(String),
}

impl Default for DsotariffClass {
    fn default() -> DsotariffClass {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DsotariffClass {}



// Anteil bei statischem Modell in Prozent
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Ecshare (pub xs::Decimal);

impl Validate for Ecshare {
    fn validate(&self) -> Result<(), String> { 
        if self.0 < "0.0001".parse::<xs::Decimal>().unwrap() {
            return Err(format!("MinInclusive validation error: invalid value of 0! \nExpected: 0 >= 0.0001.\nActual: 0 == {}", self.0));
        }
        if self.0 > "100".parse::<xs::Decimal>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 100.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

// Strom Netzebene
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ElectricityGridLevel (pub u8);

impl Validate for ElectricityGridLevel {
    fn validate(&self) -> Result<(), String> { 
        if self.0 > "7".parse::<u8>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 7.\nActual: 0 == {}", self.0));
        }
        if self.0 < "1".parse::<u8>().unwrap() {
            return Err(format!("MinInclusive validation error: invalid value of 0! \nExpected: 0 >= 1.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

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

// Energiegemeinschaft
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct EnergyCommunity (pub String);

impl Validate for EnergyCommunity {}
// Energierichtung (Erzeuger/Verbraucher)
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]

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



// Stock
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Floor (pub String);

impl Validate for Floor {
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

// Prognostizierter Jahresverbrauch
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ForecastConsumption (pub xs::Decimal);

impl Validate for ForecastConsumption {}
// Gas Netzebene
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct GasGridLevel (pub u8);

impl Validate for GasGridLevel {
    fn validate(&self) -> Result<(), String> { 
        if self.0 > "3".parse::<u8>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 3.\nActual: 0 == {}", self.0));
        }
        if self.0 < "1".parse::<u8>().unwrap() {
            return Err(format!("MinInclusive validation error: invalid value of 0! \nExpected: 0 >= 1.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

// Netzrechnung an
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]

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

// Zählwerkskennung
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct MeterCode (pub String);

impl Validate for MeterCode {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 25 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 25 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Monat; 0=monatlich; 1-12=entsprechender Monat; Angabe nur einer Zahl möglich
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Months0 (pub u8);

impl Validate for Months0 {
    fn validate(&self) -> Result<(), String> { 
        if self.0 > "12".parse::<u8>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 12.\nActual: 0 == {}", self.0));
        }
        if self.0 < "0".parse::<u8>().unwrap() {
            return Err(format!("MinInclusive validation error: invalid value of 0! \nExpected: 0 >= 0.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

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

// Teilnahmemodell
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]

pub enum PartitionModell {
    #[yaserde(rename = "DYNAMIC")]
    Dynamic,
    #[yaserde(rename = "STATIC")]
    Static,
    #[yaserde(rename = "INDIVIDUAL")]
    Individual,
    __Unknown__(String),
}

impl Default for PartitionModell {
    fn default() -> PartitionModell {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for PartitionModell {}



// Gas Höchstleistung in kWh/h
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct PeakPower (pub xs::Decimal);

impl Validate for PeakPower {}
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

// Anrede
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Salutation (pub String);

impl Validate for Salutation {
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

// Engpassleistung Einspeiseanlagen in kW
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ShortageCapacity (pub xs::Decimal);

impl Validate for ShortageCapacity {}
// Stiege
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Staircase (pub String);

impl Validate for Staircase {
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

// Strasse
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Street (pub String);

impl Validate for Street {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 60 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 60 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Hausnummer
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct StreetNo (pub String);

impl Validate for StreetNo {
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

// Grundversorgung (Y/N)
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct SupplyOfLastResort (pub bool);

impl Validate for SupplyOfLastResort {}
// Versorgungsstatus
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]

pub enum SupStatus {
    #[yaserde(rename = "ON")]
    On,
    #[yaserde(rename = "OFF")]
    Off,
    __Unknown__(String),
}

impl Default for SupStatus {
    fn default() -> SupStatus {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for SupStatus {}



// Übertragungsintervall Verbrauchsdaten
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]

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



// Voll-/Überschusseinspeiser
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31")]

pub enum TypeOfGeneration {
    #[yaserde(rename = "NONE")]
    None,
    #[yaserde(rename = "FULL")]
    Full,
    #[yaserde(rename = "SURPLUS")]
    Surplus,
    __Unknown__(String),
}

impl Default for TypeOfGeneration {
    fn default() -> TypeOfGeneration {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for TypeOfGeneration {}



// USt-ID
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Vatnumber (pub String);

impl Validate for Vatnumber {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 14 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 14 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// JahrMonat YYYYMM
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct YearMonth (pub String);

impl Validate for YearMonth {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() != 6 {
            return Err(format!("Length validation error. \nExpected: 0 length == 6 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Postleitzahl
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Zip (pub String);

impl Validate for Zip {
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

pub fn read_masterdata_01p31(file_read : &Path) -> Option<MasterData>{
  if let Ok(src_file) = File::open(file_read){
    let _data: MasterData = yaserde::de::from_reader(BufReader::new(src_file)).unwrap();
    return Some(_data)
  }
  None
}
pub fn write_masterdata_01p31(file_write : &Path, data :&MasterData) -> Result<(),String>
{
 
    if let Ok(src_file) = File::create(file_write) {  
    let config: Config = Config {
        perform_indent: true,
        write_document_declaration: true,
        indent_string: None,
    };        
    if let Ok(mut content) = yaserde::ser::to_string_with_config(data, &config) {
    content = content.replace("xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31","xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/masterdata/01p31xmlns:ns1=\"http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20\""); 
        let mut bw = BufWriter::new(src_file);
        if let Ok(_write_ok) = bw.write_all(content.as_bytes()) {
            return Ok(());
        }
    }        
    return Err("error serialize content".to_string());
}
Err("can't create file".to_string())


}
