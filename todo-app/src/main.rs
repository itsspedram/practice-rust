use core::num;
use std::{fmt, io};

struct ToDoItem{
    id:u64,
    title : String,
    completed:bool,
}
struct TodoList{
    items :Vec<ToDoItem>,
}
impl TodoList {
    fn new()->TodoList{
        TodoList{items:Vec::new()}
    }
    fn add_item(&mut self,title:String){
        let id = self.items.len() as u64 +1;
        let new_item = ToDoItem{
            id,
            title:title.clone(),
            completed :false,
        };
        self.items.push(new_item);
        println!("added new to do {}",title)
    }
    fn list_items(&self){
        if self.items.is_empty(){
            println!("Your to-do list is empty.");
        }
        else {
            println!("your tododlist is :");
            for item in &self.items{
                let status = if item.completed {"[X]"} else {"[]"};
                println!("{} {} - {}",status,item.id,item.title)
            }
        }
    }
    fn complet_item(&mut self , id:u64){
            if let Some(item)= self.items.iter_mut().find(|i| i.id==id){
                item.completed = true;
                println!("{} is done ", item.title)
            }
            else {
                println!("item did not found")
            }
    }
    
}
fn main() {
 let mut todo_list = TodoList::new();
 loop {
    println!("add. Add Item");
    println!("all. List Items");
    println!("done. Complete Item");
    println!("exit. Exit");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim();

    match choice{
        "add" => {
            loop {
                
            
            println!("Enter the title of the new item:");
            let mut title = String::new();
            io::stdin().read_line(&mut title).expect("oooops");
            todo_list.add_item(title.trim().to_string());
             let mut answer= String::new();
            println!("Do u want to add more ?");
            io::stdin().read_line(&mut answer).expect("err");
            let answer = answer.trim();
            match answer {
                "no"=>{ break;}
                _=>{continue;}
            }
        }
        }
        "all"=> {
            todo_list.list_items()
        }
        "done" => { loop {
            
        
            println!("Enter the id of the item:");
            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("OOPS");
            let id :u64 = match id.trim().parse(){
                Ok(num)=>num,
                Err(_)=>continue,
            };
            todo_list.complet_item(id);

            let mut answer= String::new();
            println!("Do u want to add more ?");
            io::stdin().read_line(&mut answer).expect("err");
            let answer = answer.trim();
            match answer {
                "no"=>{ break;}
                _=>{continue;}
            }
        }
        }
        "exit" => {
            println!("Exiting the program.");
                break;
        }
        _ => {
            println!("Invalid choice. Please enter a number between 1 and 4.");
        }
    }
 }
}