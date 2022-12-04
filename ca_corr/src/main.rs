use std::io;

fn studentcouncilvotex(){

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();

     println!("\nAre you a class rep");
     println!("\n If yes type 1, if no type 2.");
    io::stdin().read_line(&mut input1).expect("Not a valid Value");
    let a:f32 = input1.trim().parse().expect("Not a valid input");

    if a == 1.0{
        println!("\nWhat Level Are you in");
        io::stdin().read_line(&mut input2).expect("Not a valid value");
        let b:f32 = input2.trim().parse().expect("Not a valid input");


         if b > 100.0{
             println!("What is your CGPA");
             io::stdin().read_line(&mut input3).expect("Not a valid value");
             let c:f32= input3.trim().parse().expect("Not a valid input");

             if c >= 4.0{
                println!("You are Eligible To vote!!");
                println!("What is your name");
                io::stdin().read_line(&mut input4).expect("Not a valid value");
               


                println!("What is your Email");
                io::stdin().read_line(&mut input5).expect("Not a valid value");
                

                println!("What Department are you in?");
                io::stdin().read_line(&mut input6).expect("Not a valid value");
                


                println!("What is your State of Origin");
                io::stdin().read_line(&mut input7).expect("Not a valid value");
               
                println!("Name:{}",input4);
                println!("Email:{}",input5);
                println!("Department:{}",input6);
                println!("State of Origin:{}",input7 );


             }else {
                println!("Inprove Your CGPA and retry");
             }

         }else{
            println!("You cannot vote you are a fresher ");
         }


 }else{
    println!("You cannot vote you are a not a class rep");
 }




}

fn facpub(){
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("What is your name");
    io::stdin().read_line(&mut input1).expect("Not a valid Value");
    
    

    println!("Number of papers Published?");
    io::stdin().read_line(&mut input2).expect("Not a valid Value");
    let paper:f32 = input2.trim().parse().expect("Not a valid input");
    let _input2 = paper;
    if paper >=3.0 && paper <=5.0
    {
        println!("{},Your Incentive is 500,000",input1);
    }
    else if paper > 5.0 && paper < 10.0 {
        println!("{} Your Incentive is 800,000",input1 );
        
    }
    else if paper >= 10.0
     {
      println!("{}Your Incentive is 1,000,000",input1 );  
    }
    else if paper < 3.0
     {
        println!("{}Your Incentive is 100,000",input1 );
    }





}

fn main() {
     let mut input1 = String::new();

    println!("\n1:- Student Council Vote");
    println!("\n2:- The Faculty publication Incentive System");

     println!("What Do you want to do, pick 1 or 2");
    io::stdin().read_line(&mut input1).expect("Not a valid Value");
    let function:f32 = input1.trim().parse().expect("Not a valid input");

    if function == 1.0
 {
   studentcouncilvotex();
}
else if function == 2.0
{
    facpub();
}

}