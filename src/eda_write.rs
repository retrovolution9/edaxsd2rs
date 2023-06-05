
use std::{path::{Path, PathBuf}};
//use chrono::{NaiveDate, Duration, Datelike, DateTime, FixedOffset};
use xsd_types::types::DateTime;

use super::eda_typ::{ EdaRecordTyp ,
  read_consumptionrecord_01p30,
  read_consumptionrecord_01p31,
  read_cmrequest_01p10,
  read_cmrevoke_01p00,
  read_cpnotification_01p13,
  Cprequest_1p12, Cprequest_1p12_Extension,
  read_cprequest_01p12, 
  write_cprequest_01p12,
  EcmpList_1p00,
  read_ecmplist_01p00,
  write_ecmplist_01p00,
  read_masterdata_01p31,
  read_message_01p10,
  MessageAddress_01p20,AddressType_01p20,Sector_01p20
};

pub fn eda_write_default( process : &Option<&String>)
{
    if process.is_none()
    {

    }
    else {
      match process.unwrap().as_str() {
          "EC_PODLIST" => {
              print!( "prepare for write EC_PODLIST: ");
              {
                let mut d1 : Cprequest_1p12 = Default::default();
                d1.market_participant_directory.message_code.0 = "ANFORDERUNG_ECP".to_string();
                d1.market_participant_directory.routing_header.sender.address_type = AddressType_01p20::Ecnumber;
                d1.market_participant_directory.routing_header.sender.message_address.0 = "RC100300".to_string();
                d1.market_participant_directory.routing_header.receiver.address_type = AddressType_01p20::Ecnumber;
                d1.market_participant_directory.routing_header.receiver.message_address.0 = "AT008000".to_string();
                d1.market_participant_directory.sector = Sector_01p20::_01;
                d1.process_directory.conversation_id.0 = "AT008000202306011444047420000001805".to_string();
                d1.process_directory.message_id.0 = "RC100300202306011444050620000001806".to_string();
                d1.process_directory.metering_point.0 = "AT00800000000RC100300000000000108".to_string();
                let mut ext : Cprequest_1p12_Extension = Default::default();
                ext.assumption_of_costs = false;
                d1.process_directory.extension = Some(ext);

                let dt = chrono::offset::Local::now();
                d1.process_directory.process_date.value = dt.naive_local().into();

                
                d1.market_participant_directory.routing_header.document_creation_date_time.value = dt.into();
                let file_write = Path::new("EC_PODLIST_CPRequest_1p12.xml");
                write_cprequest_01p12(&file_write, &d1);
              }
          },
          _ =>(),
      }
  }
}

