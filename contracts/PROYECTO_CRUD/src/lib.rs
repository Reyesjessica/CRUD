use soroban_sdk::{contractimpl, contracttype, Env, Symbol, Map};

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
    pub fn crear(env: Env, id: Symbol, nombre: Symbol, descripcion: Symbol) {
        let mut proyectos: Map<Symbol, Proyecto> = env.storage().persistent().get(&"proyectos").unwrap_or_default();
        let proyecto = Proyecto { id: id.clone(), nombre, descripcion };
        proyectos.set(id.clone(), proyecto);
        env.storage().persistent().set(&"proyectos", &proyectos);
    }

    pub fn leer(env: Env, id: Symbol) -> Option<Proyecto> {
        let proyectos: Map<Symbol, Proyecto> = env.storage().persistent().get(&"proyectos").unwrap_or_default();
        proyectos.get(id)
    }

    pub fn actualizar(env: Env, id: Symbol, nombre: Symbol, descripcion: Symbol) {
        let mut proyectos: Map<Symbol, Proyecto> = env.storage().persistent().get(&"proyectos").unwrap_or_default();
        if proyectos.contains_key(id.clone()) {
            let proyecto = Proyecto { id: id.clone(), nombre, descripcion };
            proyectos.set(id.clone(), proyecto);
            env.storage().persistent().set(&"proyectos", &proyectos);
        }
    }

    pub fn eliminar(env: Env, id: Symbol) {
        let mut proyectos: Map<Symbol, Proyecto> = env.storage().persistent().get(&"proyectos").unwrap_or_default();
        proyectos.remove(id.clone());
        env.storage().persistent().set(&"proyectos", &proyectos);
    }
}
