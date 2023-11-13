mod lsystem;
mod turtle_graphics;
mod svg_test;
mod systems;

fn main() {

    let (system, instructions) = systems::DRAGON();

    let result = system.evolve_lsystem_from_start(12);
    //println!("finished state : {:?}", &result);

    turtle_graphics::draw(&result, &instructions);

}
