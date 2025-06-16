#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Logs, vec, Env, String};
extern crate std;

#[test]
fn test_create_and_get_task() {
    let env = Env::default();
    let contract_id = env.register(TaskContract, ());
    let client = TaskContractClient::new(&env, &contract_id);

    let task_id = 1u32;
    let description = symbol_short!("LEARN");

    // Crear tarea
    let task = client.create_task(&task_id, &description);

    // Verificar que se creó correctamente
    assert_eq!(task.id, task_id);
    assert_eq!(task.description, description);
    assert!(matches!(task.status, TaskStatus::Pending));

    // Verificar que se puede recuperar
    let retrieved_task = client.get_task(&task_id).unwrap();
    assert_eq!(retrieved_task.id, task_id);
    std::println!("{}", env.logs().all().join("\n"));
}
#[test]
fn test_update_task_status() {
    let env = Env::default();
    let contract_id = env.register(TaskContract, ());
    let client = TaskContractClient::new(&env, &contract_id);

    let task_id = 2u32;
    let description = symbol_short!("CODE");

    // Crear y actualizar tarea
    client.create_task(&task_id, &description);
    let updated = client.update_status(&task_id, &TaskStatus::Completed);

    assert!(updated);

    let task = client.get_task(&task_id).unwrap();
    assert!(matches!(task.status, TaskStatus::Completed));
}
#[test]
fn test_task_counter() {
    let env = Env::default();
    let contract_id = env.register(TaskContract, ());
    let client = TaskContractClient::new(&env, &contract_id);
    
    // Inicialmente debe ser 0
    assert_eq!(client.get_task_count(), 0);
    
    // Crear algunas tareas
    client.create_task(&1u32, &symbol_short!("TASK1"));
    client.create_task(&2u32, &symbol_short!("TASK2"));
    
    // Verificar contador
    assert_eq!(client.get_task_count(), 2);
}