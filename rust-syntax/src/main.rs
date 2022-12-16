// #[derive(Debug,Clone,Default)]
// struct Person {
//     name: String,
//     age: Option<u8>,
//     salary: u16,
// }


// impl Person {
//     fn new(name:String,age:Option<u8>,salary:u16) -> Self {
        
//         Self {
//             name,
//             age,
//             salary,
//         }
        
        
//     }
// }
#![allow(unused)]


use std::{io, u8};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use rand::Rng;
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;

mod hotel;
use crate::hotel::order_food;


use std::thread;
use std::time::Duration;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc,Mutex};


// Funcs 

fn take2(list:Vec<u8>) -> u8 {
    
    let mut t = 0;
    
    for &i in list.iter(){
        t += &i;
    }
    
    t
    
}

// generics
fn get_sum_gen<T:Add<Output = T>>(x:T,y:T) -> T {
    return x + y;
}


// fn print_str(x: String) {
//     println!("A string {}", x);
// }

// fn print_return_str(x : String){
//     println!("A String {}", x);
// }


// fn change_string(x: &mut String) {
//     x.push_str("ok");
//     println!("{}", x);
// }


fn returnheroes(){
    let mut heroes = HashMap::new();
    heroes.insert("one",1);
    heroes.insert("two",2);
    
    for (k,v) in heroes.iter(){
        println!("{} {}",k,v);
       
    }
    
    if heroes.contains_key("one") {
        let theone = heroes.get(&"one");
        
    }
    
    
}


struct Customer {
    name: String,
    address: String,
    balance: f32,
}

struct Rectangle<T,U> {
    length: T,
    height: U,
}


                      // Smart pointers  //
                      
// Box<T>, 
// Its only purpose is to be stored the type in heap instead of stack. It works liks a borrowed refference

trait Vehicle {
    fn drive(&self);
}

struct Truck {
    //next_truck: Box<Option<Truck>>, // recursive size
    
    capacity: i32,
}

// impl Vehicle for Truck {
//     fn drive(&self) {
//         println!("Truck is driving")
    
// }
// }



// Rc 


fn main () {
    
            // Box 1
    // let t: Box<dyn Vehicle>;
    // t = Box::new(Truck);
    // t.drive();
    
            // Box 2
        // Recursive size
        
        
// Rc
let tr_a = Truck{capacity:1};
let tr_b = Truck{capacity:2};

    

    
    
                               // Threading concurrency
    // common issues 
    // 1. Threads are accessing data in wrong order
    // 2. Threads are blocked from executing because of confusion
    // 3. Over requirements to proceed with execution
    
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("spawned thread {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    
    for i in 1..20 {
        println!("Main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    
    thread1.join().unwrap();
    
    // Thread example
    
    
    pub struct Bank {
        balance: f32
    }
    
    fn withdraw(the_bank: &Arc<Mutex<Bank>>,amt:f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance< 5.00 {
            println!("current {}", bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
        }
    }
    // See rustbank binary for the complete code 
    
    
    
    
    // fn customer (the_bank:&Arc<Mutex<Bank>>) {
    //     withdraw(&the_bank,5.00)
    // }
    
    // let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank{balance:20.0}));
    
    // let handles = (0..10).map(|_| {
    //     let bank_ref:&<Arc<Mutex<Bank>> =bank.clone();
    //     thread::spawn(|| {
    //         customer(bank_ref)
    //     });
    // });
    // fn withdraw(the_bank: &mut Bank, amt:f32) {
    //     the_bank.balance -= amt;
    // }
    
    // let mut bank = Bank{balance:100.0};
    // withdraw(&mut bank, 5.00);
    
    // println!("Balance : {}", bank.balance);
    
    // fn customer(the_bank: &mut Bank){
    //     withdraw(the_bank, 5.00)
    // }
    
    // thread::spawn(|| {
    //     customer(&mut bank)
    // }).join().unwrap();
    
    // A closure and thread can not outlive the main function especially if its borrowing. 
    
    
    
    
    
    
    
    
    // iterotor
    
    let mut arr_it = [1,2,3,4];
    
    // only borrowing not able to change
    for val in arr_it.iter() {
        println!("{}", val);
    }
    
    // Consume the collection but not able to use the old collection
    //let collectiter = arr_it.into_iter();
    
    // creating an iterator
    let mut iter1 = arr_it.iter();
    println!("1 st : {:?}", iter1.next());
    
    
    
    // Closures
    // let var_name = |parameter| -> return_type
    fn main() {
        let can_vote = |age:i32| {
            age >= 18
        };
        println!("can vote{}", can_vote(8))
    }
    
    // Closure can access variables outside of its function unlike function
    let mut samp1 = 5;
    let print_var = || println!("samp1 {}", samp1);
    print_var();
    
    let mut change_var = || samp1 += 1;
    change_var();
    println!("Samp = {}", samp1);
    
    // Generic Closure and pass closure to function
    fn use_func<T>(a:i32, b:i32, func: T) -> i32
    where T: Fn(i32,i32) -> i32 {
        func(a,b)
    }
    
    let sum = |a,b| a+b;
    let prod = |a,b| a*b;
    
    println!("5+4 = {}", use_func(5,4,sum));
    println!("5*8 = {}", use_func(5,8,prod));
    
    
    
    // Smart pointers Box ( strings and vectors are smart pointers) '&' is for reference only and not cleaning up the memory
    // it provides beyond referencing also track ownership of data
    // it stores data in heap not in stack. Stack is in fixed size
    // Heap (Box) stores data in heap.
    
    
    // Box is used for storing large data in Heap but have a pointers in Stack
    let bint1 = Box::new(10);
    println!("bint {}", bint1);
    
    // binary tree
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    
    impl<T> TreeNode<T> {
        pub fn new(key:T)-> Self{
            TreeNode{
                left: None, right:None,key,
            }
        }
        
        // pub fn left(mut self, node:TreeNode<T>) -> Self {
        //     Self {
        //         self.left = Some(Box::new(node));
        //         self
        //     }
        // }
        
        // pub fn right(mut self, node:TreeNode<T>) -> Self {
        //     Self {
        //         self.right = Some(Box::new(node));
        //         self
        //     }
        // }
        
        
    }
    
    // let node1: TreeNode<i32> = TreeNode::new(4)
    // .left(TreeNode::new(2))
    // .right(TreeNode::new(3));
    
    
    
    
    
    
    
    
    
    
    // Creating new file
    let path = "line.txt";
    let output = File::create(path);
    
    // Handling file creation error
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("problem {:?}", error) // you need {:?} because its result enum
        }
    };
    
    // writting data to file and handling error
    write!(output,"just some random words").expect("failed to write");
    
    
    // Opening file and using unwrap as alternative to 
    let input = File::open(path).unwrap();// unwrap ignores result and give output ignores result
    
    // Reading the file and it creates a list.
    let buffered = BufReader::new(input);
    
    for line in buffered.lines()
    {
        println!("{}", line.unwrap());
    }
    
    // Error handling for file output2
    let output2 = File::create("rand.txt");
    // Error handling for file output 2
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create ("rand.txt"){
                Ok(fc) => fc,
                Err(er) => panic!("cant create {:?}",error),
                },
                _other_error=> panic!("prob {:?}", error) 
        }
    };
    
    // using mod
    order_food();
    
    
    
let mut bob:Customer = Customer{
    name:String::from("bob"),
    address:String::from("some street"),
    balance: 234.90
};

// let rec = Rectangle{
//     length:4,
//     height: 10.9
// };


// Trait
trait Shape {
    fn new(length: f32, width: f32) -> Self;
    fn area(&self) -> f32; 
}

struct Rectangle {length:f32,width:f32};
struct Circle{length:f32,width:f32};


impl Shape for Rectangle {
    fn new(length:f32,width:f32) -> Rectangle {
        return Rectangle{length,width};
    }
    fn area(&self) -> f32 {
        return self.length * self.width
    }
}

bob.address = String::from("new address");
    
    
    println!("{}","dd".to_string()+&"dd".to_string());
    
    let list = vec![1,2,3,4,5];
    
    println!("{:?}",take2(list))
    
    
    
    
//     println!("what is your name?");
    
//     // by default immutable
//     let mut name = String::new();
//     let greeting = "Nice to meet you";
    
//     // io::stdin().read_line(&mut name).expect("Didnt receive");
    
    
//     // println!("Hello {}! {}", name.trim_end(), greeting)
    
//     const ONE_MIL: usize = 1_000_000;
//     const PI: f32 = 3.14234;
//     let age = "38";
//     let mut age: u32 = age.trim().parse().expect("age wasnt assigned number");

    
//     let random_num = rand::thread_rng().gen_range(1..101);
    
    
    
//     // let whatisimp = if (age>2) && (age < 1) {
//     //     println!("important birthday");
//     // } else if (age == 21) || (age == 50) {
//     //     println!("important birthday")
//     // } else {
//     //     println!("not important")
//     // };
    
    
//     let age2 = 8;
//     match age2.cmp(&age) {
        
//         Ordering::Less => println!("Cant Vote"),
//         Ordering::Greater => println!("Yes vote"),
//         Ordering::Equal => println!("Hi")
//         // 1..=18 => println!("Important Birthday"),
//         // 21 | 50 => println!("not imp"),
//         // 65..=u32::MAX => println!("not imp"),
//         // _ => println!("not imp")
    
// }
    
    
//     // array
    
//     let arr = [1,2,3,5,7,8,9];
//     println!("second item {}",arr[1]);
//     println!("ddd {}",arr.len());
    
    
//     // loop 
//     let mut loopindex = 0;
    
//     loop {
//         if arr[loopindex] % 2 == 0 {
//             loopindex +=1;
//             continue;
//         }
        
//         if arr[loopindex] == 9 {
//             break;
//         }
        
//         println!("odd vals are {}", arr[loopindex]);
//         loopindex +=1;
        
//     }
    
//     while loopindex < arr.len() {
//         println!("Arr: {}", arr[loopindex]);
//         loopindex +=1;
// }
    
//     for val in arr.iter() {
//             println!("Val :{}", val);
//         }
    
    
    
//     // tuple
    
//     let mytuple = (44,"hello".to_string(),true);
    
    
//     // String
    
//     let mut str1 = String::new();
    
//     str1.push('A');
//     str1.push_str("hello");
    
//     for word in str1.split_whitespace() {
//         println!("{}", word);
//     }
    
//     let st3 = String::from("x r t b h k k a m c");
    
//     let mut v1: Vec<char> = st3.chars().collect();
//     v1.sort();
//     v1.dedup();
    
    
//     for char in v1 {
//         println!("{}", char);
//     }
    
//     // slice and string from
//     let st4 = "Hello world";
    
//     let st8 = String::from(st4);
    
//     let bytar= st8.as_bytes(); // as bytes 
    
    
//     let st6 = &st8[0..3];
//     println!("{}{}",st6,st6.len());
    
    
//     let st6= String::from("Just Some");
//     let str8 = String::from("words");
    
//     let st9 = st6 + &st8; // here st6 is moved so nonexistent but str8 only referenced so both str8 exists.
    
    
    
//     // cast
//     let intu8:u8= 5;
//     let intu32:u32=444;
    
//     let casting = intu8 + intu32 as u8;

//     // age = age + 1;
//     println!("i am {} {}",str8,st9);
    
    
//     // enum
    
//     enum Days {
//         Monday,
//         Tuesday,
//         Wednesday,
//         Thrusday,
//         Saturday
//     }
    
//     impl Days {
//         fn is_weekend(&self) -> bool {
//             match self {
//                 Days::Saturday | Days::Monday => true,
//                 _ => false
//             }
//         }
//     }
    
//     let today = Days::Monday;
    
//     // Vectors
    
//     let vec1: Vec<i32> = Vec::new();
    
//     let mut vec2 = vec![1,2,3,4,5];
//     vec2.push(5);
    
//     println!("1st {}", vec2[0]);
    
//     let second = vec2[1];
    
//     match vec2.get(2) {
//         Some(second) => println!("{}",second),
//         None => println!("failed")
//     }
    
    
//     for i in &mut vec2 {
//         *i *= 2;
//     }
    
    
//     for i in &vec2 {
//         println!("{}", i)
//     }
    
    
    
    
//     println!("1st : {}",second );
//     println!("{:?}",vec2)
    
    
    
    // let age = Some(34);
    // // get the name
    // let new_person = Person::new("upal".to_string(),7986986986,2000);
    
    // println!("{:?} {:?} {:?}",new_person.name,new_person.age,new_person.salary)

}