/// octree.rs    A simple octree for spatial searching.
///
/// Copyright (c) 2017  Douglas P Lau
use std::fmt;
use std::cmp::Ordering;

use math::Vec3;
use math::BBox;
use math::Pos;
use math::Frustum;

type Vec3i = Vec3<i32>;

/// An octree is a DAG that can quickly search for points in 3D space.
///
/// The bounding box of the root node contains all points in the octree.
/// If a point outside the bounding box is added, a new root node is created
/// which contains the old root as one of its octants.  This process is repeated
/// until the point is contained.
///
/// The nodes are stored in a vector, and are indexed using a 32-bit node ID.
/// This saves memory over using pointers on 64-bit systems.  Node ID 1 is the
/// first node in the vector.
///
pub struct Octree<T: Pos> {
	points: Vec<T>,
	point_garbage: Vec<u32>,
	nodes: Vec<Node>,
	garbage: Vec<u32>,
	bbox: BBox<i32>,
	root: usize,
	n_points: u32,
}

const LINK: usize = 7;		// link to coincident leaf nodes
const LEAF: u32 = 0xFFFFFFFF;	// max u32 value (invalid handle)

/// A node can be either a branch or a leaf.
///
/// A branch can have up to 8 child nodes (each octant adjacent to the center).
///
/// A leaf can store up to 6 points; the first child must contain a LEAF
/// sentinel value, and the last may link to another leaf node with only
/// coincident points.
///
/// Each node has an implicit bounding box determined by its position in the
/// octree.  The bounding box contains all descendant nodes.
///
struct Node {
	child: [u32; 8],	// child node handles
}

impl fmt::Debug for Node {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.is_leaf() {
			try!(write!(f, "leaf: {:?}", self.leaf_children()));
			let l = self.link();
			if l > 0 {
				try!(write!(f, " link: {:?}", l));
			}
			Ok(())
		} else {
			write!(f, "branch: {:?}", self.child)
		}
	}
}

impl Node {
	/// Create a new leaf node
	fn new_leaf() -> Node {
		Node {
			child: [LEAF, 0, 0, 0, 0, 0, 0, 0],
		}
	}
	/// Create a new branch node
	fn new_branch() -> Node {
		Node { child: [0; 8], }
	}
	/// Test if a node is empty
	fn is_empty(&self) -> bool {
		self.child == [LEAF, 0, 0, 0, 0, 0, 0, 0] ||
		self.child == [0; 8]
	}
	/// Test if a node is a leaf
	fn is_leaf(&self) -> bool {
		self.child[0] == LEAF
	}
	/// Test if a node is a branch
	fn is_branch(&self) -> bool {
		!self.is_leaf()
	}
	/// Get link to next link node ID
	fn link(&self) -> usize {
		assert!(self.is_leaf());
		self.child[LINK] as usize
	}
	/// Find the first empty child slot
	fn empty_slot(&self) -> Option<usize> {
		self.child.iter().position(|v| *v == 0)
	}
	/// Find the first open child slot in a leaf
	fn open_slot(&self) -> Option<usize> {
		assert!(self.is_leaf());
		let slot = self.empty_slot();
		if let Some(s) = slot {
			if s < 7 {
				return slot;
			}
		}
		None
	}
	/// Check if a leaf node is full
	fn is_full(&self) -> bool {
		assert!(self.is_leaf());
		match self.open_slot() {
			Some(_) => false,
			None => true,
		}
	}
	/// Check if all points are coincident with the given point
	fn all_coincident<T>(&self, p: Vec3i, octree: &Octree<T>) -> bool where T: Pos {
		assert!(self.is_leaf());
		assert!(self.is_full());
		p == octree[self.child[1]].posi() &&
		p == octree[self.child[2]].posi() &&
		p == octree[self.child[3]].posi() &&
		p == octree[self.child[4]].posi() &&
		p == octree[self.child[5]].posi() &&
		p == octree[self.child[6]].posi()
	}
	/// Add a point to a leaf node
	fn add_leaf(&mut self, hnd: u32) {
		assert!(self.is_leaf());
		assert!(!self.is_full());
		let s = self.open_slot().unwrap();
		self.child[s] = hnd;
	}
	/// Remove a point from a leaf node
	fn remove_leaf(&mut self, hnd: u32) -> bool {
		assert!(self.is_leaf());

		for i in 1..7 {
			if self.child[i] == hnd {
				self.child[i] = 0;
				return true;
			}
		}

		false
	}
	/// Get an array containing the leaf children
	fn leaf_children(&self) -> [u32; 6] {
		assert!(self.is_leaf());

		[self.child[1], self.child[2], self.child[3], self.child[4],
			self.child[5], self.child[6]]
	}
	/// Determine which child for a branch point
	fn which_child(c: Vec3i, p: Vec3i) -> usize {
		match (p.x < c.x, p.y < c.y, p.z < c.z) {
			(true,  true,  true)  => 0,
			(true,  true,  false) => 1,
			(true,  false, true)  => 2,
			(true,  false, false) => 3,
			(false, true,  true)  => 4,
			(false, true,  false) => 5,
			(false, false, true)  => 6,
			(false, false, false) => 7,
		}
	}
	/// Calculate the center of a child node
	fn child_center(ch: usize, c: Vec3i, h: i32) -> Vec3i {
		let h = if h == 0 { 1 } else { h };

		match ch {
			0 => Vec3i::new(c.x - h, c.y - h, c.z - h),
			1 => Vec3i::new(c.x - h, c.y - h, c.z + h),
			2 => Vec3i::new(c.x - h, c.y + h, c.z - h),
			3 => Vec3i::new(c.x - h, c.y + h, c.z + h),
			4 => Vec3i::new(c.x + h, c.y - h, c.z - h),
			5 => Vec3i::new(c.x + h, c.y - h, c.z + h),
			6 => Vec3i::new(c.x + h, c.y + h, c.z - h),
			_ => Vec3i::new(c.x + h, c.y + h, c.z + h),
		}
	}
	/// Calculate the bounding box of a child node
	fn child_bbox(ch: usize, bbox: BBox<i32>) -> BBox<i32> {
		assert!(bbox.half_len > 0);
		let half_len = bbox.half_len / 2;
		let center = Node::child_center(ch, bbox.center, half_len);
		BBox { center: center, half_len: half_len }
	}
}

impl<T> Octree<T> where T: Pos {
	/// Create a new octree
	pub fn new() -> Octree<T> {
		Octree {
			points: Vec::new(),
			point_garbage: Vec::new(),
			nodes: Vec::new(),
			garbage: Vec::new(),
			bbox: BBox::empty(),
			root: 0,
			n_points: 0,
		}
	}
	/// Add a new node
	fn new_node(&mut self, n: Node) -> usize {
		if let Some(i) = self.garbage.pop() {
			let k = i as usize;
			self.nodes[k - 1] = n;
			k
		} else {
			self.nodes.push(n);
			self.nodes.len()
		}
	}
	/// Add a new leaf node
	fn new_leaf(&mut self) -> usize {
		self.new_node(Node::new_leaf())
	}
	/// Add a new branch node
	fn new_branch(&mut self) -> usize {
		self.new_node(Node::new_branch())
	}
	/// Add a point in the octree
	pub fn add(&mut self, point: T) -> u32 {
		let hnd = if let Some(hnd) = self.point_garbage.pop() {
			self.points[hnd as usize - 1] = point;
			hnd
		} else {
			self.points.push(point);
			self.points.len() as u32
		};

		match self.n_points {
			0 => self.add_0(hnd),
			_ => self.add_n(hnd),
		}

		hnd
	}
	/// Add a point when empty
	fn add_0(&mut self, hnd: u32) {
		assert!(self.n_points == 0);
		let p = self[hnd].posi();
		self.nodes.clear();
		self.garbage.clear();
		self.point_garbage.clear();
		let i = self.new_leaf();
		self.nodes[i - 1].add_leaf(hnd);
		self.bbox = BBox::new(p);
		self.root = 1;
		self.n_points = 1;
	}
	/// Add a point when not empty
	fn add_n(&mut self, hnd: u32) {
		assert!(self.n_points > 0);
		let p = self[hnd].posi();
		while !self.bbox.contains(p) {
			self.grow_root(p);
		}
		self.add_inside(hnd);
	}
	/// Grow the root node
	fn grow_root(&mut self, p: Vec3i) {
		assert!(!self.bbox.contains(p));
		let center = self.bbox.center;
		let i = self.root - 1;
		self.bbox.extend(p);
		if self.nodes[i].is_branch() {
			let ch = Node::which_child(self.bbox.center, center);
			let k = self.new_branch();
			self.nodes[k - 1].child[ch] = self.root as u32;
			self.root = k;
		}
	}
	/// Add a point within the bounds
	fn add_inside(&mut self, hnd: u32) {
		let p = self[hnd].posi();
		assert!(self.bbox.contains(p));
		let (mut i, mut bbox) = self.find_leaf_grow(p);
		while self.nodes[i].is_full() {
			let (j, bb) = self.grow_leaf(i, bbox, p);
			i = j;
			bbox = bb;
		}
		self.nodes[i].add_leaf(hnd);
		self.n_points += 1;
	}
	/// Find the leaf node for a point (grow it if necessary)
	fn find_leaf_grow(&mut self, p: Vec3i) -> (usize, BBox<i32>) {
		assert!(self.bbox.contains(p));
		let mut i = self.root - 1;
		let mut bbox = self.bbox;
		while self.nodes[i].is_branch() {
			let (j, bb) = self.follow_branch_grow(i, bbox, p);
			i = j;
			bbox = bb;
		}
		(i, bbox)
	}
	/// Follow a branch or grow a leaf node
	fn follow_branch_grow(&mut self, i: usize, bbox: BBox<i32>, p: Vec3i) ->
		(usize, BBox<i32>)
	{
		assert!(self.nodes[i].is_branch());
		let ch = Node::which_child(bbox.center, p);
		let j = self.nodes[i].child[ch] as usize;
		let bb = Node::child_bbox(ch, bbox);
		if j > 0 {
			(j - 1, bb)
		} else {
			let k = self.new_leaf();
			self.nodes[i].child[ch] = k as u32;
			(k - 1, bb)
		}
	}
	/// Grow a leaf node into a branch or link
	fn grow_leaf(&mut self, i: usize, bbox: BBox<i32>, p: Vec3i)
		-> (usize, BBox<i32>)
	{
		assert!(self.nodes[i].is_leaf());
		assert!(self.nodes[i].is_full());
		if self.nodes[i].all_coincident(p, &self) {
			self.grow_leaf_link(i, bbox)
		} else {
			self.grow_leaf_branch(i, bbox.center);
			self.follow_branch_grow(i, bbox, p)
		}
	}
	/// Grow a leaf node linking to another leaf
	fn grow_leaf_link(&mut self, i: usize, bbox: BBox<i32>)
		-> (usize, BBox<i32>)
	{
		assert!(self.nodes[i].is_leaf());
		assert!(self.nodes[i].is_full());
		let j = self.nodes[i].link();
		if j > 0 {
			(j - 1, bbox)
		} else {
			let k = self.new_leaf();
			// Link to new coincident leaf
			self.nodes[i].child[LINK] = k as u32;
			(k - 1, bbox)
		}
	}
	/// Grow a full leaf into a branch
	fn grow_leaf_branch(&mut self, i: usize, center: Vec3i) {
		assert!(self.nodes[i].is_leaf());
		assert!(self.nodes[i].is_full());
		let mut br = Node::new_branch();
		let link = self.nodes[i].link() as u32;
		for hnd in self.nodes[i].leaf_children().iter() {
			if *hnd < 1 { continue; }

			let p = self[*hnd].posi();
			let ch = Node::which_child(center, p);
			let j = br.child[ch] as usize;
			if j > 0 {
				// NOTE: if there is a link, all children
				//       must be coincident
				assert!(self.nodes[j - 1].link() as u32 == link);
				self.nodes[j - 1].add_leaf(*hnd);
			} else {
				let k = self.new_leaf();
				// Preserve link to coincident leaves
				self.nodes[k - 1].child[LINK] = link;
				self.nodes[k - 1].add_leaf(*hnd);
				br.child[ch] = k as u32;
			}
		}
		self.nodes[i] = br;
	}
	/// Remove a point from the octree
	pub fn remove(&mut self, hnd: u32) {
		assert!(self.n_points > 0);
		assert!(self.root > 0);

		let i = self.root - 1;
		let bbox = self.bbox;
		let p = self[hnd].posi();
		self.remove_point(i, bbox, hnd, p);
		self.point_garbage.push(hnd);
	}
	/// Remove a point within a bounding box
	fn remove_point(&mut self, i: usize, bbox: BBox<i32>, hnd: u32, p: Vec3i) {
		if self.nodes[i].is_branch() {
			self.remove_branch(i, bbox, hnd, p);
		} else {
			self.remove_leaf(i, hnd);
		}
	}
	/// Remove a point from a branch
	fn remove_branch(&mut self, i: usize, bbox: BBox<i32>, hnd: u32, p: Vec3i) {
		assert!(self.nodes[i].is_branch());
		let ch = Node::which_child(bbox.center, p);
		let j = self.nodes[i].child[ch];
		if j > 0 {
			let k = (j - 1) as usize;
			let bb = Node::child_bbox(ch, bbox);
			self.remove_point(k, bb, hnd, p);
			if self.nodes[k].is_empty() {
				self.nodes[i].child[ch] = 0;
				self.garbage.push(j);
			}
		}
	}
	/// Remove a point from a leaf
	fn remove_leaf(&mut self, i: usize, hnd: u32) {
		assert!(self.nodes[i].is_leaf());
		if self.nodes[i].remove_leaf(hnd) {
			self.n_points -= 1;
		} else {
			let l = self.nodes[i].link();

			if l > 0 {
				self.remove_leaf(l - 1, hnd);
			} else {
//				self.print();
//				panic!("Couldn't find hnd {} ({:?}) in {}!",
//					hnd, self[hnd].posi(), i);
			}
		}
	}
	/// Modify a point in the octree.
	pub fn modify(&mut self, hnd: &mut u32, p: T) {
		self.remove(*hnd);
		*hnd = self.add(p);
	}
	/// Find node children
	fn find_node_ch(&mut self, sorted: &mut Vec<u32>, i: usize,
		frustum: Frustum, bbox: BBox<i32>)
	{
		if self.nodes[i].is_leaf() {
			for hnd in self.nodes[i].leaf_children().iter() {
				if *hnd < 1 { continue; }

				if frustum.collide_point(
					self[*hnd].posf())
				{
					sorted.push(*hnd);
				}
			}
			let j = self.nodes[i].link();
			if j > 0 {
				self.find_node_ch(sorted, j - 1, frustum, bbox);
			}
		} else {
			for ch in 0 .. 8 {
//				let bb = Node::child_bbox(ch, bbox);

//				if frustum.collide_bbox(bb) {
					let k = self.nodes[i as usize]
						.child[ch as usize] as usize;
					if k > 0 {
						self.find_node_ch(sorted, k - 1,
							frustum, bbox);
					}
//				}
			}
		}
	}
	/// Sort by z value.  nr => true if Near Sort, nr => false if Far Sort
	fn zsort(&mut self, sorted: &mut Vec<u32>, nr: bool, frustum: Frustum) {
		sorted.clear();

		if self.root == 0 {
			return;
		}

		let hnd = self.root - 1;
		let bbox = self.bbox;

		self.find_node_ch(sorted, hnd, frustum, bbox);

		sorted.sort_unstable_by(|a, b| {
			let p1 = self[*a].posf() - frustum.center;
			let p2 = self[*b].posf() - frustum.center;

			if p1.mag() > p2.mag() {
				if nr {Ordering::Greater} else {Ordering::Less}
			} else if p1.mag() < p2.mag() {
				if nr {Ordering::Less} else {Ordering::Greater}
			} else {
				Ordering::Equal
			}
		});
	}

	/// Sort the octree nearest to farthest, while culling all outside of
	/// view frustum.
	pub fn nearest(&mut self, sorted: &mut Vec<u32>, frustum: Frustum) {
		self.zsort(sorted, true, frustum)
	}

	/// Sort the octree farthest to nearest, while culling all outside of
	/// view frustum.
	pub fn farthest(&mut self, sorted: &mut Vec<u32>, frustum: Frustum) {
		self.zsort(sorted, false, frustum)
	}
	/// Print the octree
	pub fn print(&self) {
		self.print_node(self.root - 1, self.bbox, 0);
		println!("");
	}
	/// Print a node and its descendants
	fn print_node(&self, i: usize, bbox: BBox<i32>, t: u32) {
		let n = &self.nodes[i];
		print!("\n{:3} ", i + 1);
		for _ in 0 .. t {
			print!("  ");
		}
		if n.is_leaf() {
			print!("leaf:");
			for hnd in n.leaf_children().iter() {
				if *hnd < 1 { continue; }

				let p = self[*hnd].posi();
				print!(" {:?}_{:?}", *hnd, p);
			}
			print!("\t{:?}", bbox);
			let j = n.link();
			if j > 0 {
				print!("\tCCoincident LLink:");
				self.print_node(j - 1, bbox, t);
			}
		} else {
			print!("{:?}", n);
			print!("\t{:?}", bbox);
			for ch in 0 .. 8 {
				let bb = Node::child_bbox(ch, bbox);
				let k = n.child[ch] as usize;
				if k > 0 {
					self.print_node(k - 1, bb, t + 1);
				}
			}
		}
	}
	/// Get the number of points in the octree.
	pub fn len(&self) -> usize {
		self.n_points as usize
	}
	/// Abort program on error if the octree is corrupt!
	#[allow(unused)] pub fn check_corrupt(&self) {
		self.check_corrupt_node(self.root - 1, self.bbox, 0);
	}
	/// Print a node and its descendants
	fn check_corrupt_node(&self, i: usize, bbox: BBox<i32>, t: u32) {
		let n = &self.nodes[i];

		if n.is_leaf() {
			for hnd in n.leaf_children().iter() {
				if *hnd < 1 { continue; }

				let p = self[*hnd].posi();

				if !bbox.contains(p) {
					self.print();
					panic!("Corrupt Octree at HND: {} P: {:?} BBOX: {:?}", *hnd, p, bbox);
				}
			}
			let j = n.link();
			if j > 0 {
				self.check_corrupt_node(j - 1, bbox, t);
			}
		} else {
			for ch in 0 .. 8 {
				let bb = Node::child_bbox(ch, bbox);
				let k = n.child[ch] as usize;
				if k > 0 {
					self.check_corrupt_node(k - 1, bb, t + 1);
				}
			}
		}
	}
}

impl<T> ::std::ops::Index<u32> for Octree<T> where T: Pos {
	type Output = T;

	fn index<'a>(&'a self, index: u32) -> &'a T {
		&self.points[index as usize - 1]
	}
}

impl<T> ::std::ops::IndexMut<u32> for Octree<T> where T: Pos {
	fn index_mut<'a>(&'a mut self, index: u32) -> &'a mut T {
		&mut self.points[index as usize - 1]
	}
}


impl<T> fmt::Debug for Octree<T> where T: Pos {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(write!(f, "octree: bbox: {:?}", self.bbox));
		try!(write!(f, "\n\troot: {:?}", self.root));
		try!(write!(f, "\n\tn_points: {:?}", self.n_points));
		try!(write!(f, "\n\tnodes: {:?}", self.nodes.len()));
		try!(write!(f, "\n\tgarbage: {:?}", self.garbage.len()));
		Ok(())
	}
}

#[test]
fn test_octree() {
	let mut o = Octree::new();

	for x in 0 .. 100 {
		for y in 0 .. 100 {
			for z in 0 .. 100 {
				o.add(Vec3i::new(x, y, z));
				o.check_corrupt();
			}
		}
	}

	assert!(o.len() == 1000_000);
}

#[test]
fn test_coincident() {
	let mut o = Octree::new();

	for _ in 0 .. 10 {
		o.add(Vec3i::new(0, 0, 0));
		o.check_corrupt();
	}

	o.add(Vec3i::new(1,1,1));
	o.check_corrupt();

	assert!(o.len() == 11);
}

#[test]
fn test_add_remove() {
	let mut o = Octree::new();

	for x in 0 .. 10 {
		for y in 0 .. 10 {
			for z in 0 .. 10 {
				o.add(Vec3i::new(x, y, z));
				o.check_corrupt();
			}
		}
	}

	o.print();

	for i in 0 .. 10*10*10 {
		o.remove(i + 1);
		o.check_corrupt();
	}

	assert!(o.len() == 0)
}

#[test]
fn test_add_and_remove() {
	let mut o = Octree::new();

	for _ in 0 .. 10 {
		o.add(Vec3i::new(0, 0, 0));
		o.check_corrupt();
	}
	assert!(o.len() == 2 + 8);

	for i in 0 .. 2 {
		o.remove(i + 1);
		o.check_corrupt();
	}
	assert!(o.len() == 0 + 8);

	for _ in 0 .. 2 {
		o.add(Vec3i::new(1, 2, 4));
		o.check_corrupt();
	}
	assert!(o.len() == 2 + 8);

	println!("ZXCVBNM");
	o.print();
}
