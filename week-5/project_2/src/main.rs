use std::io;


fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    

    println!("Enter Age");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:i32 = input1.trim().parse().expect("Not a valid number");

    println!("How many years of experience?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let _experience:i32 = input2.trim().parse().expect("Not a valid number");


    let a = 1_560_000;
    let b = 1_480_000;
    let c = 1_300_000;
    let d = 100_000;



    if _experience >10 && age >=40 // e
    {

     println!(" Your Incentive is {} ",a);
    }

    else if _experience > 10 && age >= 30 && age <= 40
    {
        println!("Your Incentive is {} ",b);
    }

    else if _experience > 10 && age <= 28 
    {
        println!(" Your Incentive is {}",c );
    }

    else if _experience <10 
    {
        println!("Your Incentive is {}",d );
    }
}








    

