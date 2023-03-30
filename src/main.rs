
use std::time::SystemTime;
use std::rc::Rc;
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
            println!("built: {}", cnt(&regex));
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
        let r = RE::Choice(Rc::new(Lit('a')), Rc::new(Eps));
        let fst = (0..j).into_iter().fold(r.clone(), |acc,_i| {
            // RE::Seq(Rc::new(acc),Rc::new(r.clone()))
            RE::Seq(Rc::new(r.clone()), Rc::new(acc))
        });
        let t = RE::Lit('a');
        let snd = (0..j).into_iter().fold(t.clone(), |acc,_i| {
            // RE::Seq(Rc::new(acc),Rc::new(t.clone()))
            RE::Seq(Rc::new(t.clone()), Rc::new(acc))
        });
        RE::Seq(Rc::new(fst),Rc::new(snd))
        // RE::Seq(Rc::new(snd),Rc::new(fst))
    } else {
        RE::Phi
    }
}


fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}