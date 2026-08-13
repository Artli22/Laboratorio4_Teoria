use crate::afn::AFN;
use std::collections::HashSet;

fn cerradura_epsilon(
    afn: &AFN,
    estados: &HashSet<usize>,
) -> HashSet<usize> {
    let mut cerradura = estados.clone();
    let mut pila: Vec<usize> = estados.iter().copied().collect();

    while let Some(estado_actual) = pila.pop() {
        for transicion in &afn.transiciones {
            if transicion.origen == estado_actual
                && transicion.simbolo.is_none()
                && !cerradura.contains(&transicion.destino)
            {
                cerradura.insert(transicion.destino);
                pila.push(transicion.destino);
            }
        }
    }

    cerradura
}

fn mover(
    afn: &AFN,
    estados: &HashSet<usize>,
    simbolo: char,
) -> HashSet<usize> {
    let mut siguientes = HashSet::new();

    for estado in estados {
        for transicion in &afn.transiciones {
            if transicion.origen == *estado
                && transicion.simbolo == Some(simbolo)
            {
                siguientes.insert(transicion.destino);
            }
        }
    }

    siguientes
}

pub fn simular(
    afn: &AFN,
    cadena: &str,
) -> bool {
    let mut actuales = HashSet::new();

    actuales.insert(afn.estado_inicial);

    actuales = cerradura_epsilon(
        afn,
        &actuales,
    );

    for simbolo in cadena.chars() {
        let siguientes = mover(
            afn,
            &actuales,
            simbolo,
        );

        actuales = cerradura_epsilon(
            afn,
            &siguientes,
        );

        if actuales.is_empty() {
            return false;
        }
    }

    actuales.contains(
        &afn.estado_aceptacion
    )
}