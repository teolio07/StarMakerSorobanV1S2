#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Logs, Env};

extern crate std;

#[test]
fn test_all_functions_with_logs() {
    let env = Env::default();
    let contract_id = env.register(PrimitivesContract, ());
    let client = PrimitivesContractClient::new(&env, &contract_id);
    std::println!("=== Iniciando tests de todas las funciones ===");

    // 1. Mostrar enteros sin signo
    std::println!("\n1. Llamando show_unsigned_integers():");
    client.show_unsigned_integers();

    // 2. Mostrar enteros con signo
    std::println!("\n2. Llamando show_signed_integers():");
    client.show_signed_integers();

    // 3. Mostrar booleanos
    std::println!("\n3. Llamando show_boolean():");
    client.show_boolean();

    // 4. Mostrar symbols
    std::println!("\n4. Llamando show_symbol():");
    client.show_symbol();

    // 5. Operaciones matemáticas básicas
    std::println!("\n5. Llamando basic_math(15, 25):");
    let math_result = client.basic_math(&15, &25);
    std::println!("Resultado de basic_math: {}", math_result);

    // 6. Comparar números
    std::println!("\n6. Llamando compare_numbers(100, 50):");
    let compare_result = client.compare_numbers(&100, &50);
    std::println!("Resultado de compare_numbers: {}", compare_result);

    // 7. Obtener u32
    std::println!("\n7. Llamando get_u32():");
    let u32_result = client.get_u32();
    std::println!("Resultado de get_u32: {}", u32_result);

    // 8. Obtener i128
    std::println!("\n8. Llamando get_i128():");
    let i128_result = client.get_i128();
    std::println!("Resultado de get_i128: {}", i128_result);

    // 9. Obtener bool
    std::println!("\n9. Llamando get_bool():");
    let bool_result = client.get_bool();
    std::println!("Resultado de get_bool: {}", bool_result);

    // 10. Obtener symbol
    std::println!("\n10. Llamando get_symbol():");
    let symbol_result = client.get_symbol();
    std::println!("Resultado de get_symbol: {:?}", symbol_result);

    // Mostrar todos los logs generados
    std::println!("\n=== TODOS LOS LOGS GENERADOS ===");
    let logs = env.logs().all();
    for (i, log) in logs.iter().enumerate() {
        std::println!("Log {}: {:?}", i + 1, log);
    }

    std::println!("\n=== Test completado - Total de logs: {} ===", logs.len());
}
