// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Jeron Lau ("Plop Grizzly"), Douglas P Lau
// Licensed under the MIT LICENSE
//
// src/math/pos.rs

use math::Vec3;

/// Pos trait allows point lookup by handle
pub trait Pos {
/*	fn add(&mut self, p: Vec3<f32>) -> u32;
	fn wrt(&mut self, hnd: u32, p: Vec3<f32>);
	fn len(&self) -> usize;*/
	fn posf(&self/*, hnd: u32*/) -> Vec3<f32>;
	fn posi(&self/*, hnd: u32*/) -> Vec3<i32>;
}
