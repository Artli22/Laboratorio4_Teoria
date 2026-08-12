use crate::nodo::Nodo;
use crate::pila::Pila;

pub fn construir_arbol(postfix: &[String]) -> Result<Nodo, String> {
    let mut pila: Pila<Nodo> = Pila::nueva();

    for token in postfix {
        match token.as_str() {
            "*" | "+" | "?" => {
            let hijo = pila
                .extraer()
                .ok_or_else(|| {
                    format!(
                        "El operador '{}' no tiene operando.",
                        token
                    )
                })?;

            pila.insertar(
                Nodo::unario(token.clone(), hijo)
            );
        }

            "." | "|" => {
                let derecho = pila
                    .extraer()
                    .ok_or_else(|| format!("Falta el operando derecho para '{}'.", token))?;

                let izquierdo = pila
                    .extraer()
                    .ok_or_else(|| format!("Falta el operando izquierdo para '{}'.", token))?;

                pila.insertar(Nodo::binario(
                    token.clone(),
                    izquierdo,
                    derecho,
                ));
            }

            _ => {
                pila.insertar(Nodo::operando(token.clone()));
            }
        }
    }

    if pila.longitud() != 1 {
        return Err(String::from(
            "La expresión postfix no produjo un único árbol sintáctico.",
        ));
    }

    pila.extraer()
        .ok_or_else(|| String::from("No se pudo construir el árbol."))
}