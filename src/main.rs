#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::error::Error;
use std::io;
use std::process;
use csv::ReaderBuilder;
use csv::{QuoteStyle, WriterBuilder};
use serde::Deserialize;
use pad::{PadStr, Alignment};
use std::fs::OpenOptions;


#[derive(Debug, Deserialize)]
struct RecordIn {
    legajo : String,
    sector : String,
    cuil : String,
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
    //saldo_venc :Option<u64>,
    saldo_venc : String,
    informacio : String,
}

#[derive(serde::Serialize)]
struct HeaderOut {
    HDR_CUIT: String,
    HDR_TIPO_REG : String,
    HDR_MATRICULA: String,
    HDR_PROVINCIA: String,
    HDR_GRADO: String,
    HDR_RETORNO: String,
    BLANCOS1: String,
    HDR_ARCHIVO: String,
    HDR_FECHA_GRABACION: String,
    HDR_HORA_GRABACION: String,
    BLANCOS2: String,
}


fn procesa_csv() -> Result<(), Box<dyn Error>> {


    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open("test.csv")
    .unwrap();
    let mut wtr2 = csv::Writer::from_writer(file);
    


    let blanco01: String;
    let blanco02: String;
    let blancos01: String;
    let blancos02: String;
    
    let blanco01 = " ";
    let blancos01 = blanco01.pad_to_width(8);
    let blanco02 = " ";
    let blancos02 = blanco02.pad_to_width(250);

    let newRecHeader = HeaderOut {
        HDR_CUIT: "30606513433".to_string(),
        HDR_TIPO_REG: "HH".to_string(),
        HDR_MATRICULA: "      735".to_string(),
        HDR_PROVINCIA: "C".to_string(),
        HDR_GRADO: "MM".to_string(),
        HDR_RETORNO: "  ".to_string(),
        BLANCOS1: blancos01,
        HDR_ARCHIVO: "M".to_string(),
        HDR_FECHA_GRABACION: "AAAAMMDD".to_string(),
        HDR_HORA_GRABACION: " HHMMSS".to_string(),
        BLANCOS2: blancos02,
    };
        


    let mut rdr = csv::ReaderBuilder::new()
             .has_headers(true)
             .delimiter(b';')
             .double_quote(true)
             .escape(Some(b'\\'))
             //.flexible(true)
             .comment(Some(b'#'))
             .from_path("Cuotas.csv")?;

    let mut wtr = csv::WriterBuilder::new()
              .has_headers(false)
              //.flexible(false)
              //.quote_style(QuoteStyle::Never)
              .from_path("Salida_Cuotas.csv")
              .unwrap();
              //.delimiter(b':');
              //.flexible(false);
             

    let mut wtr02 = csv::WriterBuilder::new()
        .has_headers(false)
        .quote_style(QuoteStyle::Never)
        .from_path("Salida_Cuotas.csv")
        .unwrap();

        //.flexible(false);
        //.delimiter(b':');


    let retHeader = wtr02.serialize(newRecHeader);
    wtr02.flush()?;

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
        let espacios16 = filler16.pad_to_width_with_char(16, ' ');

        let filler208:String;
        let filler208 = " ";
        let espacios208 = filler16.pad_to_width(208);


        //println!("{:?}",record.cuilIn);
        let newRec = RecOut {
            cuilOut: record.cuil.trim().to_string().pad_to_width(11),
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
