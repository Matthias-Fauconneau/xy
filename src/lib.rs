#![cfg_attr(feature="iter", feature(iterator_fold_self))]
core::vector!(2 xy T T, x y, X Y);

impl xy<u32> { pub const fn signed(self) -> xy<i32> { xy{x: self.x as i32, y: self.y as i32} } }
impl xy<i32> { pub const fn unsigned(self) -> xy<u32> { xy{x: self.x as u32, y: self.y as u32} } }
impl From<xy<i32>> for xy<u32> { fn from(i: xy<i32>) -> Self { i.unsigned() } }
impl From<xy<u32>> for xy<i32> { fn from(u: xy<u32>) -> Self { u.signed() } }
impl From<xy<u32>> for xy<f32> { fn from(f: xy<u32>) -> Self { xy{x: f.x as f32, y: f.y as f32} } }

#[allow(non_camel_case_types)] pub type uint2 = xy<u32>;
#[allow(non_camel_case_types)] pub type int2 = xy<i32>;
#[allow(non_camel_case_types)] pub type size = xy<u32>;
#[allow(non_camel_case_types)] pub type vec2 = xy<f32>;

pub fn div_ceil(n: uint2, d: u32) -> uint2 { xy{x:core::num::div_ceil(n.x,d), y:core::num::div_ceil(n.y,d)} }
pub fn lerp(t: f32, a: vec2, b: vec2) -> xy<f32> { (1.-t)*a + t*b }
pub fn dot(a: vec2, b: vec2) -> f32 { a.x*b.x + a.y*b.y }
pub fn cross(a: vec2, b: vec2) -> f32 { a.x*b.y - a.y*b.x }
pub fn sq(x:vec2) -> f32 { dot(x, x) }
pub fn norm(v:vec2) -> f32 { sq(v).sqrt() }
pub fn atan(v:vec2) -> f32 { v.y.atan2(v.x) }

#[derive(Default)] pub struct Rect { pub min: int2, pub max: int2 }
impl Rect {
	pub fn translate(&mut self, offset: int2) { self.min += offset; self.max += offset; }
	pub fn clip(&self, b: Rect) -> Self { Rect{min: core::vector::component_wise_max(self.min,b.min), max: core::vector::component_wise_min(self.max,b.max)} }
	pub fn size(&self) -> size { (self.max-self.min).unsigned() }
}
impl From<size> for Rect { fn from(size: size) -> Self { Self{ min: core::num::Zero::zero(), max: size.signed()} } }

impl std::ops::Mul<uint2> for core::num::Ratio { type Output=uint2; fn mul(self, b: uint2) -> Self::Output { xy{x:self*b.x, y:self*b.y} } }
pub fn ceil(scale: &core::num::Ratio, v: uint2) -> uint2 { xy{x:scale.ceil(v.x), y:scale.ceil(v.y)} }
fn ifloor(scale: core::num::Ratio, b: int2) -> int2 { xy{x:scale.ifloor(b.x), y:scale.ifloor(b.y)} }
fn iceil(scale: core::num::Ratio, b: int2) -> int2 { xy{x:scale.iceil(b.x), y:scale.iceil(b.y)} }
impl std::ops::Mul<Rect> for core::num::Ratio { type Output=Rect; fn mul(self, b: Rect) -> Self::Output { Rect{min:ifloor(self, b.min), max:iceil(self, b.max)} } }
impl std::ops::Div<core::num::Ratio> for uint2 { type Output=uint2; fn div(self, r: core::num::Ratio) -> Self::Output { xy{x:self.x/r, y:self.y/r} } }
