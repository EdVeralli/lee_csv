use anyhow::*;
use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use std::cmp::Ordering;
use time_format_conversion::{calcular_tiempo_en_millis_from_minuto_y_segundo, Millis};

#[derive(Debug, Eq)]
pub struct PilotoRecord {
    pub numero: Option<u32>,
    pub minutos: Option<u32>,
    pub segundos: Option<u32>,
    pub nombre: String,
    pub auto: String,
    pub anio: Option<u32>,
    pub categoria: Option<String>,
    pub tiempo: Option<Millis>,
}

impl Ord for PilotoRecord {
    fn cmp(&self, other: &PilotoRecord) -> Ordering {
        self.numero.cmp(&other.numero)
    }
}

impl PartialOrd for PilotoRecord {
    fn partial_cmp(&self, other: &PilotoRecord) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PilotoRecord {
    fn eq(&self, other: &PilotoRecord) -> bool {
        self.numero == other.numero && self.tiempo == other.tiempo
    }
}

impl PilotoRecord {
    pub fn get_numero(&self) -> Option<u32> {
        self.numero
    }

    pub fn get_lap_in_millis(&self) -> Option<Millis> {
        if let (Some(minutos), Some(segundos)) = (self.minutos, self.segundos) {
            return Some((segundos + (minutos * 60)) * 1000);
        }
        None
    }

    pub fn read_pilotos_csv(path: &str) -> Result<Vec<PilotoRecord>> {
        #[derive(Deserialize)]
        pub struct PilotoRecordAux {
            #[serde(rename = "N°", deserialize_with = "csv::invalid_option")]
            numero: Option<u32>,
            #[serde(rename = "m", deserialize_with = "csv::invalid_option")]
            minutos: Option<u32>,
            #[serde(rename = "s", deserialize_with = "csv::invalid_option")]
            segundos: Option<u32>,
            #[serde(rename = "PILOTO")]
            nombre: String,
            #[serde(rename = "AUTOMOVIL")]
            auto: String,
            #[serde(rename = "Año", deserialize_with = "csv::invalid_option")]
            anio: Option<u32>,
            #[serde(rename = "Cat", deserialize_with = "csv::invalid_option")]
            categoria: Option<String>,
        }

        let mut pilotos: Vec<PilotoRecord> = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .double_quote(true)
            .escape(Some(b'\\'))
            .flexible(true)
            .comment(Some(b'#'))
            .from_path(path)?;

        for result in rdr.deserialize() {
            let row: PilotoRecordAux = result?;

            // Agregar solo si tiene numero de auto
            if let Some(_value) = row.numero {
                pilotos.push(PilotoRecord {
                    numero: row.numero,
                    minutos: row.minutos,
                    segundos: row.segundos,
                    nombre: row.nombre.replace("–", "-"), // reemplazar el guion maldito que pone LibreOffice
                    auto: row.auto,
                    anio: row.anio,
                    categoria: row.categoria,
                    tiempo: if let (Some(value_minutos), Some(value_segundos)) =
                        (row.minutos, row.segundos)
                    {
                        Some(calcular_tiempo_en_millis_from_minuto_y_segundo(
                            value_minutos,
                            value_segundos,
                        ))
                    } else {
                        None
                    },
                });
            }
        }
        Ok(pilotos)
    }

    pub fn read_pilotos_excel(sheetname: &str, grupo: Option<u8>) -> Result<Vec<PilotoRecord>> {
        type ExcelRow = (
            Option<u32>,
            Option<u32>,
            Option<u32>,
            String,
            String,
            Option<u32>,
            Option<String>,
        );

        let sheetname_aux = match grupo {
            None => sheetname.to_string(),
            Some(g) => format!("{}_grupo_{}", sheetname, g),
        };

        let mut pilotos: Vec<PilotoRecord> = Vec::new();
        if let Some(r) = open_workbook::<Xlsx<_>, _>("pilotos.xlsx")
            .map_err(|_| anyhow!("Error al intentar abrir archivo pilotos.xlsx"))?
            .worksheet_range(&sheetname_aux)
            .transpose()
            .map_err(|_| {
                anyhow!("No se encuentra la hoja de la carrera en la planilla Excel de pilotos")
            })?
        {
            let iter = RangeDeserializerBuilder::new().from_range(&r)?;
            for result in iter {
                let (numero, minutos, segundos, piloto, auto, anio, categoria): ExcelRow = result?;

                pilotos.push(PilotoRecord {
                    numero,
                    minutos,
                    segundos,
                    nombre: piloto.replace("–", "-"), // reemplazar el guion maldito que pone LibreOffice
                    auto,
                    anio,
                    categoria,
                    tiempo: minutos.and_then(|value_minutos| {
                        segundos.map(|value_segundos| {
                            calcular_tiempo_en_millis_from_minuto_y_segundo(
                                value_minutos,
                                value_segundos,
                            )
                        })
                    }),
                });
            }
        }
        Ok(pilotos)
    }
}
