// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Jeron Lau ("Plop Grizzly"), Douglas P Lau
// Licensed under the MIT LICENSE
//
// src/math/vec4.rs

use std::fmt;
use std::cmp;

/// 4-dimensional vector
#[derive(Clone, Copy, PartialEq)]
pub struct Vec4<T: Copy + Clone> {
	pub x: T,
	pub y: T,
	pub z: T,
	pub w: T,
}

impl<T> fmt::Debug for Vec4<T> where T: fmt::Debug + Copy + Clone {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"({:?},{:?},{:?},{:?})",self.x,self.y,self.z,self.w)
	}
}

#[allow(unused)]
impl<T> Vec4<T> where T: Copy + Clone {
	/// Create a new Vec3
	pub fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
		Vec4 { x, y, z, w }
	}

	/// Find the minimum ordinal value
	pub(crate) fn min_p(self) -> T where T: cmp::Ord {
		self.x.min(self.y).min(self.z).min(self.w)
	}

	/// Find the maximum ordinal value
	pub(crate) fn max_p(self) -> T where T: cmp::Ord {
		self.x.max(self.y).max(self.z).max(self.w)
	}
}
