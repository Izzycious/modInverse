// Function declaration takes in two parameters of type isize (The pointer-sized signed integer type.) 

//The size of this primitive is how many bytes it takes to reference any location in memory
// a represents an integer whose multiplicative inverse is to be computed.
// modulus represents an integer whose value is greater than 1
fn egcd(mut x: isize, mut y:isize) -> (isize,isize,isize) {

  // Declaring a mutable variable 
  let (mut a0,mut a1,mut b0,mut b1) = (1,0,0,1);

  //loop function allows the execution of code as long as the codition is true usin

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

    // Rust programs start with fn main()
    // You put the code inside a block. It starts with { and ends with }

fn main() {
  let (gcd, a, b) = egcd(180,150);

  // Print function in getting the extended euclidean algorithm output of 
  println!("GCD: {}  {} and {}",
  gcd, a, b);
  
  //30   1 and -1
}