
use std::{path::{Path, PathBuf}};

use super::edagen::consumptionrecord_01p30::{ConsumptionRecord as ConsumptionRecord_01p30, read_consumptionrecord_01p30, write_consumptionrecord_01p30};
use super::edagen::consumptionrecord_01p31::{ConsumptionRecord as ConsumptionRecord_01p31, read_consumptionrecord_01p31};
use super::edagen::cmrequest_01p10::{Cmrequest as Cmrequest_1p10, read_cmrequest_01p10};
use super::edagen::cmrevoke_01p00::{Cmrevoke as Cmrevoke_1p00, read_cmrevoke_01p00};
use super::edagen::cpnotification_01p13::{Cpnotification as Cpnotification_1p13, read_cpnotification_01p13};
use super::edagen::cprequest_01p12::{Cprequest as Cprequest_1p12, read_cprequest_01p12};
use super::edagen::ecmplist_01p00::{Ecmplist as EcmpList_1p00, read_ecmplist_01p00};
use super::edagen::masterdata_01p31::{MasterData as MasterData_1p31, read_masterdata_01p31};
use super::edagen::message_01p10::{Message as Message_1p10, read_message_01p10};


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


pub fn get_eda_info( edatyp: &EdaRecordTyp, file_path: &Path ) -> String
{
    let info: String = Default::default();
    match edatyp {
        EdaRecordTyp::ConsumptionRecord1p30 => {            
            println!("identified as: ConsumptionRecord1p30");
            print!( "prepare for read : ");
            if let Some(_data) = read_consumptionrecord_01p30(file_path) {
                println!("succeded");
            }
        },
        EdaRecordTyp::ConsumptionRecord1p31 => {
            println!("identified as: ConsumptionRecord1p31");
            print!( "prepare for read : ");
            if let Some(_data) = read_consumptionrecord_01p31(file_path) {
                println!("succeded");
            }
        },
        EdaRecordTyp::Cmrequest1p10 => {
            println!("identified as: Cmrequest1p10");
            print!( "prepare for read : ");
            if let Some(_data) = read_cmrequest_01p10(file_path){
                println!("succeded");
            }
        },
        EdaRecordTyp::Cmrevoke1p00 => {
            println!("identified as: Cmrevoke1p00");
            print!( "prepare for read : ");
            if let Some(_data) = read_cmrevoke_01p00(file_path){
                println!("succeded");
            }
        },
        EdaRecordTyp::CpNotification1p13 => {
            println!("identified as: CpNotification1p13");
            print!( "prepare for read : ");
            if let Some(_data) = read_cpnotification_01p13(file_path){
                println!("succeded");
            }
        },
        EdaRecordTyp::CpRequest1p12 => {
            println!("identified as: Cmrevoke1p00");
            print!( "prepare for read : ");
            if let Some(_data) = read_cprequest_01p12(file_path){
                println!("succeded");
            }
        },
        EdaRecordTyp::EcmpList1p00 => {
            println!("identified as: EcmpList1p00");
            print!( "prepare for read : ");
            if let Some(_data) = read_ecmplist_01p00(file_path){
                println!("Read succeded");
            }
        },
        EdaRecordTyp::MasterData1p31 => {
            println!("identified as: MasterData1p31");
            print!( "prepare for read : ");
            if let Some(_data) = read_masterdata_01p31(file_path){
                println!("Read succeded");
            }
        },
        EdaRecordTyp::Message1p10 => {
            println!("identified as: Message1p10");
            print!( "prepare for read : ");
            if let Some(_data) = read_message_01p10(file_path){
                println!("Read succeded");
            }
        },
        _ =>(),
    }
    info
}

pub fn test_readwrite( edatyp: &EdaRecordTyp, file_path: &Path ) -> String
{
    let file_write = change_file_name(file_path, &("write_".to_string() + &file_path.file_name().unwrap().to_string_lossy()));
    let info: String = Default::default();
    match edatyp {
        EdaRecordTyp::ConsumptionRecord1p30 => {            
            println!("identified as: ConsumptionRecord1p30");
            print!( "prepare for read : ");
            if let Some(data) = read_consumptionrecord_01p30(file_path) {
                println!("succeded");
                
                print!( "prepare for write : ");
                if let Ok(_) = write_consumptionrecord_01p30(file_write.as_path(), &data) {
                    println!("succeded");
                }
            }
        },
        EdaRecordTyp::ConsumptionRecord1p31 => {
            println!("identified as: ConsumptionRecord1p31");
            print!( "prepare for read : ");
            if let Some(_data) = read_consumptionrecord_01p31(file_path) {
                println!("succeded");
            }
        },
        EdaRecordTyp::Cmrequest1p10 => {
            println!("identified as: Cmrequest1p10");
            print!( "prepare for read : ");
            if let Some(_data) = read_cmrequest_01p10(file_path){
                println!("succeded");
            }
        },
        EdaRecordTyp::Cmrevoke1p00 => {
            println!("identified as: Cmrevoke1p00");
            print!( "prepare for read : ");
            if let Some(_data) = read_cmrevoke_01p00(file_path){
                println!("succeded");
            }
        },
        EdaRecordTyp::CpNotification1p13 => {
            println!("identified as: CpNotification1p13");
            print!( "prepare for read : ");
            if let Some(_data) = read_cpnotification_01p13(file_path){
                println!("succeded");
            }
        },
        EdaRecordTyp::CpRequest1p12 => {
            println!("identified as: Cmrevoke1p00");
            print!( "prepare for read : ");
            if let Some(_data) = read_cprequest_01p12(file_path){
                println!("succeded");
            }
        },
        EdaRecordTyp::EcmpList1p00 => {
            println!("identified as: EcmpList1p00");
            print!( "prepare for read : ");
            if let Some(_data) = read_ecmplist_01p00(file_path){
                println!("Read succeded");
            }
        },
        EdaRecordTyp::MasterData1p31 => {
            println!("identified as: MasterData1p31");
            print!( "prepare for read : ");
            if let Some(_data) = read_masterdata_01p31(file_path){
                println!("Read succeded");
            }
        },
        EdaRecordTyp::Message1p10 => {
            println!("identified as: Message1p10");
            print!( "prepare for read : ");
            if let Some(_data) = read_message_01p10(file_path){
                println!("Read succeded");
            }
        },
        _ =>(),
    }
    info
}

fn change_file_name(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    result.set_file_name(name);
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    result
}