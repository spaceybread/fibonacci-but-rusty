fn main() {
  let mut a = 1;
  let mut b = 1;

  println!("{a}");
  println!("{b}");

  let mut n = 0;

  while n < 25 {
      let mut c = a + b;
      println!("{c}");

      a = b;
      b = c;

      n = n + 1;
  }

}
