# modInverse

Write a code that calculates the Modular multiplicative inverse of a number using the Euclidean Algorithm?

A modular multiplicative inverse is known to be https://en.wikipedia.org/wiki/Modular_multiplicative_inverse

example: 3^-1 = 3 x 1/3 which results to 1

Rust has a Small library for finding the modular multiplicative inverses. Also has an implementation of the extended Euclidean algorithm built in.

for mod inverse

fn mod_inv(a: isize, module: isize) -> isize {
  let mut mn = (module, a);
  let mut xy = (0, 1);
 
  while mn.1 != 0 {
    xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
    mn = (mn.1, mn.0 % mn.1);
  }
 
  while xy.0 < 0 {
    xy.0 += module;
  }
  xy.0
}
 
fn main() {
  println!("{}", mod_inv(42, 2017))
}
