#![feature(trait_alias)]

// https://doc.rust-lang.org/std/ops/trait.Add.html
// https://stackoverflow.com/questions/25877285/how-to-disable-unused-code-warnings-in-rust
// https://doc.rust-lang.org/book/ch10-01-syntax.html
// https://users.rust-lang.org/t/trait-bound-on-type-output/17559
// https://stackoverflow.com/questions/26983355/is-there-a-way-to-combine-multiple-traits-in-order-to-define-a-new-trait
// https://doc.rust-lang.org/std/ops/trait.Index.html
// https://github.com/rust-lang/rust/issues/44491

mod vec3;
use vec3::Vec3;

use std::convert::Into;

fn main() {
  let u: Vec3<i32> = Vec3 { x: 1, y: 2, z: 3 };
  let v: Vec3<i8> = Vec3 { x: 2, y: 3, z: 4 };
  let w = u + v.into();

  // println!("{:?}", u + Into::<Vec3<i32>>::into(v));
  println!("w {:?}", w);
}
