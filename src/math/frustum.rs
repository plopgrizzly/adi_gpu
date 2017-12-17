// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Jeron Lau ("Plop Grizzly")
// Licensed under the MIT LICENSE
//
// src/octree/frustum.rs

use std::fmt;

use math::Vec3;
use math::BBox;
// use math::Plane;

#[derive(Clone, Copy, PartialEq)]
pub struct Frustum {
	pub center: Vec3<f32>,
	pub radius: f32,
	pub wfov: f32,
	pub hfov: f32,
	pub xrot: f32,
	pub yrot: f32,
}

impl fmt::Debug for Frustum {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "(radius: {:?})", self.radius)
	}
}

impl Frustum {
	/// Create a new viewing frustum.
	///
	/// * `center` - The center of the frustum cone.
	/// * `radius` - How far can you see?
	/// * `xrot` - Direction facing on x axis (radians).
	/// * `yrot` - Direction facing on y axis (radians).
	/// * `wfov` - The fov on the X axis (radians).
	/// * `hfov` - The fov on the Y axis (radians).
	pub fn new(center: Vec3<f32>, radius: f32, xrot: f32, yrot: f32,
		wfov: f32, hfov: f32) -> Frustum
	{
/*		let xmax = far / (wfov / 2.0).tan();
		let ymax = far / (hfov / 2.0).tan();

		let rightfar = Vec3::new(xmax, 0.0, far);
		let leftfar = Vec3::new(-xmax, 0.0, far);
		let topfar = Vec3::new(0.0, -ymax, far);
		let bottomfar = Vec3::new(0.0, ymax, far);
//		let camera = Vec3::new(0.0, 0.0, -ar);

		let wdist = ((::std::f32::consts::PI - wfov) / 2.0).sin() * -xmax;
		let hdist = ((::std::f32::consts::PI - hfov) / 2.0).sin() * -ymax;

		let top = Plane::new(bottomfar, hdist);
		let bottom = Plane::new(topfar, hdist);
		let right = Plane::new(leftfar, wdist);
		let left = Plane::new(rightfar, wdist);
		let near = Plane::new(Vec3::new(0.0, 0.0, 1.0), 0.0);
		let far = Plane::new(Vec3::new(0.0, 0.0, -1.0), -far);

		Frustum { near, far, top, bottom, right, left }*/

		Frustum { center, radius, xrot, yrot, wfov, hfov }
	}

	/// If viewing frustum collides with the bounding box.
	pub fn collide_bbox(&self, bbox: BBox<i32>) -> bool {
		for i in bbox.all_points().iter() {
			let point : Vec3<f32> = (*i).into();

			if (point - self.center).mag() <= self.radius {
				return true;
			}
		}

		false

/*		let top = self.top;
		let bottom = self.bottom;
		let right = self.right;
		let left = self.left;
		let near = self.near;
		let far = self.far;*/

/*		let planes = [self.top, self.bottom, self.right, self.left,
			self.near, self.far];

		for plane in planes.iter() {
			let (a, b) = bbox.pn_pair_from_normal(plane.facing);

			if !plane.isdistpos_point(a) && !plane.isdistpos_point(b) {
				return false;
			}
		}*/

/*		// All 6 planes must have a point within their area.
		top.isdistpos_bbox(bbox) && bottom.isdistpos_bbox(bbox) &&
			right.isdistpos_bbox(bbox) && left.isdistpos_bbox(bbox)
			&& near.isdistpos_bbox(bbox) && far.isdistpos_bbox(bbox)*/
	}

	/// If viewing frustum collides with a point.
	pub fn collide_point(&self, point: Vec3<f32>) -> bool {
		(point - self.center).mag() <= self.radius

/*		self.near.isdistpos_point(point)
			&& self.far.isdistpos_point(point)
			&& self.left.isdistpos_point(point)
			&& self.right.isdistpos_point(point)
			&& self.top.isdistpos_point(point)
			&& self.bottom.isdistpos_point(point)*/
	}
}
