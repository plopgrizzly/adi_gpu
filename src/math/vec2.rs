// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Jeron Lau ("Plop Grizzly"), Douglas P Lau
// Licensed under the MIT LICENSE
//
// src/math/vec2.rs

use std::fmt;
use std::ops;

// Declare floating point type to use
type Float = f32;

/// Calculate linear interpolation of two values
///
/// The t value should be between 0 and 1.
pub fn float_lerp(a: Float, b: Float, t: Float) -> Float {
	b + (a - b) * t
}

/// Calculate intersection point of two lines.
///
/// Returns None if the lines are colinear.
#[allow(unused)]
pub fn intersection(a0: Vec2, a1: Vec2, b0: Vec2, b1: Vec2) -> Option<Vec2> {
	let av = a0 - a1;
	let bv = b0 - b1;
	let den = av * bv;
	if den != 0 as Float {
		let ca = a0 * a1;
		let cb = b0 * b1;
		let xn = bv.x * ca - av.x * cb;
		let yn = bv.y * ca - av.y * cb;
		Some(Vec2::new(xn / den, yn / den))
	} else {
		None
	}
}

/// 2-dimensional vector
#[derive(Clone, Copy, PartialEq)]
pub struct Vec2 {
	pub x: Float,
	pub y: Float,
}

impl fmt::Debug for Vec2 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({},{})", self.x, self.y)
	}
}

impl ops::Add for Vec2 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Vec2::new(self.x + other.x, self.y + other.y)
	}
}

impl ops::Sub for Vec2 {
	type Output = Self;

	fn sub(self, other: Self) -> Self{
		Vec2::new(self.x - other.x, self.y - other.y)
	}
}

impl ops::Mul<Float> for Vec2 {
	type Output = Self;

	fn mul(self, s: Float) -> Self {
		Vec2::new(self.x * s, self.y * s)
	}
}

impl ops::Mul for Vec2 {
	type Output = Float;

	/// Calculate the cross product of two Vec2
	fn mul(self, other: Self) -> Float {
		self.x * other.y - self.y * other.x
	}
}

impl ops::Div<Float> for Vec2 {
	type Output = Self;

	fn div(self, s: Float) -> Self {
		Vec2::new(self.x / s, self.y / s)
	}
}

impl ops::Neg for Vec2 {
	type Output = Self;

	fn neg(self) -> Self {
		Vec2::new(-self.x, -self.y)
	}
}

#[allow(unused)]
impl Vec2 {
	/// Create a new Vec2
	pub fn new(x: Float, y: Float) -> Self {
		Vec2 { x: x, y: y }
	}

	/// Create a zero Vec2
	pub fn zero() -> Self {
		Vec2::new(0 as Float, 0 as Float)
	}

	/// Calculate the dot product of two `Vec2`s
	pub fn dot(&self, other: Vec2) -> f32 {
		self.x * other.x + self.y * other.y
	}

	/// Get the magnitude of a Vec2
	pub fn mag(self) -> Float {
		self.x.hypot(self.y)
	}

	/// Normalize a Vec2
	pub fn normalize(self) -> Self {
		let m = self.mag();
		if m > 0.0 {
			self / m
		} else {
			Vec2::zero()
		}
	}

	/// Calculate the distance squared between two Vec2
	pub fn dist_sq(self, other: Self) -> Float {
		let dx = self.x - other.x;
		let dy = self.y - other.y;
		dx * dx + dy * dy
	}

	/// Calculate the distance between two Vec2
	pub fn dist(self, other: Self) -> Float {
		self.dist_sq(other).sqrt()
	}

	/// Find the midpoint between two Vec2
	pub fn midpoint(self, other: Self) -> Self {
		let x = (self.x + other.x) / 2 as Float;
		let y = (self.y + other.y) / 2 as Float;
		Vec2::new(x, y)
	}

	/// Create a left-hand perpendicular Vec2
	pub fn left(self) -> Self {
		Vec2::new(-self.y, self.x)
	}

	/// Create a right-hand perpendicular Vec2
	pub fn right(self) -> Self {
		Vec2::new(self.y, -self.x)
	}

	/// Calculate winding order for two Vec2.
	///
	/// The Vec2 should be initialized as edges pointing toward the same vertex.
	/// Returns true if the winding order is widdershins (counter-clockwise).
	pub fn widdershins(self, other: Self) -> bool {
		// Cross product (with Z zero) is used to determine the winding order.
		(self.x * other.y) > (other.x * self.y)
	}

	/// Calculate linear interpolation of two Vec2
	pub fn lerp(self, other: Self, t: Float) -> Self {
		let x = float_lerp(self.x, other.x, t);
		let y = float_lerp(self.y, other.y, t);
		Vec2::new(x, y)
	}

	/// Calculate angle between 2 Vec2's (on a plane they both lie on)
	pub fn angle(&self, other: Vec2) -> f32 {
		(self.dot(other) / (self.mag() * other.mag())).acos()
	}
}

#[test]
fn test_vec2() {
	let a = Vec2::new(2f32, 1f32);
	let b = Vec2::new(3f32, 4f32);
	assert!(a + b == Vec2::new(5f32, 5f32));
	assert!(b - a == Vec2::new(1f32, 3f32));
	assert!(a * 2f32 == Vec2::new(4f32, 2f32));
	assert!(a / 2f32 == Vec2::new(1f32, 0.5f32));
	assert!(-a == Vec2::new(-2f32, -1f32));
	assert!(b.mag() == 5f32);
	assert!(a.normalize() == Vec2::new(0.8944272f32, 0.4472136f32));
	assert!(a.dist_sq(b) == 10f32);
	assert!(b.dist(Vec2::new(0f32, 0f32)) == 5f32);
	assert!(a.midpoint(b) == Vec2::new(2.5f32, 2.5f32));
	assert!(a.left() == Vec2::new(-1f32, 2f32));
	assert!(a.right() == Vec2::new(1f32, -2f32));
}
