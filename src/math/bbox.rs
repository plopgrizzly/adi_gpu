// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Jeron Lau ("Plop Grizzly"), Douglas P Lau
// Licensed under the MIT LICENSE
//
// src/math/bbox.rs

use std::fmt;
use std::cmp;
use std::ops;

use math::Vec3;

/// Bounding box
#[derive(Clone, Copy)]
pub struct BBox<T: Copy + Clone + From<i32> + ops::Neg<Output=T>> {
	pub center: Vec3<T>,
	pub half_len: T,
}

impl<T> fmt::Debug for BBox<T> where T: fmt::Debug + Copy + Clone + From<i32> + ops::Neg<Output=T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}±{:?}", self.center, self.half_len)
	}
}

impl<T> BBox<T> where T: Copy + Clone + From<i32> + ops::Add<Output = T>
	+ ops::Sub<Output = T> + ops::AddAssign + ops::SubAssign
	+ ops::MulAssign + ops::DivAssign + ops::Mul<Output = T>
	+ ops::Div<Output = T> + cmp::PartialOrd + ops::Neg<Output=T>
	+ cmp::Ord
{
	pub fn empty() -> BBox<T> {
		let z = T::from(0);

		BBox { center: Vec3::new(z, z, z), half_len: T::from(-1) }
	}

	pub fn new(p: Vec3<T>) -> BBox<T> {
		BBox { center: p, half_len: T::from(1) }
	}

	fn min_p(&self) -> T {
		if self.half_len > T::from(0) {
			self.center.min_p() - self.half_len
		} else {
			self.center.min_p()
		}
	}

	fn max_p(&self) -> T {
		if self.half_len > T::from(0) {
			self.center.max_p() + self.half_len
		} else {
			self.center.max_p()
		}
	}

	pub fn extend(&mut self, p: Vec3<T>) {
		self.center = self.move_center(p);
		self.half_len *= T::from(2);
	}

	fn move_center(&self, p: Vec3<T>) -> Vec3<T> {
		let min_p = self.min_p();
		if p.min_p() < min_p {
			return Vec3::new(min_p, min_p, min_p);
		} else {
			let max_p = self.max_p();
			return Vec3::new(max_p, max_p, max_p);
		}
	}

	pub fn contains(&self, p: Vec3<T>) -> bool {
		let Vec3 { x, y, z } = self.center;
		let hl = self.half_len;
		(p.x >= x - hl) &&
		(p.x <  x + hl) &&
		(p.y >= y - hl) &&
		(p.y <  y + hl) &&
		(p.z >= z - hl) &&
		(p.z <  z + hl)
	}

	/// Get two opposite points that are the bounds of the BBox.
	pub fn to_point_pair(&self) -> (Vec3<T>, Vec3<T>) {
		let half_box = Vec3::new(self.half_len, self.half_len,
			self.half_len);

		(self.center + half_box, self.center - half_box)
	}

	/// Get all 6 points or the `BBox`.
	pub fn all_points(&self) -> [Vec3<T>; 7] {
		let z = T::from(0);

		[
			self.center,
			self.center + Vec3::new(self.half_len, z, z),
			self.center + Vec3::new(z, self.half_len, z),
			self.center + Vec3::new(z, z, self.half_len),
			self.center + Vec3::new(-self.half_len, z, z),
			self.center + Vec3::new(z, -self.half_len, z),
			self.center + Vec3::new(z, z, -self.half_len),
		]
	}

	/// Get a positive and negative pair of opposite points that are the
	/// bounds of the BBox, based around a normal.
	pub fn pn_pair_from_normal(&self, normal: Vec3<f32>)
		-> (Vec3<T>, Vec3<T>)
	{
		let mut pvertex = self.center;
		let mut nvertex = self.center;

		if normal.x >= 0.0 {
			pvertex.x += self.half_len;
			nvertex.x -= self.half_len;
		} else {
			nvertex.x += self.half_len;
			pvertex.x -= self.half_len;
		}

		if normal.y >= 0.0 {
			pvertex.y += self.half_len;
			nvertex.y -= self.half_len;
		} else {
			nvertex.y += self.half_len;
			pvertex.y -= self.half_len;
		}

		if normal.z >= 0.0 {
			pvertex.z += self.half_len;
			nvertex.z -= self.half_len;
		} else {
			nvertex.z += self.half_len;
			pvertex.z -= self.half_len;
		}

		(nvertex, pvertex)
	}
}
