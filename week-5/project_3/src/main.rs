use std::io;  



fn main() {
    println!("Menu                                                          Price");
    println!("Poundo Yam/Edikaiko Soup--------------------------------------N3200.00 (Meal 1)");
    println!("Fried Rice & Chicken------------------------------------------N3000.00 (Meal 2)");
    println!("Amala & Ewedu Soup--------------------------------------------N2500.00 (Meal 3)");
    println!("Eba & Egusi Soup----------------------------------------------N2000.00 (Meal 4)");
    println!("White Rice & Stew---------------------------------------------N2500.00 (Meal 6)");
    println!("Pick any of the meals i.e Meal1");



    let mut input1 = String::new();
    let mut input2 = String::new();
    
        println!("\nWhat Meal would you like to buy?");
        io::stdin().read_line(&mut input1).expect("Not a valid value");
        let meal:f32 = input1.trim().parse().expect("Not a valid input");

        print!("\nHow many Portions? ");
        io::stdin().read_line(&mut input2).expect("Not a valid input");
        let portion:f32 = input2.trim().parse().expect("Not a valid input");

    let Pound Yam and Edikaiko Soup = Meal1;
    let Fried Rice and Chicken = Meal2;
    let Amala and Ewedu Soup = Meal3;
    let Eba and Egusi Soup = Meal4;
    let White Rice and Stew = Meal5;
    





}
