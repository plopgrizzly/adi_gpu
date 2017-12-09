// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Jeron Lau ("Plop Grizzly"), Douglas P Lau
// Licensed under the MIT LICENSE
//
// src/math/points.rs

/*use math::Vec3;
use math::Pos;

pub struct Points {
	pts: Vec<Vec3<f32>>,
}

impl Points {
	pub fn new() -> Points {
		Points {
			pts: Vec::new(),
		}
	}
}

impl Pos for Points {
	/// Add a point and return the handle.
	fn add(&mut self, p: Vec3<f32>) -> u32 {
		self.pts.push(p);
		self.pts.len() as u32
	}

	fn wrt(&mut self, hnd: u32, p: Vec3<f32>) {
		self.pts[hnd as usize - 1] = p;
	}

	fn len(&self) -> usize {
		self.pts.len()
	}

	fn posf(&self, hnd: u32) -> Vec3<f32> {
		self.pts[hnd as usize - 1]
	}

	fn posi(&self, hnd: u32) -> Vec3<i32> {
		self.pts[hnd as usize - 1].into()
	}
}*/
