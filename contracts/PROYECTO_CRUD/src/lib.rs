use soroban_sdk::{contractimpl, contracttype, Env, Symbol, Map, Vec};

#[derive(Clone, Debug)]
#[contracttype]
pub struct Proyecto {
    pub id: Symbol,
    pub nombre: Symbol,
    pub descripcion: Symbol,
}

pub struct ProyectoCrudContract;

#[contractimpl]
impl ProyectoCrudContract {
    // Crear un nuevo proyecto, retorna true si fue exitoso, false si ya existía
    pub fn crear(env: Env, id: Symbol, nombre: Symbol, descripcion: Symbol) -> bool {
        let key = Symbol::short("proyectos");
        let mut proyectos: Map<Symbol, Proyecto> = env.storage().persistent().get(&key).unwrap_or_default();

        if proyectos.contains_key(id.clone()) {
            return false; // Ya existe un proyecto con este ID
        }

        let proyecto = Proyecto { id: id.clone(), nombre, descripcion };
        proyectos.set(id.clone(), proyecto);
        env.storage().persistent().set(&key, &proyectos);
        true
    }

    // Leer un proyecto por ID
    pub fn leer(env: Env, id: Symbol) -> Option<Proyecto> {
        let key = Symbol::short("proyectos");
        let proyectos: Map<Symbol, Proyecto> = env.storage().persistent().get(&key).unwrap_or_default();
        proyectos.get(id)
    }
    
    // Actualizar un proyecto existente, retorna true si se actualizó, false si no existía
    pub fn actualizar(env: Env, id: Symbol, nombre: Symbol, descripcion: Symbol) -> bool {
        let key = Symbol::short("proyectos");
        let mut proyectos: Map<Symbol, Proyecto> = env.storage().persistent().get(&key).unwrap_or_default();

        if !proyectos.contains_key(id.clone()) {
            return false; // No existe un proyecto con este ID
        }

        let proyecto_actualizado = Proyecto { id: id.clone(), nombre, descripcion };
        proyectos.set(id.clone(), proyecto_actualizado);
        env.storage().persistent().set(&key, &proyectos);
        true
    }
}