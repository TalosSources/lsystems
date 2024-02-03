mod lsystem;
mod svg_test;
mod systems;
mod turtle_graphics;
mod utils;
use std::hash::Hash;

fn main() {
    let (system, instructions) = systems::SAUPE_TREE();

    let result = system.evolve_from_start(9);
    turtle_graphics::draw(&result, &instructions, "saupe_tree.svg")
}

fn evolve_and_save_steps<T : Clone + Eq + Hash>(lsystem: &lsystem::LSystem<T>, ) {

    let lsystem_state = lsystem::LSystemState {lsystem, state : lsystem.start.clone()};
    // for i in 
    // TODO : just add a save_steps option to evolve_from_start, and also make evolve_from_start responsible for saving the end result (maybe with an option)
    // or maybe add an option for returning the intermediate states
}   
