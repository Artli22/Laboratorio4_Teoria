use crate::afn::{AFN, Transicion};
use crate::nodo::{Nodo, TipoNodo};

#[derive(Debug)]
struct Fragmento {
    inicio: usize,
    fin: usize,
    transiciones: Vec<Transicion>,
}

pub struct Thompson {
    siguiente_estado: usize,
}

impl Thompson {
    pub fn nuevo() -> Self {
        Self {
            siguiente_estado: 0,
        }
    }

    fn nuevo_estado(&mut self) -> usize {
        let estado = self.siguiente_estado;
        self.siguiente_estado += 1;
        estado
    }

    pub fn construir(&mut self, raiz: &Nodo) -> Result<AFN, String> {
        let fragmento = self.construir_fragmento(raiz)?;

        Ok(AFN::nuevo(
            fragmento.inicio,
            fragmento.fin,
            fragmento.transiciones,
        ))
    }

    fn construir_fragmento(
        &mut self,
        nodo: &Nodo,
    ) -> Result<Fragmento, String> {
        match nodo.tipo {
            TipoNodo::Operando => {
                self.construir_operando(&nodo.valor)
            }

            TipoNodo::OperadorUnario => {
                match nodo.valor.as_str() {
                    "*" => self.construir_kleene(nodo),
                    "+" => self.construir_positivo(nodo),
                    "?" => self.construir_opcional(nodo),

                    _ => Err(format!(
                        "Operador unario desconocido: {}",
                        nodo.valor
                    )),
                }
            }

            TipoNodo::OperadorBinario => {
                match nodo.valor.as_str() {
                    "." => self.construir_concatenacion(nodo),
                    "|" => self.construir_union(nodo),

                    _ => Err(format!(
                        "Operador binario desconocido: {}",
                        nodo.valor
                    )),
                }
            }
        }
    }

    fn construir_operando(
        &mut self,
        valor: &str,
    ) -> Result<Fragmento, String> {
        let inicio = self.nuevo_estado();
        let fin = self.nuevo_estado();

        let simbolo = if valor == "ε" {
            None
        } else {
            let mut caracteres = valor.chars();

            let caracter = caracteres
                .next()
                .ok_or_else(|| {
                    String::from(
                        "Se encontró un operando vacío."
                    )
                })?;

            if caracteres.next().is_some() {
                return Err(format!(
                    "El operando '{}' contiene más de un símbolo.",
                    valor
                ));
            }

            Some(caracter)
        };

        Ok(Fragmento {
            inicio,
            fin,
            transiciones: vec![
                Transicion {
                    origen: inicio,
                    destino: fin,
                    simbolo,
                }
            ],
        })
    }

    fn construir_concatenacion(
        &mut self,
        nodo: &Nodo,
    ) -> Result<Fragmento, String> {
        let izquierdo = nodo
            .izquierdo
            .as_ref()
            .ok_or_else(|| {
                String::from(
                    "La concatenación no tiene hijo izquierdo."
                )
            })?;

        let derecho = nodo
            .derecho
            .as_ref()
            .ok_or_else(|| {
                String::from(
                    "La concatenación no tiene hijo derecho."
                )
            })?;

        let fragmento_izquierdo =
            self.construir_fragmento(izquierdo)?;

        let fragmento_derecho =
            self.construir_fragmento(derecho)?;

        let inicio = fragmento_izquierdo.inicio;
        let fin = fragmento_derecho.fin;

        let mut transiciones = Vec::new();

        transiciones.extend(
            fragmento_izquierdo.transiciones
        );

        transiciones.push(
            Transicion {
                origen: fragmento_izquierdo.fin,
                destino: fragmento_derecho.inicio,
                simbolo: None,
            }
        );

        transiciones.extend(
            fragmento_derecho.transiciones
        );

        Ok(Fragmento {
            inicio,
            fin,
            transiciones,
        })
    }

    fn construir_union(
        &mut self,
        nodo: &Nodo,
    ) -> Result<Fragmento, String> {
        let izquierdo = nodo
            .izquierdo
            .as_ref()
            .ok_or_else(|| {
                String::from(
                    "El operador | no tiene hijo izquierdo."
                )
            })?;

        let derecho = nodo
            .derecho
            .as_ref()
            .ok_or_else(|| {
                String::from(
                    "El operador | no tiene hijo derecho."
                )
            })?;

        let fragmento_izquierdo =
            self.construir_fragmento(izquierdo)?;

        let fragmento_derecho =
            self.construir_fragmento(derecho)?;

        let inicio = self.nuevo_estado();
        let fin = self.nuevo_estado();

        let mut transiciones = Vec::new();

        transiciones.extend(
            fragmento_izquierdo.transiciones
        );

        transiciones.extend(
            fragmento_derecho.transiciones
        );

        transiciones.push(
            Transicion {
                origen: inicio,
                destino: fragmento_izquierdo.inicio,
                simbolo: None,
            }
        );

        transiciones.push(
            Transicion {
                origen: inicio,
                destino: fragmento_derecho.inicio,
                simbolo: None,
            }
        );

        transiciones.push(
            Transicion {
                origen: fragmento_izquierdo.fin,
                destino: fin,
                simbolo: None,
            }
        );

        transiciones.push(
            Transicion {
                origen: fragmento_derecho.fin,
                destino: fin,
                simbolo: None,
            }
        );

        Ok(Fragmento {
            inicio,
            fin,
            transiciones,
        })
    }

    fn construir_kleene(
        &mut self,
        nodo: &Nodo,
    ) -> Result<Fragmento, String> {
        let hijo = nodo
            .izquierdo
            .as_ref()
            .ok_or_else(|| {
                String::from(
                    "El operador * no tiene hijo."
                )
            })?;

        let fragmento =
            self.construir_fragmento(hijo)?;

        let inicio = self.nuevo_estado();
        let fin = self.nuevo_estado();

        let mut transiciones =
            fragmento.transiciones;

        transiciones.push(
            Transicion {
                origen: inicio,
                destino: fragmento.inicio,
                simbolo: None,
            }
        );

        transiciones.push(
            Transicion {
                origen: inicio,
                destino: fin,
                simbolo: None,
            }
        );

        transiciones.push(
            Transicion {
                origen: fragmento.fin,
                destino: fragmento.inicio,
                simbolo: None,
            }
        );

        transiciones.push(
            Transicion {
                origen: fragmento.fin,
                destino: fin,
                simbolo: None,
            }
        );

        Ok(Fragmento {
            inicio,
            fin,
            transiciones,
        })
    }

    fn construir_positivo(
        &mut self,
        nodo: &Nodo,
    ) -> Result<Fragmento, String> {
        let hijo = nodo
            .izquierdo
            .as_ref()
            .ok_or_else(|| {
                String::from(
                    "El operador + no tiene hijo."
                )
            })?;

        let fragmento =
            self.construir_fragmento(hijo)?;

        let inicio = self.nuevo_estado();
        let fin = self.nuevo_estado();

        let mut transiciones =
            fragmento.transiciones;

        transiciones.push(
            Transicion {
                origen: inicio,
                destino: fragmento.inicio,
                simbolo: None,
            }
        );

        transiciones.push(
            Transicion {
                origen: fragmento.fin,
                destino: fragmento.inicio,
                simbolo: None,
            }
        );

        transiciones.push(
            Transicion {
                origen: fragmento.fin,
                destino: fin,
                simbolo: None,
            }
        );

        Ok(Fragmento {
            inicio,
            fin,
            transiciones,
        })
    }

    fn construir_opcional(
        &mut self,
        nodo: &Nodo,
    ) -> Result<Fragmento, String> {
        let hijo = nodo
            .izquierdo
            .as_ref()
            .ok_or_else(|| {
                String::from(
                    "El operador ? no tiene hijo."
                )
            })?;

        let fragmento =
            self.construir_fragmento(hijo)?;

        let inicio = self.nuevo_estado();
        let fin = self.nuevo_estado();

        let mut transiciones =
            fragmento.transiciones;

        transiciones.push(
            Transicion {
                origen: inicio,
                destino: fragmento.inicio,
                simbolo: None,
            }
        );

        transiciones.push(
            Transicion {
                origen: inicio,
                destino: fin,
                simbolo: None,
            }
        );

        transiciones.push(
            Transicion {
                origen: fragmento.fin,
                destino: fin,
                simbolo: None,
            }
        );

        Ok(Fragmento {
            inicio,
            fin,
            transiciones,
        })
    }
}

pub fn construir_afn(
    raiz: &Nodo,
) -> Result<AFN, String> {
    let mut thompson = Thompson::nuevo();
    thompson.construir(raiz)
}