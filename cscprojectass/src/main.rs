use std::io;
use std::io::Write;


 fn code_pick() {
   
    println!("Enter Your Code");
    println!("Codes Available:\n Code: 7\n Code: 8\n Code: 9\n");

    let mut input1 = String::new();

    io::stdin().read_line(&mut input1).expect("Not a String");
    let a:i32 = input1.trim().parse().expect("not a number");

    if a == 7
    {
        println!("Persons Assigned Code 7:\n Aigbona Julietn\n Apevwe Iloka\n ");
        code7_exe();
    }
    else if a == 8{
         println!("Persons Assigned to Code 8:\n Adamu Sagamu\n Gbenga Daniels\n ");
        code8_exe();
                    }
    else if a == 9{
        println!("Persons Assigned to Code 9:\n Ehis Ero\n Maria Akinsola\n ");
        code9_exe();
        }
    else{
        println!("Lol what are you doing? I said pick between 7,8,9. try again :(");
        code_pick();
    }
    }




 fn code7_exe(){

    println!("Who are you?\n (1)Aigbona Juliet\n (2)Apevwe Iloka\n ");
    println!("Enter 1 or 2");

    let mut input1 = String::new();

    io::stdin().read_line(&mut input1).expect("Not a String");
    let a:i32 = input1.trim().parse().expect("not a number");

    if a == 1 {
        let name = "Aigbona Juliet\n";
        let dept = "Consulting\n";
        let qualification ="B.Sc.\n";
        let code = "Code 7\n";
        let services = vec!["Analytics Consulting Services","Customer Experience", "Cyber Security Strategy, risk, compliance and resilience",
         "Digital Transformation","Risk Consulting Services","Supply chain and operations","Technology Transformation"];

         let mut file = std::fs::File::create("Aigbona_Juliet.txt").expect("create failed");
        file.write_all("Name:".as_bytes()).expect("Failed");
        file.write_all(name.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Department: ".as_bytes()).expect("Failed");
        file.write_all(dept.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Qualifications: ".as_bytes()).expect("Failed");
        file.write_all(qualification.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Code: ".as_bytes()).expect("Failed");
        file.write_all(code.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Services: ".as_bytes()).expect("Failed");
        for i in services{
            file.write_all("\n".as_bytes()).expect("Failed");
            file.write_all(i.as_bytes()).expect("Failed");
        }


    println!("\n Data written to file.");
 }
    else if a == 2{
        let name = "Apevwe Iloka\n";
        let dept = "Assurance\n";
        let qualification ="HND.\n";
        let code = "Code 7\n";
        let services = vec!["Audit Services","Climate Change and Sustainability Services", "Financial Accounting Advisory Services",
         "Forensic and integrity service","Private Client Audit Experience","Accounting Link","Assurance"];

         let mut file = std::fs::File::create("Apevwe_Iloka.txt").expect("create failed");
        file.write_all("Name:".as_bytes()).expect("Failed");
        file.write_all(name.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Department: ".as_bytes()).expect("Failed");
        file.write_all(dept.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Qualifications: ".as_bytes()).expect("Failed");
        file.write_all(qualification.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Code: ".as_bytes()).expect("Failed");
        file.write_all(code.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Services: ".as_bytes()).expect("Failed");
        for i in services{
            file.write_all("\n".as_bytes()).expect("Failed");
            file.write_all(i.as_bytes()).expect("Failed");
        }
          println!("\n Data written to file.");

    }
    else{
         println!("Lol what are you doing? I said pick between 1 and 2. try again :(");
        code7_exe();
    }


 }

 fn code8_exe() {
      println!("Who are you?\n (1)Adamu Sagamu\n (2) Gbenga Daniels\n ");
    println!("Enter 1 or 2");

    let mut input1 = String::new();

    io::stdin().read_line(&mut input1).expect("Not a String");
    let a:i32 = input1.trim().parse().expect("not a number");

        if a == 1 {
        let name = "Adamu Sagamu\n";
        let dept = "Tax\n";
        let qualification ="B.Sc.\n";
        let code = "Code 8\n";
        let services = vec!["Tax Planning","Tax Funtion operations","Tax Policy and controversy",
         "Global Trade","Tax Accounting","Tax Comliance","Transaction Tax"];

         let mut file = std::fs::File::create("Adamu_Sagamu.txt").expect("create failed");
        file.write_all("Name:".as_bytes()).expect("Failed");
        file.write_all(name.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Department: ".as_bytes()).expect("Failed");
        file.write_all(dept.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Qualifications: ".as_bytes()).expect("Failed");
        file.write_all(qualification.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Code: ".as_bytes()).expect("Failed");
        file.write_all(code.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Services: ".as_bytes()).expect("Failed");
        for i in services{
            file.write_all("\n".as_bytes()).expect("Failed");
            file.write_all(i.as_bytes()).expect("Failed");
        }


    println!("\n Data written to file.");
 }
    else if a == 2 {
        let name = " Gbenga Daniels\n";
        let dept = "People and Workforce\n";
        let qualification ="HND\n";
        let code = "Code 8\n";
        let services = vec!["Change and management Experience","HR Transformation",
         "Intergrated Workforce Mobility","Learning and Development Consulting","Recognition and reward Advisory","Workforce analytics","People and workforce"];

         let mut file = std::fs::File::create("Gbenga_Daniels.txt").expect("create failed");
        file.write_all("Name:".as_bytes()).expect("Failed");
        file.write_all(name.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Department: ".as_bytes()).expect("Failed");
        file.write_all(dept.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Qualifications: ".as_bytes()).expect("Failed");
        file.write_all(qualification.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Code: ".as_bytes()).expect("Failed");
        file.write_all(code.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Services: ".as_bytes()).expect("Failed");
        for i in services{
            file.write_all("\n".as_bytes()).expect("Failed");
            file.write_all(i.as_bytes()).expect("Failed");

    }
    println!("\n Data written to file.");
 }
  else{
         println!("Lol what are you doing? I said pick between 1 and 2. try again :(");
        code8_exe();
    }
}


fn code9_exe(){
     println!("Who are you?\n (1)Ehis Ero\n (2)Maria Akinsola\n ");
    println!("Enter 1 or 2");

    let mut input1 = String::new();

    io::stdin().read_line(&mut input1).expect("Not a String");
    let a:i32 = input1.trim().parse().expect("not a number");

        if a == 1 {
        let name = "Ehis Ero\n";
        let dept = "Strategy\n";
        let qualification ="M.Sc.\n";
        let code = "Code 9\n";
        let services = vec!["Strategy Consulting","Coporate and growth Strategy","Transaction Strategy and Execution",
         "Restructuring and turnaround Strategy","Industry Strategy","Digital business building","Commercial strategy"];

         let mut file = std::fs::File::create("Ehis_Ero.txt").expect("create failed");
        file.write_all("Name:".as_bytes()).expect("Failed");
        file.write_all(name.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Department: ".as_bytes()).expect("Failed");
        file.write_all(dept.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Qualifications: ".as_bytes()).expect("Failed");
        file.write_all(qualification.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Code: ".as_bytes()).expect("Failed");
        file.write_all(code.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Services: ".as_bytes()).expect("Failed");
        for i in services{
            file.write_all("\n".as_bytes()).expect("Failed");
            file.write_all(i.as_bytes()).expect("Failed");
        }


    println!("\n Data written to file.");
 }
    else if a == 2 {
        let name = "Maria Akinsola\n";
        let dept = "Transactions and Corporate finance\n";
        let qualification ="M.Sc.\n";
        let code = "Code 9\n";
        let services = vec!["Corporate Finance","Divestments and carve-outs",
         "Sustainability and ESG Services","M&A advisory","M&A Integration","M&A technology and tools","M&A advanced analytics"];

         let mut file = std::fs::File::create("Maria_Akinsola.txt").expect("create failed");
        file.write_all("Name:".as_bytes()).expect("Failed");
        file.write_all(name.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Department: ".as_bytes()).expect("Failed");
        file.write_all(dept.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Qualifications: ".as_bytes()).expect("Failed");
        file.write_all(qualification.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Code: ".as_bytes()).expect("Failed");
        file.write_all(code.as_bytes()).expect("Failed");
        file.write_all("\n".as_bytes()).expect("Failed");

        file.write_all("Services: ".as_bytes()).expect("Failed");
        for i in services{
            file.write_all("\n".as_bytes()).expect("Failed");
            file.write_all(i.as_bytes()).expect("Failed");

    }


 println!("\n Data written to file.");
}
 else{
         println!("Lol what are you doing? I said pick between 1 and 2. try again :(");
        code9_exe();
    }
}






fn main() {
     println!("\nWelcome to Ernst & Young Global Limited\n");
    println!("\nList of Staff Generated By HR Unit for Restructuring\n");
    
let name = vec!["Aigbona Juliet", "Ehis Ero", "Adam Sagamu", "Akpevwe Illoka", "Maria Akinsola", "Gbenga Daniels"];
let dept = vec!["Consulting", "Strategy", "Tax", "Assurance", "Transactions and Corporate finance", "People and workforce"];
let qualification = vec!["B.Sc.", "M.Sc.", "B.Sc.", "HND", "M.Sc.", "HND"];
let code = vec!["Code 7", "Code 9", "Code 8.", "Code 7", "Code 9.", "Code 8"];


for i in 0..code.len()
   {
    // iterating through i on the vector
    print!("\n{}    {}     {}      {}   \n  ",name[i],dept[i],qualification[i],code[i]);

    
   }

   println!("\n");

    code_pick();
   
}
