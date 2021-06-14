#![allow(non_snake_case)]
pub fn run() {
      // normal int, defalut = i32
      let x = 1;
      // normal float, defalut = f64

      let y = 2.4;

      // set type i64
      let z: i64 = 892478932720;

      // get max len, i32 i64
      println!("{}", std::i32::MAX);
      println!("{}", std::i64::MAX);

      // bool
      let isTrue:bool = true;

      // get bool
      let isGreater: bool = 10 < 5;

      println!("{:?}", (x, y, z, isTrue, isGreater))
}