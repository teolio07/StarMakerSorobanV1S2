#![no_std]
use soroban_sdk::{contract, contractimpl, log, Env, Symbol, symbol_short};

#[contract]
pub struct AgeClassifierContract;
#[contractimpl]
impl AgeClassifierContract {
    
    // Función principal para clasificar por edad
    pub fn classify_age(env: Env, age: u32) -> Symbol {
        log!(&env, "Clasificando edad: {}", age);
        
        let classification = if age >= 18 {
            log!(&env, "Persona de {} años es MAYOR DE EDAD", age);
            symbol_short!("mayor")
        } 
        else if age >= 13 {
            log!(&env, "Persona de {} años es ADOLESCENTE", age);
            symbol_short!("adolesc")
        } 
        else {
            log!(&env, "Persona de {} años es MENOR DE EDAD", age);
            symbol_short!("menor")
        };
        
        log!(&env, "Clasificación final: {}", classification);
        classification
    }
    
    // Función que verifica si es mayor de edad (true/false)
    pub fn is_adult(env: Env, age: u32) -> bool {
        let adult = age >= 18;
        log!(&env, "¿Es {} años mayor de edad?: {}", age, adult);
        adult
    }
    
    // Función que verifica si es adolescente
    pub fn is_teenager(env: Env, age: u32) -> bool {
        let teenager = age >= 13 && age < 18;
        log!(&env, "¿Es {} años adolescente?: {}", age, teenager);
        teenager
    }
    
    // Función que verifica si es menor de edad
    pub fn is_child(env: Env, age: u32) -> bool {
        let child = age < 13;
        log!(&env, "¿Es {} años menor de edad?: {}", age, child);
        child
    }
    
    // Función para mostrar información detallada de la edad
    pub fn age_info(env: Env, age: u32) {
        log!(&env, "=== INFORMACIÓN DE EDAD ===");
        log!(&env, "Edad ingresada: {} años", age);
        
        if age >= 18 {
            log!(&env, "Categoría: MAYOR DE EDAD");
            log!(&env, "Puede votar: SÍ");
            log!(&env, "Puede conducir: SÍ");
        } else if age >= 13 {
            log!(&env, "Categoría: ADOLESCENTE");
            log!(&env, "Puede votar: NO");
            log!(&env, "Puede conducir: DEPENDE (con permiso)");
        } else {
            log!(&env, "Categoría: MENOR DE EDAD");
            log!(&env, "Puede votar: NO");
            log!(&env, "Puede conducir: NO");
        }
        
        log!(&env, "========================");
    }
}
mod test;
