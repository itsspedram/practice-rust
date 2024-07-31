use std::io;

fn main() {
    loop{
        println!("welcome to calculator");
        println!("Simple Calculator");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("oops");
        let choice:u32 =match choice.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue
        };
        if choice==5 {
            break;
        }
         let mut num1 = String::new();
         println!("Enter your first number ");
        io::stdin().read_line(&mut num1).expect("oops");
         let num1 :i64= match num1.trim().parse(){
            Ok(num)=>num,
            Err(_)=> {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
         };
         let mut num2 = String::new();
         println!("Enter your second number ");
        io::stdin().read_line(&mut num2).expect("oops");
         let num2 :i64= match num2.trim().parse(){
            Ok(num)=>num,
            Err(_)=> {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
         };
         match choice {
             1=>{
                let res = num1 + num2;
                println!("result is {}",res)

             },
             2=>{
                let res = num1 - num2;
                println!("result is {}",res)

             },
             3=>{
                let res = num1 * num2;
                println!("result is {}",res)

             },
             4=>{
                let res = num1 / num2;
                println!("result is {}",res)

             },
             _=>{
                println!("something Went wrong")
             }
         }

    }
}
