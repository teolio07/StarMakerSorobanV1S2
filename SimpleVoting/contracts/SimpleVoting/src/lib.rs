#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, Address, Env, Symbol};
// Solo almacenamos lo básico
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]

// Solo almacenamos lo básico
pub enum DataKey {
    // Quien creó la votación
    Creator,
    // Si la votación está activa
    Active,
    // Cuántos votos tiene "SI"
    VotesSi,
    // Cuántos votos tiene "NO"
    VotesNo,
    // Si una persona ya votó
    HasVoted(Address),
}

#[contract]
pub struct SimpleVoting;

#[contractimpl]
impl SimpleVoting {
    /// Inicializar la votación (solo una vez)
    pub fn init(env: Env, creator: Address) {
        // El creador debe autorizar
        creator.require_auth();

        log!(&env, "Iniciando votación UUUUUUUUUUU, creador: {}", creator);

        // Guardar datos iniciales
        env.storage().instance().set(&DataKey::Creator, &creator);
        env.storage().instance().set(&DataKey::Active, &true);
        env.storage().instance().set(&DataKey::VotesSi, &0u32);
        env.storage().instance().set(&DataKey::VotesNo, &0u32);

        log!(&env, "Votación inicializada correctamente");
    }
    /// Votar SI
    pub fn vote_si(env: Env, voter: Address) {
        // El votante debe autorizar
        voter.require_auth();

        log!(&env, "Usuario {} votando SI", voter);

        // Verificar que la votación esté activa
        let active: bool = env
            .storage()
            .instance()
            .get(&DataKey::Active)
            .unwrap_or(false);

        if !active {
            panic!("La votación no está activa");
        }

        // Verificar que no haya votado antes
        let has_voted_key = DataKey::HasVoted(voter.clone());
        if env.storage().instance().has(&has_voted_key) {
            panic!("Ya votaste");
        }

        // Registrar que votó
        env.storage().instance().set(&has_voted_key, &true);

        // Incrementar votos SI
        let current_votes: u32 = env.storage().instance().get(&DataKey::VotesSi).unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::VotesSi, &(current_votes + 1));

        log!(
            &env,
            "Voto SI registrado. Total votos SI: {}",
            current_votes + 1
        );
    }
    /// Votar NO
    pub fn vote_no(env: Env, voter: Address) {
        // El votante debe autorizar
        voter.require_auth();

        log!(&env, "Usuario {} votando NO", voter);

        // Verificar que la votación esté activa
        let active: bool = env
            .storage()
            .instance()
            .get(&DataKey::Active)
            .unwrap_or(false);

        if !active {
            panic!("La votación no está activa");
        }

        // Verificar que no haya votado antes
        let has_voted_key = DataKey::HasVoted(voter.clone());
        if env.storage().instance().has(&has_voted_key) {
            panic!("Ya votaste");
        }

        // Registrar que votó
        env.storage().instance().set(&has_voted_key, &true);

        // Incrementar votos NO
        let current_votes: u32 = env.storage().instance().get(&DataKey::VotesNo).unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::VotesNo, &(current_votes + 1));

        log!(
            &env,
            "Voto NO registrado. Total votos NO: {}",
            current_votes + 1
        );
    }
    /// Cerrar votación (solo el creador)
    pub fn close_voting(env: Env, creator: Address) {
        creator.require_auth();

        log!(&env, "Cerrando votación...");

        // Verificar que sea el creador
        let stored_creator: Address = env
            .storage()
            .instance()
            .get(&DataKey::Creator)
            .expect("No hay creador registrado");

        if stored_creator != creator {
            panic!("Solo el creador puede cerrar la votación");
        }

        // Cerrar votación
        env.storage().instance().set(&DataKey::Active, &false);

        log!(&env, "Votación cerrada");
    }
    /// Ver resultados
    pub fn get_results(env: Env) -> (u32, u32, bool) {
        let votes_si: u32 = env.storage().instance().get(&DataKey::VotesSi).unwrap_or(0);

        let votes_no: u32 = env.storage().instance().get(&DataKey::VotesNo).unwrap_or(0);

        let active: bool = env
            .storage()
            .instance()
            .get(&DataKey::Active)
            .unwrap_or(false);

        (votes_si, votes_no, active)
    }

    /// Verificar si alguien ya votó
    pub fn has_voted(env: Env, user: Address) -> bool {
        env.storage().instance().has(&DataKey::HasVoted(user))
    }
}

mod test;
