//tutorial-read-01.rs
#![allow(unused_imports)]
#![allow(dead_code)]
use std::env;
use std::io;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;
//#[serde(deserialize_with = "csv::invalid_option")]

extern crate csv;
extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
#[macro_use]
extern crate serde_derive;

// EL CSV DEBE ESTAR SEPARADO POR ","
#[derive(Debug, Deserialize, Serialize,Clone)]
//#[serde(rename_all = "PascalCase")]

struct RecordIn {
    Legajo : String,
    Sector :Option<u64>,
    Cuit : String,
    #[serde(rename = "Tipodoc")]
    Tipo_Doc: String,
    Nrodoc : String,
    Atraso : String,
    Tipocarte : String,
    Estado : String,
    Estadoina : String,
    Comcorr :u32,
    Totali :u32,
    #[serde(rename = "Salvenc")]
    Saldo_Vencido :u32,
    Informacio : String,
    //population: Option<u64>,
}

struct RecordOut {
    Legajo : String,
    Sector :u64,
    Cuit : String,
    Tipo_Doc: String,
    Nrodoc : String,
    Atraso : String,
    Tipocarte : String,
    Estado : String,
    Estadoina : String,
    Comcorr :u32,
    Totali :u32,
    Saldo_Vencido :u32,
    Informacio : String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    //wtr.write_record(rdr.headers()?)?;

    for result in rdr.deserialize() {
        let record: RecordIn= result?;
        //println!("{:?}", record);
        println!("{:?}",record.Legajo);

        let _otro_record = record.clone();
        //let mut legajito = otro_record.Legajo;
        let legajito = String::from("JUANCHO");
        println!("El legajito {}",legajito);

        let _salida = RecordOut{
            Legajo : legajito,
            Sector : 50,
            Cuit : String::from("2099999"),
            Tipo_Doc: String::from("DNU"),
            Nrodoc : String::from("99999"),
            Atraso : String::from("90"),
            Tipocarte : String::from("A"),
            Estado : String::from("B"),
            Estadoina : String::from("C"),
            Comcorr : 269,
            Totali : 2975,
            Saldo_Vencido: 1081,
            Informacio : String::from("Z"),
        };


        //wtr.serialize(salida)?;  // ESTO NO ANDA

        //wtr.serialize(record)?;  // ESTO SI ANDA



        // Try this if you don't like each record smushed on one line:
        // println!("{:#?}", record);
    }
    wtr.flush()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}