use crate::afn::AFN;

use std::fs;
use std::process::Command;

pub fn generar_grafo(
    afn: &AFN,
    nombre_archivo: &str,
) -> Result<(), String> {
    let mut dot = String::new();

    dot.push_str("digraph AFN {\n");
    dot.push_str("    rankdir=LR;\n\n");
    dot.push_str("    node [shape=circle];\n\n");
    dot.push_str("    inicio [shape=point];\n");

    dot.push_str(&format!(
        "    inicio -> q{};\n\n",
        afn.estado_inicial
    ));

    dot.push_str(&format!(
        "    q{} [shape=doublecircle];\n\n",
        afn.estado_aceptacion
    ));

    for transicion in &afn.transiciones {
        let simbolo = match transicion.simbolo {
            Some(c) => c.to_string(),
            None => "ε".to_string(),
        };

        dot.push_str(&format!(
            "    q{} -> q{} [label=\"{}\"];\n",
            transicion.origen,
            transicion.destino,
            simbolo
        ));
    }

    dot.push_str("}\n");

    let archivo_dot = format!("{}.dot", nombre_archivo);
    let archivo_png = format!("{}.png", nombre_archivo);

    fs::write(&archivo_dot, dot)
        .map_err(|error| {
            format!(
                "No se pudo crear {}: {}",
                archivo_dot,
                error
            )
        })?;

    let resultado = Command::new("dot")
        .args([
            "-Tpng",
            &archivo_dot,
            "-o",
            &archivo_png,
        ])
        .output()
        .map_err(|error| {
            format!(
                "No se pudo ejecutar Graphviz: {}",
                error
            )
        })?;

    if !resultado.status.success() {
        let error = String::from_utf8_lossy(
            &resultado.stderr
        );

        return Err(format!(
            "Graphviz produjo un error: {}",
            error
        ));
    }

    println!(
        "Grafo generado correctamente: {}",
        archivo_png
    );

    Ok(())
}