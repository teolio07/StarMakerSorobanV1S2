#![cfg(test)]
use super::*;
use soroban_sdk::{symbol_short, testutils::Logs, Env};
extern crate std;
#[test]
fn test_age_classification_complete() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AgeClassifierContract);
    let client = AgeClassifierContractClient::new(&env, &contract_id);

    std::println!("=== PROBANDO CLASIFICADOR DE EDAD ===\n");

    // Probar diferentes edades
    let test_ages = [0, 5, 10, 12, 13, 15, 17, 18, 21, 25, 65];

    for age in test_ages {
        std::println!("--- Probando edad: {} años ---", age);

        // Clasificar edad
        let classification = client.classify_age(&age);
        std::println!("classify_age({}) = {:?}", age, classification);

        // Verificar si es adulto
        let is_adult = client.is_adult(&age);
        std::println!("is_adult({}) = {}", age, is_adult);

        // Verificar si es adolescente
        let is_teenager = client.is_teenager(&age);
        std::println!("is_teenager({}) = {}", age, is_teenager);

        // Verificar si es menor
        let is_child = client.is_child(&age);
        std::println!("is_child({}) = {}", age, is_child);

        // Mostrar información detallada
        client.age_info(&age);

        std::println!(); // Línea en blanco
    }

    // Verificar clasificaciones específicas
    assert_eq!(client.classify_age(&5), symbol_short!("menor"));
    assert_eq!(client.classify_age(&15), symbol_short!("adolesc"));
    assert_eq!(client.classify_age(&25), symbol_short!("mayor"));

    // Verificar booleanos
    assert_eq!(client.is_adult(&25), true);
    assert_eq!(client.is_adult(&15), false);
    assert_eq!(client.is_teenager(&15), true);
    assert_eq!(client.is_teenager(&25), false);
    assert_eq!(client.is_child(&5), true);
    assert_eq!(client.is_child(&15), false);

    // Mostrar todos los logs
    std::println!("=== TODOS LOS LOGS GENERADOS ===");
    let logs = env.logs().all();
    for (i, log) in logs.iter().enumerate() {
        std::println!("Log {}: {:?}", i + 1, log);
    }

    std::println!("\n=== Test completado - Total logs: {} ===", logs.len());
}
