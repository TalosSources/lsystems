use std::collections::HashMap;
use std::hash::Hash;

use svg::node::element::{path::Data, Path};
use svg::Document;

type DrawUnit = f32;

type Vector2 = (DrawUnit, DrawUnit);
type Color = (u8, u8, u8);

pub enum Command {
    DrawForward(DrawUnit),
    MoveForward(DrawUnit),
    Turn(DrawUnit),
    Push,
    Pop,
}

struct Turtle2D {
    position: Vector2,
    direction: Vector2,
    pen: Pen,
    stack: Vec<(Vector2, Vector2)>,
}

struct Pen {
    color: Color,
    width: f32,
}

fn scale(v: &Vector2, s: f32) -> Vector2 {
    (v.0 * s, v.1 * s)
}

fn add(v1: &Vector2, v2: &Vector2) -> Vector2 {
    (v1.0 + v2.0, v1.1 + v2.1)
}

impl Turtle2D {
    pub fn rotate(&mut self, angle: f32) {
        //( (cos -sin) (sin cos) )
        self.direction = (
            self.direction.0 * angle.cos() - self.direction.1 * angle.sin(),
            self.direction.0 * angle.sin() + self.direction.1 * angle.cos(),
        );
    }
}

pub fn draw<E: Clone + Hash + Eq>(
    string: &Vec<E>,
    instructions: &HashMap<E, Vec<Command>>,
    filename: &str,
) {
    let mut turtle: Turtle2D = Turtle2D {
        position: (1000., 1900.),
        direction: (0.0, -1.0),
        pen: (Pen {
            color: (180, 220, 80),
            width: (1.0),
        }),
        stack: vec![],
    };

    let mut data = Data::new().move_to(turtle.position);

    for c in string {
        if let Some(instructions) = instructions.get(c) {
            for instruction in instructions {
                match instruction {
                    Command::DrawForward(l) => {
                        let delta_v = scale(&turtle.direction, *l);
                        data = data.line_by(delta_v);
                        turtle.position = add(&turtle.position, &delta_v);
                    }

                    Command::MoveForward(l) => {
                        let delta_v = scale(&turtle.direction, *l);
                        data = data.move_by(delta_v);
                        turtle.position = add(&turtle.position, &delta_v);
                    }

                    Command::Turn(a) => turtle.rotate(*a),

                    Command::Push => turtle.stack.push((turtle.position, turtle.direction)),

                    Command::Pop => {
                        if let Some((pos, dir)) = turtle.stack.pop() {
                            data = data.move_to(pos);
                            turtle.position = pos;
                            turtle.direction = dir;
                        }
                    }
                }
            }
        }
    }

    let color_string = format!(
        "rgb({}, {}, {})",
        turtle.pen.color.0, turtle.pen.color.1, turtle.pen.color.2
    );
    println!("color String = {}", color_string);

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", color_string)
        .set("stroke-width", turtle.pen.width)
        .set("d", data);

    let document = Document::new().set("viewBox", (0, 0, 2000, 2000)).add(path);

    svg::save(filename, &document).unwrap();
}
