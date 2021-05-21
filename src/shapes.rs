use macroquad::prelude::*;
use crate::Vec2;

#[allow(dead_code)]
pub struct Rectangle {
	pub position: Vec2,
	pub w: f32,
	pub h: f32,
	pub color: Color,
}

#[allow(dead_code)] 
impl Rectangle {
	pub fn new(position: &Vec2, w: f32, h: f32, color: Color) -> Rectangle {
		let newpos = Vec2{x: position.x, y: position.y};
		return Rectangle{position: newpos, w, h, color};
	}
	pub fn square(position: &Vec2, s: f32, color: Color) -> Rectangle {
		let newpos = Vec2{x: position.x, y: position.y};
		return Rectangle{position: newpos, w: s, h: s, color};
	}
	pub fn draw(&self, lastpos: &Vec2) {
		let last_pos = Vec2{x: lastpos.x, y: lastpos.y};
		if last_pos.x != self.position.x || last_pos.y != self.position.y {
			draw_rectangle(self.position.x, self.position.y, self.w, self.h, self.color);
		}
	}
}