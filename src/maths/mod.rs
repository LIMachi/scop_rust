pub mod vector;
pub mod matrix;
pub mod quaternion;
pub mod transform;

pub trait Unit {
    fn unit() -> Self;
}

impl Unit for f32 { fn unit() -> Self { 1. } }
impl Unit for f64 { fn unit() -> Self { 1. } }
impl Unit for i8 { fn unit() -> Self { 1 } }
impl Unit for i16 { fn unit() -> Self { 1 } }
impl Unit for i32 { fn unit() -> Self { 1 } }
impl Unit for i64 { fn unit() -> Self { 1 } }
impl Unit for i128 { fn unit() -> Self { 1 } }
impl Unit for u8 { fn unit() -> Self { 1 } }
impl Unit for u16 { fn unit() -> Self { 1 } }
impl Unit for u32 { fn unit() -> Self { 1 } }
impl Unit for u64 { fn unit() -> Self { 1 } }
impl Unit for u128 { fn unit() -> Self { 1 } }

pub trait Root2 {
    fn root2(&self) -> Self;
}

impl Root2 for f32 { fn root2(&self) -> Self { self.sqrt() } }
impl Root2 for f64 { fn root2(&self) -> Self { self.sqrt() } }
impl Root2 for i8 { fn root2(&self) -> Self { (*self as f32).sqrt() as Self } }
impl Root2 for i16 { fn root2(&self) -> Self { (*self as f32).sqrt() as Self } }
impl Root2 for i32 { fn root2(&self) -> Self { (*self as f32).sqrt() as Self } }
impl Root2 for i64 { fn root2(&self) -> Self { (*self as f32).sqrt() as Self } }
impl Root2 for i128 { fn root2(&self) -> Self { (*self as f32).sqrt() as Self } }
impl Root2 for u8 { fn root2(&self) -> Self { (*self as f32).sqrt() as Self } }
impl Root2 for u16 { fn root2(&self) -> Self { (*self as f32).sqrt() as Self } }
impl Root2 for u32 { fn root2(&self) -> Self { (*self as f32).sqrt() as Self } }
impl Root2 for u64 { fn root2(&self) -> Self { (*self as f64).sqrt() as Self } }
impl Root2 for u128 { fn root2(&self) -> Self { (*self as f64).sqrt() as Self } }
