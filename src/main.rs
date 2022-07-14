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

#[derive(Debug, Deserialize)]
struct RecordIn {
    //compromiso;saldo_tota;saldo_venc;informacio
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
        diasAtrasOut : String,
        tipoOut: String,
        estadoOut : String,
        estadoInaOut : String,
        compromisoOut :u32,
        saldoTotalOut :u32,
        saldoVencOut :u32,
        informacionOut : String,
    }

    
    for result in rdr.deserialize() {
        let record: RecordIn = result?;
        
        //println!("{:?}",record.cuilIn);
        let newRec = RecOut {
            cuilOut: record.cuilIn.trim().to_owned(),
            docTipoOut: record.doc_tipo.trim(),
            docNroOut: record.doc_nro,
            diasAtrasOut : record. dias_atras,
            tipoOut: record.tipo,
            estadoOut : record.estado,
            estadoInaOut : record.estado_ina,
            compromisoOut : record.compromiso,
            saldoTotalOut : record.saldo_tota,
            saldoVencOut : record.saldo_venc,
            informacionOut: record.informacio,
        };
   
        println!("{:?}",newRec.docTipoOut.trim());
        println!("{:?}",newRec.cuilOut.pad_to_width(11));
        

        let ret = wtr.serialize(newRec);

        //wtr.write_record(&["a", "b", "c"])?;
        wtr.flush()?;
    }
    
    Ok(())
}

fn main() {
    if let Err(err) = procesa_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
