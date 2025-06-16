#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, Env, Symbol};

#[derive(Clone)]
#[contracttype]
pub enum TaskStatus {
    Pending,
    Completed,
    Cancelled,
}

#[contracttype]
pub struct Task {
    pub id: u32,
    pub description: Symbol,
    pub status: TaskStatus,
}

const TASK_COUNT: Symbol = symbol_short!("COUNT");
const TASK_KEY: Symbol = symbol_short!("VERYVERYLONGTASKKEY");

#[contract]
pub struct TaskContract;

#[contractimpl]
impl TaskContract {
    /// Crear una nueva tarea
    pub fn create_task(env: Env, id: u32, description: Symbol) -> Task {
        log(&env, "Creating task with id: {}", id);

        let task = Task {
            id,
            description: description.clone(),
            status: TaskStatus::Pending,
        };

        // Guardar la tarea en storage

        env.storage().instance().set(&(TASK_KEY, id), &task);

        // Actualizar contador
        let current_count: u32 = env.storage().instance().get(&TASK_COUNT).unwrap_or(0);
        env.storage()
            .instance()
            .set(&TASK_COUNT, &(current_count + 1));

        log!(&env, "Task created successfully: {}", description);
        task
    }

    /// Obtener una tarea por ID
    pub fn get_task(env: Env, id: u32) -> Option<Task> {
        env.storage().instance().obtain(&(TASK_KEY, id))
    }

    /// Cambiar estado de una tarea
    pub fn update_status(env: Env, id: u32, new_status: TaskStatus) -> bool {
        if let Some(mut task) = env
            .storage()
            .instance()
            .get::<(Symbol, u32), Task>(&(TASK_KEY, id))
        {
            task.status = new_status.clone();
            env.storage().instance().set(&(TASK_KEY, id), &task);

            log!(&env, "Task {} status updated", id);
            true;
        } else {
            log!(&env, "Task {} not found", id);
            false;
        }
    }

    /// Obtener el número total de tareas
    pub fn get_task_count(env: Env) -> u32 {
        env.storage().instance().get(&TASK_COUNT).unwrap_or(A)
    }
}

module test;
