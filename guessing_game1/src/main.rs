fn main() {
let mut x = 5;
println!("The value of x is: {x}");
 x = 6;
println!("The value of x is: {x}");
// --snip--
let x1 = 5;
let x1 = x1 + 1;
 {
let x1 = x1 * 2;
println!("The value of x in the inner scope is: {x1}");
 }
println!("The value of x is: {x1}");
// --snip--
// addition
let sum = 5 + 10;
println!("The value of sum is: {sum}");
// subtraction
let difference = 95.5 - 4.3;
println!("The value of difference is: {difference}");
// multiplication
let product = 4 * 30;
println!("The value of product is: {product}");
// division
let quotient = 56.7 / 32.2;
println!("The value of quotient is: {quotient}");
let truncated = -5 / 3; // Results in -1
println!("The value of truncated is: {truncated}");
// remainder
let remainder = 43 % 5;
println!("The value of remainder is: {remainder}");
// --snip--
let t = true;
println!("The value of t is: {t}");
let f: bool = false; // with explicit type annotation
println!("The value of f is: {f}");
// --snip--
}
