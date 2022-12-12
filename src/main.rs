#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused)]

// $env:RUST_BACKTRACE=1
// $env:RUST_BACKTRACE=1; cargo run


// LINEA 13 DEL CSV DEL INPUT ME QUEDA DESFAZADA.... ALGUN CAMPO ES MAS LARGO DE LO QUE DEBE SER...

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;

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


fn procesa_csv() -> Result<(),io::Error> {

    let mut file_csv1 = OpenOptions::new()
    .write(true)
    .append(true)
    .open("C:\\Users\\20171078343\\lee_csv\\Cuotas.csv")
    .expect("Error abriendo el archivo CSV");

    //let mut wtr2 = csv::Writer::from_writer(file_csv1);
    
    let file_ascii = File::create("salidas.txt")?;
    let mut file_ascii2 = LineWriter::new(file_ascii);

    let blanco01: String;
    let blanco02: String;
    let blancos01: String;
    let blancos02: String;
    
    let blanco01 = " ";
    let blancos01 = blanco01.pad_to_width(8);
    let blanco02 = " ";
    let blancos02 = blanco02.pad_to_width(250);

    let HDR_CUIT = String::from("30606513433");
	let HDR_TIPO_REG = String::from("HH");
	let HDR_MATRICULA = String::from("      735");
	let HDR_PROVINCIA = String::from("C");
	let HDR_GRADO = String::from("MM");
	let HDR_RETORNO = String::from("  ");
	let BLANCOS1 = blancos01;
	let HDR_ARCHIVO = String::from("M");
	let HDR_FECHA_GRABACION = String::from("AAAAMMDD");
	let HDR_HORA_GRABACION = String::from(" HHMMSS");
	let BLANCOS2 = blancos02;
 
    let data_cuit  = String::from("30606513433");
    let data_cuit2 = String::from("000000030606513433");
    let data_cuit3 = String::from("11111111000000030606513433");
    let fin_linea  = String::from("\r\n");
    
    let todas = HDR_CUIT  + &HDR_TIPO_REG + &HDR_MATRICULA +&HDR_PROVINCIA + &HDR_GRADO + &HDR_RETORNO +&BLANCOS1 + &HDR_ARCHIVO + &HDR_FECHA_GRABACION +&HDR_HORA_GRABACION +&BLANCOS2 + &fin_linea;
    
    file_ascii2.write_all(todas.as_bytes(),);

    let mut rdr = csv::ReaderBuilder::new()
             .has_headers(true)
             .delimiter(b';')
             .double_quote(true)
             .escape(Some(b'\\'))
             //.flexible(true)
             .comment(Some(b'#'))
             .from_path("Cuotas.csv")?;

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
        let filler02 = "  ";
        let espacios02 = filler02.pad_to_width(2);

        let filler16:String;
        let filler16 = "                ";
        let espacios16 = filler16; //.pad_to_width_with_char(16, ' '); // No me funciono....:(

        let filler208:String;
        let filler208 = "                                                                                                                                                                                                               ";
        let espacios208 = filler208; //.pad_to_width_with_char(208, ' ');  // No me funciono....:(


        //println!("{:?}",record.cuilIn);
        let newRec = RecOut {
            cuilOut: record.cuil.trim().to_string().pad_to_width(11),
            docTipoOut: record.doc_tipo.trim().to_string().pad_to_width(3),
            docNroOut: record.doc_nro.trim().to_string().pad(20, '0', Alignment::Right, true),
            diasAtrasOut : record.dias_atras.trim().to_string().pad_to_width_with_char(3, '0'),
            tipoOut: record.tipo.trim().to_string().pad_to_width_with_char(2, 'Y'),
            estadoOut : record.estado.trim().to_string().pad_to_width(1),
            estadoInaOut : record.estado_ina.trim().to_string().pad_to_width(1),
            compromisoOut : record.compromiso.to_string().pad(9, '0', Alignment::Right, true),
            saldoTotalOut : record.saldo_tota.to_string().pad(9, '0', Alignment::Right, true),
            saldoVencOut : record.saldo_venc.to_string().pad(9, '0', Alignment::Right, true),
            informacionOut: record.informacio.trim().to_string().pad_to_width_with_char(6, '0'),
            filler02: espacios02.to_string(),
            filler16: espacios16.to_string(),
            filler208: espacios208.to_string(),
        };
   
        let mut dias = newRec.diasAtrasOut.parse::<i32>().unwrap();  /// Sacar UnWrap !!!
        let mut diaz = 0;
        if dias > 999 {
            diaz = 999;
        }else {
            diaz = dias;
        };
        let mut dias_print = diaz.to_string();

        //let mut dias_print = diaz.to_string();
        //println!("el numerico transformado {}",dias_print);
        let lineas = newRec.cuilOut + &newRec.docTipoOut + &newRec.docNroOut + &dias_print + &newRec.tipoOut + &newRec.estadoOut + &newRec.estadoInaOut +&newRec.compromisoOut + &newRec.saldoTotalOut + &newRec.saldoVencOut + &newRec.informacionOut + &newRec.filler02 + &newRec.filler16 + &newRec.filler208 + &fin_linea;
   
        file_ascii2.write_all(lineas.as_bytes(),);
    
        //file_ascii2.write_all(todas.as_bytes(),);

        //let ret = wtr.serialize(newRec);
        //wtr.flush()?;

        file_ascii2 .flush();
    }
    
    Ok(())
}

fn main() {
    let resultado = match procesa_csv() {
        Ok(resultado) => {println!("Ejecucion Correcta del Programa para INAES")}
        Err(e) => {println!("Error en la Ejecucion del Programa para INAES{}",e);}
    };
}
