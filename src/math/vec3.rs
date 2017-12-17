// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Jeron Lau ("Plop Grizzly"), Douglas P Lau
// Licensed under the MIT LICENSE
//
// src/math/vec3.rs

use std::fmt;
use std::ops;
use std::cmp;

/// 3-dimensional vector
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3<T: Copy + Clone> {
	pub x: T,
	pub y: T,
	pub z: T,
}

impl<T> fmt::Debug for Vec3<T> where T: fmt::Debug + Copy + Clone {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({:?},{:?},{:?})", self.x, self.y, self.z)
	}
}

impl<T> ops::Add for Vec3<T> where T: ops::Add<Output=T> + Copy + Clone {
	type Output = Vec3<T>;

	fn add(self, other: Self) -> Self::Output {
		Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
	}
}

impl<T> ops::Sub for Vec3<T> where T: ops::Sub<Output=T> + Copy + Clone {
	type Output = Vec3<T>;

	fn sub(self, other: Self) -> Self::Output {
		Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
	}
}

impl<T> ops::Mul<T> for Vec3<T> where T: ops::Mul<Output=T> + Copy + Clone {
	type Output = Vec3<T>;

	fn mul(self, s: T) -> Self::Output {
		Vec3::new(self.x * s, self.y * s, self.z * s)
	}
}

/*impl<T, U> ops::Mul for Vec3<T> where f64: convert::From<T> {
	type Output = T;

	/// Calculate the cross product of two Vec2
	fn mul(self, other: Self) -> T {
		self.x * other.y - self.y * other.x
	}
}*/

impl<T> ops::Div<T> for Vec3<T> where T: ops::Div<Output=T> + Copy + Clone {
	type Output = Vec3<T>;

	fn div(self, s: T) -> Vec3<T> {
		Vec3::new(self.x / s, self.y / s, self.z / s)
	}
}

impl<T> ops::Neg for Vec3<T> where  T: ops::Neg<Output=T> + Copy + Clone {
	type Output = Vec3<T>;

	fn neg(self) -> Vec3<T> {
		Vec3::new(-self.x, -self.y, -self.z)
	}
}

impl From<Vec3<f32>> for Vec3<i64> {
	fn from(v: Vec3<f32>) -> Self {
		Vec3::new(v.x as i64, v.y as i64, v.z as i64)
	}
}

impl From<Vec3<f32>> for Vec3<i32> {
	fn from(v: Vec3<f32>) -> Self {
		Vec3::new(v.x as i32, v.y as i32, v.z as i32)
	}
}

impl From<Vec3<i64>> for Vec3<f32> {
	fn from(v: Vec3<i64>) -> Self {
		Vec3::new(v.x as f32, v.y as f32, v.z as f32)
	}
}

impl From<Vec3<i32>> for Vec3<f32> {
	fn from(v: Vec3<i32>) -> Self {
		Vec3::new(v.x as f32, v.y as f32, v.z as f32)
	}
}

impl ::math::Pos for Vec3<f32> {
	fn posf(&self) -> Vec3<f32> {
		*self
	}

	fn posi(&self) -> Vec3<i32> {
		(*self).into()
	}
}

impl ::math::Pos for Vec3<i32> {
	fn posi(&self) -> Vec3<i32> {
		*self
	}

	fn posf(&self) -> Vec3<f32> {
		(*self).into()
	}
}

impl<T> Vec3<T> where T: Copy + Clone {
	/// Create a new Vec3
	pub fn new(x: T, y: T, z: T) -> Vec3<T> {
		Vec3 { x, y, z }
	}

	/// Find the minimum ordinal value
	pub(crate) fn min_p(self) -> T where T: cmp::Ord {
		self.x.min(self.y).min(self.z)
	}

	/// Find the maximum ordinal value
	pub(crate) fn max_p(self) -> T where T: cmp::Ord {
		self.x.max(self.y).max(self.z)
	}

	/// Get the magnitude of a Vec3
	pub fn mag(self) -> f32 where Self: Into<Vec3<f32>> {
		let point: Vec3<f32> = self.into();

		(point.x).hypot(point.y).hypot(point.z) as f32
	}
}

impl Vec3<f32> {
	/// Multiply matrix onto Vec3 (as directional vector)
	pub fn transform_dir(self, rhs: ::Transform) -> Self {
		let x = rhs.0[0]*self.x + rhs.0[4]*self.y + rhs.0[8]*self.z;
		let y = rhs.0[1]*self.x + rhs.0[5]*self.y + rhs.0[9]*self.z;
		let z = rhs.0[2]*self.x + rhs.0[6]*self.y + rhs.0[10]*self.z;

		Self::new(x, y, z)
	}

	/// Create a zero Vec3
	pub fn zero() -> Self {
		Vec3::new(0.0, 0.0, 0.0)
	}

	/// Find the midpoint between two Vec3
	pub fn midpoint(self, other: Self) -> Self {
		let x = (self.x + other.x) / 2.0;
		let y = (self.y + other.y) / 2.0;
		let z = (self.z + other.z) / 2.0;
		Vec3::new(x, y, z)
	}

	/// Calculate the distance squared between two Vec3
	pub fn dist_sq(self, other: Self) -> f32 {
		let dx = other.x - self.x;
		let dy = other.y - self.y;
		let dz = other.z - self.z;
		dx * dx + dy * dy + dz * dz
	}

	/// The recipricol (inverse) of the vector.
	pub fn recip(self) -> Self {
		Vec3::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z)
	}

	/// Calculate the dot product of two `Vec3`s
	pub fn dot(&self, other: Vec3<f32>) -> f32 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	/// Normalize a Vec3
	pub fn normalize(self) -> Self {
		let m = self.mag();
		if m > 0.0 {
			self / m
		} else {
			Vec3::zero()
		}
	}

	/// Calculate angle between 2 Vec3's
	pub fn angle(&self, other: Vec3<f32>) -> f32 {
		let mag1 = (self.x as f64)
			.hypot(self.y as f64)
			.hypot(self.z as f64);
		let mag2 = (other.x as f64)
			.hypot(other.y as f64)
			.hypot(other.z as f64);
		let dot = ((self.x as f64) * (other.x as f64))
			+ ((self.y as f64) * (other.y as f64))
			+ ((self.z as f64) * (other.z as f64));

		(dot / (mag1 * mag2)).acos() as f32
	}
}
