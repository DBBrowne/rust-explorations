use std::fmt;
// fn sqr(x: f64) -> f64{
//   // no semicolon triggers implicit return
//   x * x
// }

// fn floats_and_macros() {
//   let answer = 4_200;
//   let res = sqr(answer as f64);
//   println!("sum {}!", res);
//   assert_eq!(res, 17_640_000 as f64);
// }

// fn by_ref(x & i32)-> i32{

// }
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
impl fmt::Display for Matrix{
  fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
    write!(
      f, "( {} {} )\n( {} {} )",
      self.0, self.1,self.2, self.3
    )
  }
}
fn printme(){
      let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);
}




pub fn sub() {
  // println!("SUB: 123 {}", 456.0);
  printme();
}