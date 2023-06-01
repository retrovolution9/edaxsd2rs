
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
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cpdocument/01p12")]
pub struct Cpdocument {
    #[yaserde(prefix = "ns0", rename = "MarketParticipantDirectory")]
    pub market_participant_directory: MarketParticipantDirectory,

    #[yaserde(prefix = "ns0", rename = "ProcessDirectory")]
    pub process_directory: ProcessDirectory,
}

impl Validate for Cpdocument {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cpdocument/01p12")]
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
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cpdocument/01p12")]
pub struct ProcessDirectory {
    #[yaserde(prefix = "ns0", rename = "VerificationDocument")]
    pub verification_document: VerificationDocument,

    #[yaserde(prefix = "ns0", rename = "AdditionalData")]
    pub additional_data: Vec<ns1::AdditionalData>,
}

impl Validate for ProcessDirectory {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns0", namespace = "ns0: http://www.ebutilities.at/schemata/customerprocesses/cpdocument/01p12")]
pub struct VerificationDocument {
    #[yaserde(prefix = "ns0", rename = "DOCNumber")]
    pub doc_number: Docnumber,

    #[yaserde(prefix = "ns0", rename = "DOCCategory")]
    pub doc_category: Doccategory,

    #[yaserde(prefix = "ns0", rename = "DOCOwner")]
    pub doc_owner: Option<Docowner>,

    #[yaserde(prefix = "ns0", rename = "DOCAuthentificationMethod")]
    pub doc_authentification_method: Option<DocauthentificationMethod>,

    #[yaserde(prefix = "ns0", rename = "DOCAuthentificationDescription")]
    pub doc_authentification_description: Option<DocauthentificationDescription>,

    #[yaserde(prefix = "ns0", rename = "DOCSignatureDate")]
    pub doc_signature_date: Option<DocsignatureDate>,

    #[yaserde(prefix = "ns0", rename = "DOCValidUntil")]
    pub doc_valid_until: Option<DocvalidUntil>,

    #[yaserde(prefix = "ns0", rename = "DOCUrl")]
    pub doc_url: Option<Docurl>,

    #[yaserde(prefix = "ns0", rename = "DOCDescription")]
    pub doc_description: Option<Docdescription>,

    #[yaserde(prefix = "ns0", rename = "DOCExtension")]
    pub doc_extension: Option<Docextension>,

    #[yaserde(prefix = "ns0", rename = "DOCFile")]
    pub doc_file: Option<String>,
}

impl Validate for VerificationDocument {}


// Authentifizierungsverfahren
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DocauthentificationMethod (pub u8);

impl Validate for DocauthentificationMethod {}
// Authentifizierungsverfahren bei "Sonstige"
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DocauthentificationDescription (pub String);

impl Validate for DocauthentificationDescription {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 120 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 120 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Dokumentenkategorie
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Doccategory (pub String);

impl Validate for Doccategory {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 10 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 10 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Dokumenten-Beschreibung
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Docdescription (pub String);

impl Validate for Docdescription {
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

// Dokument-File Endung
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Docextension (pub String);

impl Validate for Docextension {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() > 10 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 10 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

// Dokumenten-ID
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Docnumber (pub String);

impl Validate for Docnumber {
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

// Dokumenteneigner
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Docowner (pub String);

impl Validate for Docowner {}
// Zeichnungsdatum
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DocsignatureDate (pub xs::Date);

impl Validate for DocsignatureDate {}
// Verweis auf Dokument (z.B. Ediktsdatei, Impressum…)
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Docurl (pub String);

impl Validate for Docurl {}
// Gültig bis
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DocvalidUntil (pub xs::Date);

impl Validate for DocvalidUntil {}
pub fn read_cpdocument_01p12(file_read : &Path) -> Option<Cpdocument>{
  if let Ok(src_file) = File::open(file_read){
    let _data: Cpdocument = yaserde::de::from_reader(BufReader::new(src_file)).unwrap();
    return Some(_data)
  }
  None
}
pub fn write_cpdocument_01p12(file_write : &Path, data :&Cpdocument) -> Result<(),String>
{
 
    if let Ok(src_file) = File::create(file_write) {  
    let config: Config = Config {
        perform_indent: true,
        write_document_declaration: true,
        indent_string: None,
    };        
    if let Ok(mut content) = yaserde::ser::to_string_with_config(data, &config) {
    content = content.replace("xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/cpdocument/01p12","xmlns:ns0=\"http://www.ebutilities.at/schemata/customerprocesses/cpdocument/01p12xmlns:ns1=\"http://www.ebutilities.at/schemata/customerprocesses/common/types/01p20\""); 
        let mut bw = BufWriter::new(src_file);
        if let Ok(_write_ok) = bw.write_all(content.as_bytes()) {
            return Ok(());
        }
    }        
    return Err("error serialize content".to_string());
}
Err("can't create file".to_string())


}
