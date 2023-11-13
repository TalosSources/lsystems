mod lsystem;
mod svg_test;
mod systems;
mod turtle_graphics;

fn main() {
    let (system, instructions) = systems::DRAGON();

    let result = system.evolve_lsystem_from_start(12);
    //println!("finished state : {:?}", &result);

    turtle_graphics::draw(&result, &instructions);
}
