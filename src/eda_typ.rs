
pub use super::edagen::consumptionrecord_01p30::{ConsumptionRecord as ConsumptionRecord_01p30, read_consumptionrecord_01p30, write_consumptionrecord_01p30};
pub use super::edagen::consumptionrecord_01p31::{ConsumptionRecord as ConsumptionRecord_01p31, read_consumptionrecord_01p31};
pub use super::edagen::cmrequest_01p10::{Cmrequest as Cmrequest_1p10, read_cmrequest_01p10, write_cmrequest_01p10};
pub use super::edagen::cmrevoke_01p00::{Cmrevoke as Cmrevoke_1p00, read_cmrevoke_01p00, write_cmrevoke_01p00};
pub use super::edagen::cpnotification_01p13::{Cpnotification as Cpnotification_1p13, read_cpnotification_01p13, write_cpnotification_01p13};
pub use super::edagen::cprequest_01p12::{Cprequest as Cprequest_1p12, Extension as Cprequest_1p12_Extension, read_cprequest_01p12, write_cprequest_01p12};
pub use super::edagen::ecmplist_01p00::{Ecmplist as EcmpList_1p00, read_ecmplist_01p00, write_ecmplist_01p00};
pub use super::edagen::masterdata_01p31::{MasterData as MasterData_1p31, read_masterdata_01p31, write_masterdata_01p31};
pub use super::edagen::message_01p10::{Message as Message_1p10, read_message_01p10, write_message_01p10};
pub use super::edagen::cpcommontypes_01p20::{  
  MessageAddress as MessageAddress_01p20, 
  AddressType as AddressType_01p20, 
  Sector as Sector_01p20,
  DocumentMode as DocumentMode_01p20,
};


// EDA Record Typ
pub enum EdaRecordTyp {
  Invalid,
  Unknown,
  ConsumptionRecord1p30,
  ConsumptionRecord1p31,
  Cmrequest1p10,
  Cmrevoke1p00,
  CpNotification1p13,
  CpRequest1p12,
  EcmpList1p00,
  MasterData1p31,
  Message1p10,        
}

