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

pub fn lerp(t: f32, a: vec2, b: vec2) -> xy<f32> { (1.-t)*a + t*b }
pub fn dot(a: vec2, b: vec2) -> f32 { a.x*b.x + a.y*b.y }
pub fn cross(a: vec2, b: vec2) -> f32 { a.x*b.y - a.y*b.x }
pub fn sq(x:vec2) -> f32 { dot(x, x) }
pub fn norm(v:vec2) -> f32 { core::num::sqrt(sq(v)) }
pub fn atan(v:vec2) -> f32 { core::num::atan(v.y,v.x) }

impl std::ops::Mul<uint2> for core::num::Ratio { type Output=uint2; #[track_caller] fn mul(self, b: uint2) -> Self::Output { xy{x:self*b.x, y:self*b.y} } }
impl std::ops::Div<core::num::Ratio> for uint2 { type Output=uint2; #[track_caller] fn div(self, r: core::num::Ratio) -> Self::Output { xy{x:self.x/r, y:self.y/r} } }
