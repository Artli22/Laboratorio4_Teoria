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
mod simulacion;

use std::fs;
use std::io::{self, Write};

use crate::afn::AFN;

fn normalizar_expresion(expresion: &str) -> String {
    expresion.replace('∗', "*")
}

fn main() {
    println!("============================================");
    println!(" Conversión Infix a Postfix - Shunting Yard ");
    println!("============================================");

    let contenido = match fs::read_to_string(
        "NuevasExpresionesRegulares.txt"
    ) {
        Ok(contenido) => contenido,

        Err(error) => {
            eprintln!(
                "No fue posible abrir NuevasExpresionesRegulares.txt: {}",
                error
            );

            return;
        }
    };

    let mut automatas: Vec<(String, AFN)> = Vec::new();

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

                                println!(
                                    "\nGenerando grafo del AFN..."
                                );

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

                                automatas.push(
                                    (
                                        expresion_original.to_string(),
                                        afn,
                                    )
                                );
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
                println!(
                    "\nError: {}",
                    error
                );
            }
        }
    }

    if automatas.is_empty() {
        println!(
            "\nNo se generó ningún AFN válido."
        );

        return;
    }

    iniciar_simulador(&automatas);
}

fn iniciar_simulador(
    automatas: &[(String, AFN)]
) {
    loop {
        println!("\n============================================");
        println!(" Simulación de AFN ");
        println!("============================================");

        println!("\nExpresiones regulares disponibles:");

        for (indice, (expresion, _)) in
            automatas.iter().enumerate()
        {
            println!(
                "{}. {}",
                indice + 1,
                expresion
            );
        }

        println!("0. Salir");

        print!("\nSeleccione una expresión r: ");

        io::stdout()
            .flush()
            .expect(
                "No se pudo actualizar la salida."
            );

        let mut entrada = String::new();

        if io::stdin()
            .read_line(&mut entrada)
            .is_err()
        {
            println!(
                "No se pudo leer la selección."
            );

            continue;
        }

        let seleccion: usize =
            match entrada.trim().parse() {
                Ok(numero) => numero,

                Err(_) => {
                    println!(
                        "Ingrese una opción válida."
                    );

                    continue;
                }
            };

        if seleccion == 0 {
            println!(
                "\nFinalizando simulador."
            );

            break;
        }

        if seleccion > automatas.len() {
            println!(
                "La opción seleccionada no existe."
            );

            continue;
        }

        let (_, afn) =
            &automatas[seleccion - 1];

        print!(
            "\nIngrese la cadena w: "
        );

        io::stdout()
            .flush()
            .expect(
                "No se pudo actualizar la salida."
            );

        let mut cadena = String::new();

        if io::stdin()
            .read_line(&mut cadena)
            .is_err()
        {
            println!(
                "No se pudo leer la cadena."
            );

            continue;
        }

        let cadena = cadena.trim();

        let pertenece =
            simulacion::simular(
                afn,
                cadena
            );

        println!(
            "\nResultado:"
        );

        if pertenece {
            println!(
                "Sí, w ∈ L(r)"
            );
        } else {
            println!(
                "No, w ∉ L(r)"
            );
        }
    }
}