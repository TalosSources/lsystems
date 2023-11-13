use std::collections::HashMap;
use std::hash::Hash;

pub struct LSystem<E: Clone + Eq + Hash> {
    // should always be immutable
    pub start: Vec<E>,
    pub rules: HashMap<E, Vec<E>>,
}

pub struct LSystemState<'a, E: Clone + Eq + Hash> {
    pub lsystem: &'a LSystem<E>,
    pub state: Vec<E>, //maybe iter, n
}

impl<E: Clone + Eq + Hash> LSystemState<'_, E> {
    pub fn lsystem_iter(&mut self) {
        let mut result: Vec<E> = Vec::new();
        for c in &self.state {
            let c_prime_opt = self.lsystem.rules.get(c);
            let mut c_prime = match c_prime_opt {
                Some(str) => str.clone(),
                None => vec![c.clone()],
            };
            result.append(&mut c_prime);
        }

        self.state = result; // TODO : not this
    }

    pub fn lsystem_n_iters(&mut self, n: u32) {
        if n != 0 {
            self.lsystem_iter();
            self.lsystem_n_iters(n - 1);
        }
    }
}

impl<E: Clone + Eq + Hash> LSystem<E> {
    pub fn evolve_lsystem_from_start(&self, n: u32) -> Vec<E> {
        let mut lsystem_state = LSystemState {
            lsystem: self,
            state: self.start.clone(),
        };
        lsystem_state.lsystem_n_iters(n);
        lsystem_state.state
    }
}
