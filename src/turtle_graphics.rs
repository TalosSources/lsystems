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
    Turn(DrawUnit), // in radians?
}

struct Turtle2D {
    direction: Vector2,
    pen: Pen,
}

struct Pen {
    color: Color,
    width: f32,
}

fn scale(v: &Vector2, s: f32) -> Vector2 {
    (v.0 * s, v.1 * s)
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

pub fn draw<E: Clone + Hash + Eq>(string: &Vec<E>, instructions: &HashMap<E, Command>) {
    let mut turtle: Turtle2D = Turtle2D {
        direction: (1.0, 0.0),
        pen: (Pen {
            color: (0xff, 0x0, 0xff),
            width: (4.0),
        }),
    };

    let mut data = Data::new().move_to((1200, 1000));

    for c in string {
        if let Some(ins) = instructions.get(c) {
            match ins {
                Command::DrawForward(l) => {
                    let delta_v = scale(&turtle.direction, *l);
                    data = data.line_by(delta_v)
                }
                Command::MoveForward(l) => {
                    let delta_v = scale(&turtle.direction, *l);
                    data = data.move_by(delta_v);
                }
                Command::Turn(a) => turtle.rotate(*a),
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

    svg::save("lsystem.svg", &document).unwrap();
}
