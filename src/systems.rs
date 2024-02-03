use std::collections::HashMap;
use std::f32::consts::PI;

use crate::lsystem::*;
use crate::turtle_graphics::Command::{self, *};
use crate::utils;

fn rot_forward_instructions(
    angle: f32,
    scale: f32,
    g_means_draw: bool,
) -> HashMap<char, Vec<Command>> {
    let mut instructions = HashMap::new();
    instructions.insert('F', vec![DrawForward(scale)]);
    instructions.insert(
        'G',
        if g_means_draw {
            vec![DrawForward(scale)]
        } else {
            vec![MoveForward(scale)]
        },
    );
    instructions.insert('+', vec![Turn(angle)]);
    instructions.insert('-', vec![Turn(-angle)]);

    instructions
}

fn rot_forward_stack_instructions(
    angle: f32,
    scale: f32,
    g_means_draw: bool,
) -> HashMap<char, Vec<Command>> {
    let mut instructions = rot_forward_instructions(angle, scale, g_means_draw);
    instructions.insert('[', vec![Push]);
    instructions.insert(']', vec![Pop]);

    instructions
}

fn rot_forward_stack_and_turn_instructions(angle: f32, scale: f32) -> HashMap<char, Vec<Command>> {
    let mut instructions = HashMap::new();
    instructions.insert('[', vec![Push, Turn(angle)]);
    instructions.insert(']', vec![Pop, Turn(-angle)]);
    instructions.insert('F', vec![DrawForward(scale)]);
    instructions.insert('G', vec![DrawForward(scale)]);

    instructions
}

pub fn KOCH() -> (LSystem<char>, HashMap<char, Vec<Command>>) {
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

pub fn SIERPINSKI() -> (LSystem<char>, HashMap<char, Vec<Command>>) {
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

pub fn DRAGON() -> (LSystem<char>, HashMap<char, Vec<Command>>) {
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

pub fn FRACTAL_PLANT() -> (LSystem<char>, HashMap<char, Vec<Command>>) {
    let mut rules: HashMap<char, Vec<char>> = HashMap::new();
    rules.insert('X', "F+[[X]-X]-F[-FX]+X".chars().collect());
    rules.insert('F', "FF".chars().collect());

    let instructions = rot_forward_stack_instructions(0.4363, 5.0, true);

    (
        LSystem {
            start: "X".chars().collect(),
            rules,
        },
        instructions,
    )
}

pub fn SIMPLE_TREE() -> (LSystem<char>, HashMap<char, Vec<Command>>) {
    let mut rules: HashMap<char, Vec<char>> = HashMap::new();
    rules.insert('F', "FF".chars().collect());
    rules.insert('G', "F[G]G".chars().collect());

    let instructions = rot_forward_stack_and_turn_instructions(PI / 4.0, 0.625);

    (
        LSystem {
            start: "G".chars().collect(),
            rules,
        },
        instructions,
    )
}

pub fn BUSH() -> (LSystem<char>, HashMap<char, Vec<Command>>) {
    let mut rules = HashMap::new();
    rules.insert('X', "X[-FFF][+FFF]FX".chars().collect());
    rules.insert('Y', "YFX[+Y][-Y]".chars().collect());

    let instructions = rot_forward_stack_instructions(utils::degree_to_rad(25.7), 5.0, true);

    (
        LSystem {
            start: "Y".chars().collect(),
            rules,
        },
        instructions,
    )
}

pub fn SAUPE_TREE() -> (LSystem<char>, HashMap<char, Vec<Command>>) {
    let mut rules = HashMap::new();
    rules.insert('V', "[+++W][---W]YV".chars().collect());
    rules.insert('W', "+X[-W]Z".chars().collect());
    rules.insert('X', "-W[+X]Z".chars().collect());
    rules.insert('Y', "YZ".chars().collect());
    rules.insert('Z', "[-FFF][+FFF]F".chars().collect());
    

    let instructions = rot_forward_stack_instructions(utils::degree_to_rad(20.0), 40.0, true);

    (
        LSystem {
            start: "VZFFF".chars().collect(),
            rules,
        },
        instructions,
    )
}
