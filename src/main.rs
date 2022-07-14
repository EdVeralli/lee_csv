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
        diasAtrasOut : String,
        tipoOut: String,
        estadoOut : String,
        estadoInaOut : String,
        compromisoOut :String,
        saldoTotalOut :String,
        saldoVencOut :String,
        informacionOut : String,
    }

    
    for result in rdr.deserialize() {
        let record: RecordIn = result?;
        
        //println!("{:?}",record.cuilIn);
        let newRec = RecOut {
            cuilOut: record.cuilIn.trim().to_string().pad_to_width(11),
            docTipoOut: record.doc_tipo.trim().to_string().pad_to_width(3),
            docNroOut: record.doc_nro.trim().to_string().pad_to_width(20),
            diasAtrasOut : record. dias_atras.trim().to_string().pad_to_width_with_char(3, '0'),
            tipoOut: record.tipo.trim().to_string().pad_to_width(2),
            estadoOut : record.estado.trim().to_string().pad_to_width(1),
            estadoInaOut : record.estado_ina.trim().to_string().pad_to_width(1),
            compromisoOut : record.compromiso.to_string().pad(9, '0', Alignment::Right, true),
            saldoTotalOut : record.saldo_tota.to_string().pad(9, '0', Alignment::Right, true),
            saldoVencOut : record.saldo_venc.to_string().pad(9, '0', Alignment::Right, true),
            informacionOut: record.informacio.trim().to_string().pad_to_width_with_char(6, '0'),
        };
   
        //println!("{:?}",newRec.docTipoOut.trim());
        println!("{:?}",newRec.compromisoOut);

        let ret = wtr.serialize(newRec);

        wtr.flush()?;
    }
    
    Ok(())
}

fn main() {
    if let Err(err) = procesa_csv() {
        use fill::Fill;
        let mut memory = None;

        memory.fill(42..);
        assert_eq!(memory, Some(42));
        println!("error running example: {}", err);
        process::exit(1);
    }
}
