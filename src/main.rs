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

/* 
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably =  || list.push(7);

    borrows_mutably();
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

*/



/*
fn main() {
    let list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);

    fn borrow_and_move (mut x:Vec<i32>)->Vec<i32>  {
        if x.len() > 5 {
            println!("here");
            x
        } else {
            x.push(7);
            borrow_and_move(x)
        }
    };
    let list2 = borrow_and_move(list);
    println!("After calling closure: {:?}", list2);
}
*/
use std::time::{Duration, SystemTime};
use std::{env, fs};
use rs_pderiv::regex::re::*;
use rs_pderiv::regex::pderiv::parse::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let time0 = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    match (&args.get(1), &args.get(2)) {
        (Some(n_str), Some(file_path)) => {
            let r = generate_re(n_str);
            // dbg!(&r);
            dbg!(calculate_hash(&r));
            dbg!(calculate_hash(&calculate_hash(&r)));
            let regex = build_regex(&r);
            // println!("built: {}", cnt(&regex));
            let time1 = SystemTime::now();
            println!("{:#?}", time1.duration_since(time0));
            let mut contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
            contents.pop();
            dbg!(contents.len());
            match regex.parse_regex(&contents) {
                None => println!("match failed."),
                Some(x) => println!("{:?}", x)
            };
            let time2 = SystemTime::now();
            println!("{:#?}", time2.duration_since(time1));
        },
        _ => println!("usage: cargo run <num> <filename>")
    }
   
}

fn generate_re(n_arg:&String) -> RE {
    let n = n_arg.parse::<i32>().unwrap();
    mkpat(n)
}

fn mkpat(n:i32) -> RE {
    use RE::*;
    if n > 0 {
        let j = n-1;
        let r = RE::Choice(Box::new(Lit('a')), Box::new(Eps));
        let fst = (0..j).into_iter().fold(r.clone(), |acc,_i| {
            RE::Seq(Box::new(acc),Box::new(r.clone()))
        });
        let t = RE::Lit('a');
        let snd = (0..j).into_iter().fold(t.clone(), |acc,_i| {
            RE::Seq(Box::new(acc),Box::new(t.clone()))
        });
        RE::Seq(Box::new(fst),Box::new(snd))
        // RE::Seq(Box::new(snd),Box::new(fst))
    } else {
        RE::Phi
    }
}


fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}