#![allow(dead_code)]
#![allow(unused_variables)]

use core::fmt::Debug;
use core::ops::{Index, IndexMut};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::convert::{Into};

// https://www.worthe-it.co.za/programming/2017/01/15/aliasing-traits-in-rust.html
pub trait VectorElementTrait =
  Copy + Debug + Clone + PartialEq +
  Add<Output=Self> + AddAssign +
  Sub<Output=Self> + SubAssign +
  Mul<Output=Self> + MulAssign +
  Div<Output=Self> + DivAssign +
  Neg<Output=Self> +
  Into<Self>
;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T: VectorElementTrait> {
  pub x: T,
  pub y: T,
  pub z: T,
}

macro_rules! impl_operator {
  ($op:ident, $fn:ident) => {
    impl<T: VectorElementTrait> $op for Vec3<T> {
      type Output = Vec3<T>;
      fn $fn(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
          x: self.x.$fn(rhs.x),
          y: self.y.$fn(rhs.y),
          z: self.z.$fn(rhs.z),
        }
      }
    }

    impl<T: VectorElementTrait> $op for &Vec3<T> {
      type Output = Vec3<T>;
      fn $fn(self, rhs: &Vec3<T>) -> Self::Output {
        Vec3 {
          x: self.x.$fn(rhs.x),
          y: self.y.$fn(rhs.y),
          z: self.z.$fn(rhs.z),
        }
      }
    }

    impl<T: VectorElementTrait> $op<&Vec3<T>> for Vec3<T> {
      type Output = Vec3<T>;
      fn $fn(self, rhs: &Vec3<T>) -> Self::Output {
        Vec3 {
          x: self.x.$fn(rhs.x),
          y: self.y.$fn(rhs.y),
          z: self.z.$fn(rhs.z),
        }
      }
    }

    impl<T: VectorElementTrait> $op<T> for Vec3<T> {
      type Output = Vec3<T>;
      fn $fn(self, rhs: T) -> Self::Output {
        Vec3 {
          x: self.x.$fn(rhs),
          y: self.y.$fn(rhs),
          z: self.z.$fn(rhs),
        }
      }
    }
  }
}

macro_rules! impl_assign_operator {
  ($op:ident, $fn:ident) => {
    impl<T: VectorElementTrait> $op for Vec3<T> {
      fn $fn(&mut self, rhs: Self) {
        self.x.$fn(rhs.x);
        self.y.$fn(rhs.y);
        self.z.$fn(rhs.z);
      }
    }

    impl<T: VectorElementTrait> $op<T> for Vec3<T> {
      fn $fn(&mut self, rhs: T) {
        self.x.$fn(rhs);
        self.y.$fn(rhs);
        self.z.$fn(rhs);
      }
    }
  }
}

impl_operator!(Add, add);
impl_operator!(Sub, sub);
impl_operator!(Mul, mul);
impl_operator!(Div, div);
impl_assign_operator!(AddAssign, add_assign);
impl_assign_operator!(SubAssign, sub_assign);
impl_assign_operator!(MulAssign, mul_assign);
impl_assign_operator!(DivAssign, div_assign);

impl From<Vec3<i8>> for Vec3<i32> {
  fn from(input: Vec3<i8>) -> Vec3<i32> {
    Vec3 {
      x: input.x.into(),
      y: input.y.into(),
      z: input.z.into(),
    }
  }
}

#[test]
fn test_add() {
  let u = Vec3 { x: 1, y: 2, z: 3 };
  let v = Vec3 { x: 2, y: 3, z: 4 };

  assert_eq!(u + &v, Vec3 { x: 3, y: 5, z: 7 });
  assert_eq!(u + v, Vec3 { x: 3, y: 5, z: 7 });
  assert_eq!(Vec3 { x: 1, y: 2, z: 3 } + 1, Vec3 { x: 2, y: 3, z: 4 });
}

#[test]
fn test_add_assign() {
  let mut u = Vec3 { x: 1, y: 2, z: 3 };

  u += Vec3 { x: 2, y: 3, z: 4 };
  assert_eq!(u, Vec3 { x: 3, y: 5, z: 7 });

  u += 2;
  assert_eq!(u, Vec3 { x: 5, y: 7, z: 9 });
}

#[test]
fn test_sub() {
  assert_eq!(Vec3 { x: 2, y: 3, z: 4 } - Vec3 { x: 1, y: 2, z: 3 }, Vec3 { x: 1, y: 1, z: 1 });
}

#[test]
fn test_sub_assign() {
  let mut u = Vec3 { x: 2, y: 3, z: 4 };
  u -= Vec3 { x: 1, y: 2, z: 3 };
  assert_eq!(u, Vec3 { x: 1, y: 1, z: 1 });
}

impl<T: VectorElementTrait> Neg for Vec3<T> {
  type Output = Vec3<T>;
  fn neg(self) -> Self::Output {
    Vec3 {
      x: self.x.neg(),
      y: self.y.neg(),
      z: self.z.neg(),
    }
  }
}

#[test]
fn test_neg() {
  assert_eq!(-Vec3 { x: 2, y: 3, z: 4 }, Vec3 { x: -2, y: -3, z: -4 });
}

impl<T: VectorElementTrait> Index<usize> for Vec3<T> {
  type Output = T;

  fn index(&self, i: usize) -> &Self::Output {
    match i {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => panic!("Index out of range"),
    }
  }
}

#[test]
fn test_index() {
  assert_eq!(Vec3 { x: 1, y: 2, z: 3 }[1], 2);
}

#[should_panic]
#[test]
fn test_index_out_of_bound() {
  Vec3 { x: 1, y: 2, z: 3 }[3];
}

impl<T: VectorElementTrait> IndexMut<usize> for Vec3<T> {
  // No Output here because IndexMut derive from Index
  fn index_mut(&mut self, i: usize) -> &mut Self::Output {
    match i {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      _ => panic!("Index out of range"),
    }    
  }
}

#[test]
fn test_index_mut() {
  let mut u = Vec3 { x: 1, y: 2, z: 3 };
  u[2] = 42;
  assert_eq!(u[2], 42);
}

#[test]
#[should_panic]
fn test_index_mut_out_of_bound() {
  let mut u = Vec3 { x: 1, y: 2, z: 3 };
  u[4] = 42;
}

// Non working version with integral operator parameter: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=15681aedd56f6a9e0e55e0dbe836ad14