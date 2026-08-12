#[derive(Debug, Clone)]
pub struct Transicion {
    pub origen: usize,
    pub destino: usize,
    pub simbolo: Option<char>,
}

#[derive(Debug, Clone)]
pub struct AFN {
    pub estado_inicial: usize,
    pub estado_aceptacion: usize,
    pub transiciones: Vec<Transicion>,
}

impl AFN {
    pub fn nuevo(
        estado_inicial: usize,
        estado_aceptacion: usize,
        transiciones: Vec<Transicion>,
    ) -> Self {
        Self {
            estado_inicial,
            estado_aceptacion,
            transiciones,
        }
    }

    pub fn mostrar(&self) {
        println!("Estado inicial: q{}", self.estado_inicial);
        println!("Estado de aceptación: q{}", self.estado_aceptacion);

        for transicion in &self.transiciones {
            let simbolo = match transicion.simbolo {
                Some(c) => c.to_string(),
                None => "ε".to_string(),
            };

            println!(
                "q{} --{}--> q{}",
                transicion.origen,
                simbolo,
                transicion.destino
            );
        }
    }
}