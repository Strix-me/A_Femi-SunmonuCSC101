use std::io;

fn StudentCouncil_Votex()->f32{

    

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

     println!("\nAre you a  Current class rep.");
    io::stdin().read_line(&mut input1).expect("Not a valid Value");
    let a:f32 = input1.trim().parse().expect("Not a valid input");

       println!("\n What Level are you in");
    io::stdin().read_line(&mut input2).expect("Not a valid Value");
    let b:f32 = input2.trim().parse().expect("Not a valid input");


         if a == 1.0 
          {
                println!("\n What Level are you in");
                io::stdin().read_line(&mut input2).expect("Not a valid Value");
                let b:f32 = input2.trim().parse().expect("Not a valid input");
      
            println!("\nInput Your CGPA");
            io::stdin().read_line(&mut input3).expect("Not a valid Value");
            let c:f32 = input2.trim().parse().expect("Not a valid input");
        }


fn facpub ()->f32{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Name {?}");
    io::stdin().read_line(&mut input1).expect("Not a valid Value");
    let n:f32 = input1.trim().parse().expect("Not a valid input");


}




fn main() {
     let mut input1 = String::new();

    println!("1:- Student Council Vote");
    println!("2:- The Faculty publication Incentive System");

     println!("What Do you want to do, pick 1 or 2");
    io::stdin().read_line(&mut input1).expect("Not a valid Value");
    let function:f32 = input1.trim().parse().expect("Not a valid input");

    if function == 1.0
 {
    println!("Student Council Vote",);
}
else if function == 2.0
{
    println!("The Faculty publication Incentive System",);
}

