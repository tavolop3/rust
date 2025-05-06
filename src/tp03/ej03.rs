pub struct Fecha {
    pub año:u16,
    pub dia:u8,
    pub mes:u8,
}

// TODO: hacer un wrapper con alguna dependencia de fechas
impl Fecha {
    pub fn new(dia:u8, mes:u8, año:u16) -> Self {
        Fecha {
            dia,
            mes,
            año
        }
    }

    pub fn es_fecha_valida(&self) -> bool {
        if self.dia == 0 || self.mes == 0 || self.dia > 31 || self.mes > 12 {
            return false;
        }
        
        let dias_x_mes = match self.mes {
            4 | 6 | 9 | 11 => 30,
            2 => if self.es_bisiesto() { 29 } else { 28 },
            _ => 31,
        };

        return self.dia <= dias_x_mes
    }

    pub fn es_bisiesto(&self) -> bool {
        (self.año % 4 == 0 && self.año % 100 != 0) || (self.año % 400 == 0)
    }

    pub fn sumar_dias(&mut self, dias:u8) {
        let mut dias_restantes = dias;
        while dias_restantes > 0 {
            let dias_en_mes = match self.mes {
                4 | 6 | 9 | 11 => 30,
                2 => if self.es_bisiesto() { 29 } else { 28 },
                _ => 31,
            };

            let dias_hasta_fin_mes = dias_en_mes - self.dia + 1;
            if dias_restantes >= dias_hasta_fin_mes {
                dias_restantes -= dias_hasta_fin_mes;
                self.dia = 1;
                self.mes += 1;
                if self.mes > 12 {
                    self.mes = 1;
                    self.año += 1;
                }
            } else {
                self.dia += dias_restantes;
                dias_restantes = 0;
            }
        }  
    }

    pub fn restar_dias(&mut self, dias:u8) {
        let mut dias_restantes = dias;
        while dias_restantes > 0 {
            if dias_restantes >= self.dia {
                dias_restantes -= self.dia;
                self.mes -= 1;
                if self.mes == 0 {
                    self.mes = 12;
                    self.año -= 1;
                }
                self.dia = match self.mes {
                    4 | 6 | 9 | 11 => 30,
                    2 => if self.es_bisiesto() { 29 } else { 28 },
                    _ => 31,
                };
            } else {
                self.dia -= dias_restantes;
                dias_restantes = 0;
            }
        }
    }

    pub fn es_mayor(&self, fecha:Fecha) -> bool {
        let t1 = self.dia as u16 + self.mes as u16 + self.año;
        let t2 = fecha.dia as u16 + fecha.mes as u16 + fecha.año;

        if t1 > t2 {
            return true;
        } else {
            return false;
        }
    }
}
