#![allow(dead_code, unused_variables)]

use chrono::{Datelike, Duration, Local, NaiveDate};

#[derive(Debug, Clone)]
pub struct Fecha {
    pub año: i32,
    pub dia: u32,
    pub mes: u32,
}

impl Fecha {
    pub fn new(dia: u32, mes: u32, año: i32) -> Self {
        Fecha { dia, mes, año }
    }

    pub fn es_fecha_valida(&self) -> bool {
        NaiveDate::from_ymd_opt(self.año, self.mes, self.dia).is_some()
    }

    pub fn es_bisiesto(&self) -> bool {
        (self.año % 4 == 0 && self.año % 100 != 0) || (self.año % 400 == 0)
    }

    pub fn sumar_dias(&mut self, dias: i64) {
        let date = NaiveDate::from_ymd_opt(self.año, self.mes, self.dia).unwrap();
        let date = date + Duration::days(dias);

        self.dia = date.day();
        self.mes = date.month();
        self.año = date.year();
    }

    pub fn restar_dias(&mut self, dias: i64) {
        let date = NaiveDate::from_ymd_opt(self.año, self.mes, self.dia).unwrap();
        let date = date - Duration::days(dias);

        self.dia = date.day();
        self.mes = date.month();
        self.año = date.year();
    }

    pub fn es_mayor(&self, fecha: &Fecha) -> bool {
        let date1 = match NaiveDate::from_ymd_opt(self.año, self.mes, self.dia) {
            Some(date) => date,
            None => return false,
        };

        let date2 = match NaiveDate::from_ymd_opt(fecha.año, fecha.mes, fecha.dia) {
            Some(date) => date,
            None => return true,
        };

        date1 > date2
    }

    pub fn es_menor(&self, fecha: Fecha) -> bool {
        let date1 = match NaiveDate::from_ymd_opt(self.año, self.mes, self.dia) {
            Some(date) => date,
            None => return false,
        };

        let date2 = match NaiveDate::from_ymd_opt(fecha.año, fecha.mes, fecha.dia) {
            Some(date) => date,
            None => return true,
        };

        date1 < date2
    }

    pub fn comparar(&self, f: &Fecha) -> bool {
        self.dia == f.dia && self.mes == f.mes && self.año == f.año
    }

    pub fn fecha_actual() -> Self {
        let hoy = Local::now().date_naive();
        Fecha {
            dia: hoy.day(),
            mes: hoy.month(),
            año: hoy.year(),
        }
    }
}
