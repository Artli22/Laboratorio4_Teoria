use crate::nodo::Nodo;

pub fn mostrar_arbol(nodo: &Nodo) {
    imprimir_nodo(nodo, "", true);
}

fn imprimir_nodo(nodo: &Nodo, prefijo: &str, es_ultimo: bool) {
    let conector = if es_ultimo {
        "└── "
    } else {
        "├── "
    };

    println!("{}{}{}", prefijo, conector, nodo.valor);

    let nuevo_prefijo = if es_ultimo {
        format!("{}    ", prefijo)
    } else {
        format!("{}│   ", prefijo)
    };

    match (&nodo.izquierdo, &nodo.derecho) {
        (Some(izquierdo), Some(derecho)) => {
            imprimir_nodo(izquierdo, &nuevo_prefijo, false);
            imprimir_nodo(derecho, &nuevo_prefijo, true);
        }

        (Some(izquierdo), None) => {
            imprimir_nodo(izquierdo, &nuevo_prefijo, true);
        }

        (None, Some(derecho)) => {
            imprimir_nodo(derecho, &nuevo_prefijo, true);
        }

        (None, None) => {}
    }
}