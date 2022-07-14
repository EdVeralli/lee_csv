#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::error::Error;
use std::io;
use std::process;
use csv::ReaderBuilder;
use csv::WriterBuilder;
use serde::Deserialize;
use pad::{PadStr, Alignment};


#[derive(Debug, Deserialize)]
struct RecordIn {
    legajo : String,
    sector :Option<u64>,
    cuilIn : String,
    //#[serde(rename = "tipodoc")]
    doc_tipo: String,
    doc_nro : String,
    dias_atras : String,
    tipo: String,
    estado : String,
    estado_ina : String,
    compromiso :u32,
    saldo_tota :u32,
    //#[serde(rename = "salvenc")]
    saldo_venc :u32,
    informacio : String,
}

fn procesa_csv() -> Result<(), Box<dyn Error>> {
  
    let mut rdr = csv::ReaderBuilder::new()
             .has_headers(true)
             .delimiter(b';')
             .double_quote(true)
             .escape(Some(b'\\'))
             .flexible(true)
             .comment(Some(b'#'))
             .from_path("Cuotas.csv")?;

    let mut wtr = WriterBuilder::new()
              .has_headers(true)
              .from_path("Salida_Cuotas.csv")
              .unwrap();

    #[derive(serde::Serialize)]
    struct RecOut {
        cuilOut: String,
        docTipoOut: String,
        docNroOut : String,
        filler16 :  String,
        diasAtrasOut : String,
        tipoOut: String,
        estadoOut : String,
        estadoInaOut : String,
        compromisoOut :String,
        saldoTotalOut :String,
        saldoVencOut :String,
        filler02 : String,
        informacionOut : String,
        filler208 : String,
    }

    
    for result in rdr.deserialize() {
        let record: RecordIn = result?;
        
        let filler02:String;
        let filler02 = " ";
        let espacios02 = filler02.pad_to_width(2);

        let filler16:String;
        let filler16 = " ";
        let espacios16 = filler16.pad_to_width_with_char(16, '-');

        let filler208:String;
        let filler208 = " ";
        let espacios208 = filler16.pad_to_width(208);


        //println!("{:?}",record.cuilIn);
        let newRec = RecOut {
            cuilOut: record.cuilIn.trim().to_string().pad_to_width(11),
            docTipoOut: record.doc_tipo.trim().to_string().pad_to_width(3),
            docNroOut: record.doc_nro.trim().to_string().pad(20, '0', Alignment::Right, true),
            diasAtrasOut : record. dias_atras.trim().to_string().pad_to_width_with_char(3, '0'),
            tipoOut: record.tipo.trim().to_string().pad_to_width(2),
            estadoOut : record.estado.trim().to_string().pad_to_width(1),
            estadoInaOut : record.estado_ina.trim().to_string().pad_to_width(1),
            compromisoOut : record.compromiso.to_string().pad(9, '0', Alignment::Right, true),
            saldoTotalOut : record.saldo_tota.to_string().pad(9, '0', Alignment::Right, true),
            saldoVencOut : record.saldo_venc.to_string().pad(9, '0', Alignment::Right, true),
            informacionOut: record.informacio.trim().to_string().pad_to_width_with_char(6, '0'),
            filler02: espacios02,
            filler16: espacios16,
            filler208: espacios208,
        };
   

        //println!("{:?}",newRec.docNroOut);
        //println!("{:?}",espacios208);
        
        let ret = wtr.serialize(newRec);

        wtr.flush()?;
    }
    
    Ok(())
}

fn main() {
    if let Err(err) = procesa_csv() {
        println!("Error en la Ejecucion del Programa para INAES: {}", err);
        process::exit(1);
    }
}
