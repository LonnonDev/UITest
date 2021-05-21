#![windows_subsystem = "windows"]
use macroquad::prelude::*;
mod shapes;
use crate::shapes::Rectangle;

pub struct Vec2 {
	pub x: f32,
	pub y: f32
}

const SIZE: f32 = 20.0;
const MOVESPEED: f32 = 10.0;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Stolen Lol"),
        window_width: 800,
        window_height: 600,
        high_dpi: false,
        fullscreen: false,
        sample_count: 2,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
	let mut player_pos = Vec2 {
		x: 20.0,
		y: 20.0
	};
	let mut player_character = Rectangle::square(&player_pos, SIZE, BLUE);
	let mut _lastpos = Vec2 {
		x: 0.0,
		y: 0.0
	};
	let mut currentkeys: u32 = 4;
	loop {
		let tmp = movement(currentkeys);
		let player_direction = tmp.0;
		currentkeys = tmp.1;
		println!("{}", currentkeys);
		clear_background(RED);
		//# Get player pos
		player_pos.x = player_character.position.x+player_direction.x;
		player_pos.y = player_character.position.y+player_direction.y;
		//# create player character and draw it
		player_character = Rectangle::square(&player_pos, SIZE, BLUE);
		player_character.draw(&_lastpos);
		let mut _lastpos = Vec2 {x: player_character.position.x, y: player_character.position.y};

		next_frame().await;
	}
}

fn movement(currentkeys: u32) -> (Vec2, u32) {
	let mut plymove = Vec2 {
		x: 0.0,
		y: 0.0
	};
	let mut currentkeys: u32 = currentkeys;
	//# Get the key that's pressed down and then changed plymove to that position
	if is_key_down(KeyCode::W) { plymove.y = -MOVESPEED; currentkeys = currentkeys + 1; } else { currentkeys = currentkeys - 1; }
	if is_key_down(KeyCode::S) { plymove.y = MOVESPEED; currentkeys = currentkeys + 1; } else { currentkeys = currentkeys - 1; }
	if is_key_down(KeyCode::A) { plymove.x = -MOVESPEED; currentkeys = currentkeys + 1; } else { currentkeys = currentkeys - 1; }
	if is_key_down(KeyCode::D) { plymove.x = MOVESPEED; currentkeys = currentkeys + 1; } else { currentkeys = currentkeys - 1; }

	return (plymove, currentkeys)
}