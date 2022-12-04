use std::io;

fn area_of_trap()->f32{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\nEnter Height");
    io::stdin().read_line(&mut input1).expect("Not a valid Value");
    let h:f32 = input1.trim().parse().expect("Not a valid input");

    println!("\n Enter Base 1");
    io::stdin().read_line(&mut input2).expect("Not a valid Value");
    let base1:f32 = input2.trim().parse().expect("Not a valid input");

    println!("\nEnter Base 2");
    io::stdin().read_line(&mut input3).expect("Not a valid Value");
    let base2:f32 = input3.trim().parse().expect("Not a valid input");


    let b1 = base1;
    let b2 = base2;
    let x = h / 2.0;
    let at = x  * (b1 + b2);
    return at;
}

fn area_of_rhombus()->f32{
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("\nEnter Diagonal 1");
    io::stdin().read_line(&mut input1).expect("Not a valid Value");
    let diagonal1:f32 = input1.trim().parse().expect("Not a valid input");

    println!("\n Enter Diagonal 2");
    io::stdin().read_line(&mut input2).expect("Not a valid value");
    let diagonal2:f32 = input2.trim().parse().expect("Not a valid input");

    let d1 = diagonal1;
    let d2 = diagonal2;
    let x = 1.0 / 2.0;
    let a = x * (d1 * d2);
    return a;

}

fn area_of_parallelogram()->f32{
    let mut input1 = String::new();
    let mut input2 = String::new();


    println!("\n Base");
    io::stdin().read_line(&mut input1).expect("Not a valid Value");
    let base:f32 = input1.trim().parse().expect("Not a valid input");

    println!("\n Altitude");
    io::stdin().read_line(&mut input2).expect("Not a valid Value");
    let altitude:f32 = input2.trim().parse().expect("Not a valid input");

    let b = base;
    let t = altitude;
    let a = b * t;
    return a;
}

    fn area_of_cube()->f32{
    
    let mut input1 = String::new();

    println!("\nEnter Legnth of side");
    io::stdin().read_line(&mut input1).expect("Not a valid Value");
    let length:f32 = input1.trim().parse().expect("Not a valid input");

    let l = length;
    let a = 6.0 * (l * l);
    return a;
    }

    fn volume_of_cylinder()->f32{

    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("\nEnter Radius");
    io::stdin().read_line(&mut input1).expect("Not a valid Value");
    let radius:f32 = input1.trim().parse().expect("Not a valid input");

    println!("\nEnter Height");
    io::stdin().read_line(&mut input2).expect("Not a valid value");
    let height:f32 = input2.trim().parse().expect("Not a valid input");

    let r = radius;
    let h = height;
    let p = 22.0 / 7.00;
    let a = p * (r * r) * h;
    return a;
    }





fn main() {


println!("Formulas Available:");
println!("1:- Area of Trapexium");
println!("2:- Area of Rhombus");
println!("3:- Area of Parallelogram formula");
println!("4:- Area of Cube ");
println!("5:- Volume of Cylinder formula");



    let mut input1 = String::new();


    
println!("\nPlease Enter what formula to apply from 1-5");
io::stdin().read_line(&mut input1).expect("Not a valid Value");
let formula:f32 = input1.trim().parse().expect("Not a valid input");
if formula == 1.0
 {
    println!("THe Area of your trapezium is {:?}",area_of_trap());
}
else if formula == 2.0
{
    println!("The Area of your Rhombus is {:?}",area_of_rhombus());
}
else if formula == 3.0
{
    println!("The Area of your Parallelogram is {:?}",area_of_parallelogram());
}
else if formula == 4.0
{
    println!("The area of your cube is {:?}",area_of_cube());
}
else if formula == 5.0
{
    println!("The Volume of your Cylinder is {:?}",Svolume_of_cylinder());
}
}
