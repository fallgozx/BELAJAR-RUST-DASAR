
fn main() {
    println!("Hello, world!");
}

#[test]
fn unit_test() {
   println!("This is a unit test.");
}

#[test]
fn test_variable() {
    let name = "Alice";
    print!("Hello, {}!", name);
}

#[test]
fn test_mutable_variable() {
    let mut count = 0;
    count += 1;
    println!("Count: {}", count);
}

// #[test]
// fn static_variable() {
//     let mut static_count = 12;
//     print!("Static Count: {}", static_count);

//     // static_count = "makan"
//     // print!("Static Count: {}", static_count);
// }

#[test]
fn shadowing_variable() {
    let name = "Alice";
    println!("Hello, {}!", name);

    let name = "Bob";
    println!("Hello, {}!", name);
}

#[test]
fn number_constants() {
    const PI: i8 = 3;
    let b: i16 = PI as i16;
    let c: i32 = PI as i32;
    let d: i64 = PI as i64;
    println!("PI as i8: {}", PI);
    println!("PI as i16: {}", b);
    println!("PI as i32: {}", c);
    println!("PI as i64: {}", d);
}

#[test]
fn tuple() {
    let person: (&str, i32) = ("Alice", 30);
    println!("Name: {}, Age: {}", person.0, person.1);
}

#[test]
fn tuple_access() {
    let person: (&str, i32) = ("Alice", 30);
    let (_name, _age) = person;
    let name = person.0;
    let age = person.1;
    println!("Name: {}, Age: {}", name, age);
    println!("Name: {}, Age: {}", _name, _age);
}

#[test]
fn tuple_destructuring() {
    let person: (&str, i32) = ("Alice", 30);
    let (name, age) = person;
    println!("Name: {}, Age: {}", name, age);
}

#[test]
fn tuple_mutable() {
    let mut person: (&str, i32) = ("Alice", 30);
    person.0 = "Bob";
    person.1 = 25;
    println!("Name: {}, Age: {}", person.0, person.1);
}

#[test]
fn array() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("Matrix: {:?}", matrix);
}

//Memory management
#[test]
fn  stack_and_heap() {
    let stack_variable = 42; // Stored on the stack
    let heap_variable = Box::new(42); // Stored on the heap

    println!("Stack variable: {}", stack_variable);
    println!("Heap variable: {}", heap_variable);
}

// &str slices
#[test]
fn string_slices() {
    let s: &str = "Hello, world!";
    println!("String slice: {}", s);
}

#[test]
fn string() {
    let name: &str = " Alice ";
    let trimmed_name = name.trim();
    println!("Trimmed name: '{}''{}'", trimmed_name,name);
}

// string
#[test]
fn string_heap() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("String: {}", s);
    let s = s.replace("hello", "hi");
    println!("String after replace: {}", s);
}

//ownership
#[test]
fn ownership_rules() {
    // a belum di deklarasikan jadi tidak bisa diakses
    let a = 10 ;// a bisa diakses karena sudah di deklarasikan

    {
        let b = 20; // b hanya bisa diakses dalam scope ini
        println!("Inside block: a = {}, b = {}", a, b);
    }

        // b tidak bisa diakses karena sudah keluar dari scope
    println!("Outside block: a = {}", a); // a masih bisa diakses karena berada di scope yang lebih luas
}

// Data copy ownership
#[test]
fn data_copy() {
    let x = 5;
    let y = x; // y is a copy of x, both x and y can be used independently
    println!("x: {}, y: {}", x, y);
}

// Data move ownership terjadi data yang disimpan di heap, seperti String, akan dipindahkan ke variabel baru saat diassign, sehingga variabel lama tidak bisa digunakan lagi untuk mengakses data tersebut.
#[test]
fn data_move() {
    let s1 = String::from("Hello");
    let s2 = s1; // s1 is moved to s2, s1 can no longer be used
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // This will cause a compile-time error
}

//cloning data ownership terjadi ketika kita ingin membuat salinan data yang disimpan di heap, seperti String, kita bisa menggunakan metode clone() untuk membuat salinan data tersebut, sehingga kedua variabel dapat digunakan secara independen.
#[test]
fn data_cloning() {
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // s1 is cloned to s2, both s1 and s2 can be used independently
    println!("s1: {}, s2: {}", s1, s2);
}

//loop label
#[test]
fn loop_label() {
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i == 2 && j == 2 {
                break 'outer; // Breaks out of the outer loop
            }
            println!("i: {}, j: {}", i, j); 
        }
    }
}   

#[test]
fn while_loop() {
    let mut count = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }
}

// method 

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_hello(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }
}

#[test]
fn test_person() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    person.say_hello();
}   


// enum 

enum  Level {
    Low,
    Medium,
    High,
}

#[test]
fn test_level() {
    let _level : Level = Level::Medium;
    match _level {
        Level::Low => println!("Level is Low"),
        Level::Medium => println!("Level is Medium"),
        Level::High => println!("Level is High"),
        
    }
}

//enum tuple
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

#[test]
fn test_message() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello"));   
    match msg1 {
        Message::Quit => println!("Message: Quit"),
        Message::Move { x, y } => println!("Message: Move to ({}, {})", x, y),
        Message::Write(text) => println!("Message: Write '{}'", text),
    }
    match msg2 {
        Message::Quit => println!("Message: Quit"),
        Message::Move { x, y } => println!("Message: Move to ({}, {})", x, y),
        Message::Write(text) => println!("Message: Write '{}'", text),
    }
    match msg3 {
        Message::Quit => println!("Message: Quit"),
        Message::Move { x, y } => println!("Message: Move to ({}, {})", x, y),
        Message::Write(text) => println!("Message: Write '{}'", text),
    }
}