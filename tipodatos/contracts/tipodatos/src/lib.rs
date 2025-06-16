#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, String, Symbol};
#[contract]
pub struct ConditionalSt;

#[contractimpl]
impl ConditionalSt {
    // Función que usa if-else para determinar si un número es positivo o negativo.
    pub fn check_number(env: Env, num: i32) -> String {
        if num >= 0 {
            String::from_str(&env, "Numero positivo")
        } else {
            String::from_str(&env, "Numero negativo")
        }
    }

    // Función que usa match para asignar permisos según el rol del usuario.
    pub fn check_role(env: Env, role: Symbol) -> String {
        let admin: Symbol = symbol_short!("Admin");
        let user: Symbol = symbol_short!("User");
        let guest: Symbol = symbol_short!("Guest");

        match role {
            admin => String::from_str(&env, "Acceso total"),
            user => String::from_str(&env, "Acceso limitado"),
            guest => String::from_str(&env, "Solo lectura"),
            _ => String::from_str(&env, "Rol no reconocido"),
        }
    }
}