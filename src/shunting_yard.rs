use crate::pila::Pila;
use crate::precedencia;
use crate::reglas::{self, Token};

pub struct ResultadoConversion {
    pub postfix_original: String,
    pub postfix_convertido: String,
    pub tokens_postfix: Vec<String>,
}

pub fn convertir_a_postfix(
    expresion: &str,
) -> Result<ResultadoConversion, String> {
    let tokens = reglas::tokenizar(expresion)?;

    println!("Expresión infix: {}", expresion);
    println!(
        "Expresión con concatenación explícita: {}",
        reglas::tokens_como_texto(&tokens)
    );

    println!("\nPasos de Shunting Yard:");

    let mut salida: Vec<String> = Vec::new();
    let mut operadores: Pila<char> = Pila::nueva();

    for token in tokens {
        match token {
            Token::Operando(valor) => {
                salida.push(valor.clone());

                mostrar_paso(
                    &valor,
                    "Enviar operando a la salida",
                    &salida,
                    &operadores,
                );
            }

            Token::Apertura => {
                operadores.insertar('(');

                mostrar_paso(
                    "(",
                    "Insertar apertura en la pila",
                    &salida,
                    &operadores,
                );
            }

            Token::Cierre => {
                let mut encontro_apertura = false;

                while let Some(&cima) = operadores.cima() {
                    if cima == '(' {
                        operadores.extraer();
                        encontro_apertura = true;
                        break;
                    }

                    if let Some(operador) = operadores.extraer() {
                        salida.push(operador.to_string());
                    }
                }

                if !encontro_apertura {
                    return Err(String::from(
                        "Se encontró ')' sin una apertura correspondiente.",
                    ));
                }

                mostrar_paso(
                    ")",
                    "Extraer operadores hasta encontrar '('",
                    &salida,
                    &operadores,
                );
            }

            Token::Operador(operador)
                if precedencia::es_operador_unario(operador) =>
            {
                salida.push(operador.to_string());

                mostrar_paso(
                    &operador.to_string(),
                    "Enviar operador unario a la salida",
                    &salida,
                    &operadores,
                );
            }

            Token::Operador(operador)
                if precedencia::es_operador_binario(operador) =>
            {
                while let Some(&cima) = operadores.cima() {
                    if cima == '(' {
                        break;
                    }

                    if !precedencia::es_operador(cima) {
                        break;
                    }

                    if precedencia::obtener_precedencia(cima)
                        < precedencia::obtener_precedencia(operador)
                    {
                        break;
                    }

                    if let Some(extraido) = operadores.extraer() {
                        salida.push(extraido.to_string());
                    }
                }

                operadores.insertar(operador);

                mostrar_paso(
                    &operador.to_string(),
                    "Comparar precedencia e insertar operador",
                    &salida,
                    &operadores,
                );
            }

            Token::Operador(operador) => {
                return Err(format!(
                    "El símbolo '{}' no pudo clasificarse correctamente.",
                    operador
                ));
            }
        }
    }

    while let Some(operador) = operadores.extraer() {
        if operador == '(' {
            return Err(String::from(
                "Quedó un paréntesis de apertura sin cerrar.",
            ));
        }

        salida.push(operador.to_string());
    }

    println!("\nVaciado final de la pila:");
    println!("Salida: {}", salida.join(" "));
    println!("Pila: {}", operadores.como_texto());

    let postfix_original = salida.join("");
    let postfix_convertido = reglas::convertir_extensiones(&salida)?;

    Ok(ResultadoConversion {
        postfix_original,
        postfix_convertido,
        tokens_postfix: salida,
    })
}

fn mostrar_paso(
    simbolo: &str,
    accion: &str,
    salida: &[String],
    operadores: &Pila<char>,
) {
    println!("\nSímbolo leído: {}", simbolo);
    println!("Acción: {}", accion);
    println!("Salida: {}", salida.join(" "));
    println!("Pila: {}", operadores.como_texto());
}