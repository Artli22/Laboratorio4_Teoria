pub fn es_operador(simbolo: char) -> bool {
    matches!(simbolo, '|' | '.' | '*' | '+' | '?' | '^')
}

pub fn obtener_precedencia(simbolo: char) -> u8 {
    match simbolo {
        '(' | ')' => 1,
        '|' => 2,
        '.' => 3,
        '*' | '+' | '?' => 4,
        '^' => 5,
        _ => 6,
    }
}

pub fn es_operador_unario(operador: char) -> bool {
    matches!(operador, '*' | '+' | '?' | '^')
}

pub fn es_operador_binario(operador: char) -> bool {
    matches!(operador, '|' | '.')
}