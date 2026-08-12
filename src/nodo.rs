#[derive(Debug, Clone)]
pub enum TipoNodo {
    Operando,
    OperadorUnario,
    OperadorBinario,
}

#[derive(Debug, Clone)]
pub struct Nodo {
    pub valor: String,
    pub tipo: TipoNodo,
    pub izquierdo: Option<Box<Nodo>>,
    pub derecho: Option<Box<Nodo>>,
}

impl Nodo {
    pub fn operando(valor: String) -> Self {
        Self {
            valor,
            tipo: TipoNodo::Operando,
            izquierdo: None,
            derecho: None,
        }
    }

    pub fn unario(operador: String, hijo: Nodo) -> Self {
        Self {
            valor: operador,
            tipo: TipoNodo::OperadorUnario,
            izquierdo: Some(Box::new(hijo)),
            derecho: None,
        }
    }

    pub fn binario(operador: String, izquierdo: Nodo, derecho: Nodo) -> Self {
        Self {
            valor: operador,
            tipo: TipoNodo::OperadorBinario,
            izquierdo: Some(Box::new(izquierdo)),
            derecho: Some(Box::new(derecho)),
        }
    }
}