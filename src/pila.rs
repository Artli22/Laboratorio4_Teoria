use std::fmt::Display;

pub struct Pila<T> {
    elementos: Vec<T>,
}

impl<T> Pila<T> {
    pub fn nueva() -> Self {
        Self {
            elementos: Vec::new(),
        }
    }

    pub fn insertar(&mut self, valor: T) {
        self.elementos.push(valor);
    }

    pub fn extraer(&mut self) -> Option<T> {
        self.elementos.pop()
    }

    pub fn cima(&self) -> Option<&T> {
        self.elementos.last()
    }

    pub fn esta_vacia(&self) -> bool {
        self.elementos.is_empty()
    }

    pub fn longitud(&self) -> usize {
        self.elementos.len()
    }
}

impl<T: Display> Pila<T> {
    pub fn como_texto(&self) -> String {
        if self.elementos.is_empty() {
            return String::from("Vacía");
        }

        self.elementos
            .iter()
            .map(|elemento| elemento.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn mostrar(&self) {
        println!("Pila: {}", self.como_texto());
    }
}