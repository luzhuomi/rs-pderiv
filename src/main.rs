/*
fn main() {
    println!("Hello, world!");
}
*/

// use std::io;

/* fn main() {
x    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    foo();

    

}


fn foo() {
    let mut x = String::from("hello");

    let r1 = bar(&x);
    let r2 = &x;
    println!("{r1} {r2}");
}

fn bar(s:&String) -> String {
    let t = s.to_string() + "bye";
    t // &t wont work as it will leak t out of scope
}

fn multref() {
    let mut x = String::from("hello");
    let r1 = &mut x;
    let r2 = &x; 
    // print!("{r1} {r2}"); // error, extending r1's life time here., x2 cant borrow
}

/*
fn main() {
    foo();
}

*/
struct AlwaysEqual;

struct Another;


fn main() {
    let x = AlwaysEqual;
    let y = AlwaysEqual;
}
*/

fn main() {

}