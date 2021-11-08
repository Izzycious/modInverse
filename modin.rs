// Function declaration which takes in two parameters of type isize 
// a representing an int whose multiplicative inverse to compute
// b representing an int value is > 1
fn mod_inverse(a: isize, b: isize) -> isize {
  // Declaring a mutable variable x set equal to a tuple  that holds "b" and "a".
  //let mut y = ... is avariable that counts algorithm step
  let mut x = (b, a);
  let mut y = (0, 1);

  // for each iteration, a turple is created for x and y
 
  while x.1 != 0 {
    y = (y.1, y.0 - (x.0/x.1) * y.1);
    x = (x.1, x.0 % x.1);
  }

  //implementing thw euclidean algorithm
 
  while y.0 < 0 {
    y.0 += b;
  }
  y.0
}
// function declared
 
fn main() {

  //printing the value of the output
  println!("{}", mod_inverse(12, 78))

  // output 72
}