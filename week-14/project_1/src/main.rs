use std::io::Read;
use std::io;




fn admin() {
     let mut file = std::fs::File::open("globacom.sql").unwrap();
     let mut contents = String::new();
     file.read_to_string(&mut contents).unwrap();
     print!("{}", contents);

}
fn proj_man(){
    let mut file = std::fs::File::open("project_table.sql").unwrap();
     let mut contents = String::new();
     file.read_to_string(&mut contents).unwrap();
     print!("{}", contents);

}
fn employee() {
    let mut file = std::fs::File::open("staff_table.sql").unwrap();
     let mut contents = String::new();
     file.read_to_string(&mut contents).unwrap();
     print!("{}", contents);
}
fn customer(){
    let mut file = std::fs::File::open("customer_table.sql").unwrap();
     let mut contents = String::new();
     file.read_to_string(&mut contents).unwrap();
     print!("{}", contents);
}
fn vendor(){
    let mut file = std::fs::File::open("dataplan_tabel.sql").unwrap();
     let mut contents = String::new();
     file.read_to_string(&mut contents).unwrap();
     print!("{}", contents);
}
fn main(){

    println!("Welcome To Globacom");
    println!("Who are you? Pick from 1-5");

    let user = vec!["1. Administrator", "2. Project Manager", "3. Employee", "4. Customer", "5. Vendor",];
    for i in 0..user.len()
   {
    // iterating through i on the vector
    print!("\n{} ",user[i]);
   }

     let mut input1 = String::new();

    io::stdin().read_line(&mut input1).expect("Not a String");
    let a:i32 = input1.trim().parse().expect("not a number");

    if a == 1 {
        admin();
    }
    else if a == 2{
        proj_man();
    }
    else if a == 3{
        employee();
    }
    else if a == 4{
        customer();
    }
    else if a == 5{
        vendor();
    }

}