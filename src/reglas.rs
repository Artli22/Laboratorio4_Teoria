use crate::pila::Pila;

#[derive(Clone, Debug)]
pub enum Token {
    Operando(String),
    Operador(char),
    Apertura,
    Cierre,
}

pub fn tokenizar(expresion: &str) -> Result<Vec<Token>, String> {
    let caracteres: Vec<char> = expresion.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut indice = 0;

    while indice < caracteres.len() {
        let simbolo = caracteres[indice];

        if simbolo.is_whitespace() {
            indice += 1;
            continue;
        }

        if simbolo == '\\' {
            if indice + 1 >= caracteres.len() {
                return Err(String::from(
                    "La expresión termina con un carácter de escape incompleto.",
                ));
            }

            let escapado = format!("\\{}", caracteres[indice + 1]);
            tokens.push(Token::Operando(escapado));
            indice += 2;
            continue;
        }

        if simbolo == '[' {
            let mut clase = String::from("[");
            indice += 1;

            let mut cerrada = false;
            let mut escapado = false;

            while indice < caracteres.len() {
                let actual = caracteres[indice];
                clase.push(actual);

                if escapado {
                    escapado = false;
                } else if actual == '\\' {
                    escapado = true;
                } else if actual == ']' {
                    cerrada = true;
                    indice += 1;
                    break;
                }

                indice += 1;
            }

            if !cerrada {
                return Err(String::from(
                    "Se encontró una clase de caracteres sin ']'.",
                ));
            }

            tokens.push(Token::Operando(clase));
            continue;
        }

        match simbolo {
            '(' => tokens.push(Token::Apertura),
            ')' => tokens.push(Token::Cierre),

            '|' | '.' | '*' | '+' | '?' | '^' => {
                tokens.push(Token::Operador(simbolo));
            }

            _ => tokens.push(Token::Operando(simbolo.to_string())),
        }

        indice += 1;
    }

    Ok(insertar_concatenaciones(tokens))
}

fn insertar_concatenaciones(tokens: Vec<Token>) -> Vec<Token> {
    let mut resultado: Vec<Token> = Vec::new();

    for token in tokens {
        if let Some(anterior) = resultado.last() {
            if puede_terminar_expresion(anterior) && puede_iniciar_expresion(&token) {
                resultado.push(Token::Operador('.'));
            }
        }

        resultado.push(token);
    }

    resultado
}

fn puede_terminar_expresion(token: &Token) -> bool {
    matches!(
        token,
        Token::Operando(_)
            | Token::Cierre
            | Token::Operador('*')
            | Token::Operador('+')
            | Token::Operador('?')
            | Token::Operador('^')
    )
}

fn puede_iniciar_expresion(token: &Token) -> bool {
    matches!(token, Token::Operando(_) | Token::Apertura)
}

pub fn tokens_como_texto(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|token| match token {
            Token::Operando(valor) => valor.clone(),
            Token::Operador(operador) => operador.to_string(),
            Token::Apertura => String::from("("),
            Token::Cierre => String::from(")"),
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn convertir_extensiones(postfix: &[String]) -> Result<String, String> {
    let mut pila: Pila<String> = Pila::nueva();

    for token in postfix {
        match token.as_str() {
            "*" => {
                let expresion = extraer_un_operando(&mut pila, '*')?;
                pila.insertar(format!("{}*", expresion));
            }

            "+" => {
                let expresion = extraer_un_operando(&mut pila, '+')?;
                pila.insertar(format!("{}{}*.", expresion, expresion));
            }

            "?" => {
                let expresion = extraer_un_operando(&mut pila, '?')?;

                // R? equivale a R|ε
                pila.insertar(format!("{}ε|", expresion));
            }

            "^" => {
                let expresion = extraer_un_operando(&mut pila, '^')?;
                pila.insertar(format!("{}^", expresion));
            }

            "." | "|" => {
                let derecha = pila.extraer().ok_or_else(|| {
                    format!("Falta el operando derecho para '{}'.", token)
                })?;

                let izquierda = pila.extraer().ok_or_else(|| {
                    format!("Falta el operando izquierdo para '{}'.", token)
                })?;

                pila.insertar(format!("{}{}{}", izquierda, derecha, token));
            }

            _ => {
                pila.insertar(token.clone());
            }
        }
    }

    if pila.longitud() != 1 {
        return Err(String::from(
            "La expresión postfix no produjo un único resultado.",
        ));
    }

    pila.extraer()
        .ok_or_else(|| String::from("No se pudo obtener la expresión final."))
}

fn extraer_un_operando(
    pila: &mut Pila<String>,
    operador: char,
) -> Result<String, String> {
    pila.extraer().ok_or_else(|| {
        format!(
            "El operador '{}' no tiene una expresión anterior sobre la cual actuar.",
            operador
        )
    })
}