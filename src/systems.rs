use std::collections::HashMap;
use std::f32::consts::PI;

use crate::lsystem::*;
use crate::turtle_graphics::Command::{self, *};

fn rot_forward_instructions(angle: f32, scale: f32, g_means_draw: bool) -> HashMap<char, Command> {
    let mut instructions = HashMap::new();
    instructions.insert('F', DrawForward(scale));
    instructions.insert(
        'G',
        if g_means_draw {
            DrawForward(scale)
        } else {
            MoveForward(scale)
        },
    );
    instructions.insert('+', Turn(angle));
    instructions.insert('-', Turn(-angle));

    instructions
}

pub fn KOCH() -> (LSystem<char>, HashMap<char, Command>) {
    let mut rules: HashMap<char, Vec<char>> = HashMap::new();
    rules.insert('F', "F+F-F-F+F".chars().collect());

    let instructions = rot_forward_instructions(PI / 2.0, 2.0, false);

    (
        LSystem {
            start: "F".chars().collect(),
            rules,
        },
        instructions,
    )
}

pub fn SIERPINSKI() -> (LSystem<char>, HashMap<char, Command>) {
    let mut rules: HashMap<char, Vec<char>> = HashMap::new();
    rules.insert('F', "F-G+F+G-F".chars().collect());
    rules.insert('G', "GG".chars().collect());

    let instructions = rot_forward_instructions(2.0 * PI / 3.0, 3.0, false);

    (
        LSystem {
            start: "F-F-F".chars().collect(),
            rules,
        },
        instructions,
    )
}

pub fn DRAGON() -> (LSystem<char>, HashMap<char, Command>) {
    let mut rules: HashMap<char, Vec<char>> = HashMap::new();
    rules.insert('F', "F+G".chars().collect());
    rules.insert('G', "F-G".chars().collect());

    let instructions = rot_forward_instructions(PI / 2.0, 14.0, true);

    (
        LSystem {
            start: "F".chars().collect(),
            rules,
        },
        instructions,
    )
}
