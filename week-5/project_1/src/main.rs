use std::io;

fn main() {
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

   println!("\nEnter Value of a", );
io::stdin().read_line(&mut input1).expect("Not a valid Value");
let a :f32 = input1.trim().parse().expect("Not a valid input");

 println!("\nEnter Value of b", );
io::stdin().read_line(&mut input2).expect("Not a valid value");
let b:f32 = input2.trim().parse().expect("Not a valid input");


 println!("\nEnter Value of c", );
io::stdin().read_line(&mut input3).expect("Not a valid value");
let c:f32 = input3.trim().parse().expect("Not a valid input");


let x = 4.0 * a * c;
let y = b  *  b;
let z = 2.0 * a;
let h = y - x;



let discriminant:f32 = y - x;

if discriminant > 0.0{

   println!("The discriminant {}, is greater than zero so there are 2 real roots",discriminant);
   let root1:f32 = (-b + h.sqrt())/z;
   let root2:f32 = (-b - h.sqrt())/z;
   print!("The roots of the equation are  x: {} , {}",root1,root2 );

} else if discriminant == 0.0{
   
   println!("The discriminant is equal to zero {}, so the discriminants are 1 real number ",discriminant);
   let root3:f32 = (-b + h.sqrt())/z;
   let root4:f32 = (-b - h.sqrt())/z;
   print!("The root of the equation are x: {} and {}",root3,root4 );

} else if discriminant < 0.0{
   
   println!("The discriminant is less than zero {}, so the discriminants are 2 imaginary numbers",discriminant );


}








 








}
