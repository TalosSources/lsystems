mod lsystem;
mod svg_test;
mod systems;
mod turtle_graphics;

fn main() {
    let (system, instructions) = systems::SIMPLE_TREE();

    let result = system.evolve_from_start(11);
    turtle_graphics::draw(&result, &instructions, "simple_tree.svg")
}
