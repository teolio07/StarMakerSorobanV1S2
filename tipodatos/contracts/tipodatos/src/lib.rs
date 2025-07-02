#![no_std]
use soroban_sdk::{contract, contractimpl, log, Env, Symbol, symbol_short};

#[contract]
pub struct PrimitivesContract;

#[contractimpl]
impl PrimitivesContract {
    
    // Tipos numéricos sin signo permitidos en Soroban
    pub fn show_unsigned_integers(env: Env) {
        let val_u32: u32 = 4294967295;  // Máximo u32
        let val_u64: u64 = 18446744073709551615;  // Máximo u64
        let val_u128: u128 = 340282366920938463463374607431768211455;  // Máximo u128
        
        log!(&env, "Tipos sin signo:");
        log!(&env, "u32: {}", val_u32);
        log!(&env, "u64: {}", val_u64);
        log!(&env, "u128: {}", val_u128);
    }
    
    // Tipos numéricos con signo permitidos en Soroban
    pub fn show_signed_integers(env: Env) {
        let val_i32: i32 = -2147483648;  // Mínimo i32
        let val_i64: i64 = -9223372036854775808;  // Mínimo i64
        let val_i128: i128 = -170141183460469231731687303715884105728;  // Mínimo i128
        
        log!(&env, "Tipos con signo:");
        log!(&env, "i32: {}", val_i32);
        log!(&env, "i64: {}", val_i64);
        log!(&env, "i128: {}", val_i128);
    }
    
    // Tipo booleano
    pub fn show_boolean(env: Env) {
        let is_true: bool = true;
        let is_false: bool = false;
        
        log!(&env, "Tipo booleano:");
        log!(&env, "true: {}", is_true);
        log!(&env, "false: {}", is_false);
    }
    
    // Tipo Symbol (cadenas cortas hasta 32 caracteres)
    pub fn show_symbol(env: Env) {
        let sym1: Symbol = symbol_short!("hello");
        let sym2: Symbol = symbol_short!("world123");
        let sym3: Symbol = symbol_short!("symbol");
        
        log!(&env, "Tipo Symbol:");
        log!(&env, "Symbol 1: {}", sym1);
        log!(&env, "Symbol 2: {}", sym2);
        log!(&env, "Symbol 3: {}", sym3);
    }
    
    // Operaciones básicas con enteros
    pub fn basic_math(env: Env, a: u32, b: u32) -> u32 {
        let sum = a + b;
        let product = a * b;
        
        log!(&env, "Matemáticas básicas:");
        log!(&env, "a: {}, b: {}", a, b);
        log!(&env, "Suma: {}", sum);
        log!(&env, "Producto: {}", product);
        
        sum
    }
    
    // Comparaciones con booleanos
    pub fn compare_numbers(env: Env, x: i64, y: i64) -> bool {
        let is_equal = x == y;
        let is_greater = x > y;
        let is_less = x < y;
        
        log!(&env, "Comparaciones:");
        log!(&env, "x: {}, y: {}", x, y);
        log!(&env, "x == y: {}", is_equal);
        log!(&env, "x > y: {}", is_greater);
        log!(&env, "x < y: {}", is_less);
        
        is_equal
    }
    
    // Función simple que retorna cada tipo primitivo
    pub fn get_u32(env: Env) -> u32 {
        let value: u32 = 42;
        log!(&env, "Retornando u32: {}", value);
        value
    }
    
    pub fn get_i128(env: Env) -> i128 {
        let value: i128 = -123456789;
        log!(&env, "Retornando i128: {}", value);
        value
    }
    
    pub fn get_bool(env: Env) -> bool {
        let value: bool = true;
        log!(&env, "Retornando bool: {}", value);
        value
    }
    
    pub fn get_symbol(env: Env) -> Symbol {
        let value: Symbol = symbol_short!("result");
        log!(&env, "Retornando Symbol: {}", value);
        value
    }
}
mod test;