mod lsystem;
mod svg_test;
mod systems;
mod turtle_graphics;

fn main() {
    let (system, instructions) = systems::FRACTAL_PLANT();

    let result = system.evolve_from_start(7);
    turtle_graphics::draw(&result, &instructions, "fractal_plant.svg")
}
