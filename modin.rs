fn egcd(mut x: isize, mut y:isize) -> (isize,isize,isize) {
  let (mut a0,mut a1,mut b0,mut b1) = (1,0,0,1);

  while y != 0 {
      // dbg!(x,y);
      let (q,r)  = (x / y, x % y);
      let (c, d) = ( a0 - q * a1, b0 - q * b1  ); 

      x = y;
      y = r;
      a0 = a1;
      a1 = c;
      b0 = b1; 
      b1 = d;

  }
  (x, a0, b0)
}

fn main() {
  let (gcd, a, b) = egcd(180,150);
  println!("GCD: {}  {} and {}",
  gcd, a, b);
  
  //
}