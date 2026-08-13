mod pila;
mod precedencia;
mod reglas;
mod shunting_yard;
mod nodo;
mod arbol;
mod diagrama;
mod afn;
mod thompson;
mod graphviz;

use std::fs;

fn normalizar_expresion(expresion: &str) -> String {
    expresion.replace('∗', "*")
}

fn main() {
    println!("============================================");
    println!(" Conversión Infix a Postfix - Shunting Yard ");
    println!("============================================");

    let contenido = match fs::read_to_string("NuevasExpresionesRegulares.txt") {
        Ok(contenido) => contenido,

        Err(error) => {
            eprintln!(
                "No fue posible abrir NuevasExpresionesRegulares.txt: {}",
                error
            );

            return;
        }
    };

    for (indice, linea) in contenido.lines().enumerate() {
        let expresion_original = linea.trim();
        let expresion = normalizar_expresion(expresion_original);

        if expresion.is_empty() {
            continue;
        }

        println!("\n============================================");
        println!("Expresión número {}", indice + 1);
        println!("============================================");

        println!("Expresión original: {}", expresion_original);
        println!("Expresión procesada: {}", expresion);

        match shunting_yard::convertir_a_postfix(&expresion) {
            Ok(resultado) => {
                println!("\nResultado:");

                println!(
                    "Postfix antes de convertir + y ?: {}",
                    resultado.postfix_original
                );

                println!(
                    "Postfix final: {}",
                    resultado.postfix_convertido
                );

                match arbol::construir_arbol(
                    &resultado.tokens_postfix
                ) {
                    Ok(arbol) => {
                        println!("\nÁrbol sintáctico:");
                        diagrama::mostrar_arbol(&arbol);

                        println!("\nAFN de Thompson:");

                        match thompson::construir_afn(&arbol) {
                            Ok(afn) => {
                                afn.mostrar();

                                println!("\nGenerando grafo del AFN...");

                                let nombre = format!(
                                    "afn_{}",
                                    indice + 1
                                );

                                match graphviz::generar_grafo(
                                    &afn,
                                    &nombre
                                ) {
                                    Ok(()) => {}

                                    Err(error) => {
                                        println!(
                                            "Error al generar el grafo:"
                                        );

                                        println!("{}", error);
                                    }
                                }
                            }

                            Err(error) => {
                                println!(
                                    "Error al construir el AFN:"
                                );

                                println!("{}", error);
                            }
                        }
                    }

                    Err(error) => {
                        println!(
                            "\nError al construir el árbol:"
                        );

                        println!("{}", error);
                    }
                }
            }

            Err(error) => {
                println!("\nError: {}", error);
            }
        }
    }
}