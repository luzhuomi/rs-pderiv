use core::ops::*;

/** 
 * the RE data type
 */
#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

pub fn member<T:Eq>(t:&T, l:&List<T>) -> bool {
    match l {
        List::Nil => false,
        List::Cons(x,xs) => {
            if t == x {
                true
            } else {
                member(t, xs)
            }
        }
    }
}

impl <T:Eq>List<T> {
    pub fn contains(&self, t:&T) -> bool {
        member(t,self)
    }
}

impl <T:Eq + Clone>List<T> {
    pub fn nub(&self) -> List<T> {
        match self {
            List::Nil => List::Nil,
            List::Cons(x,xs) => {
                if xs.contains(x) {
                    xs.nub()
                } else {
                    List::Cons(x.clone(), Box::new(xs.nub()))
                }
            }
        }
    }    
}


impl <A:Clone>List<A> {
    pub fn append(&self, s:List<A>) -> List<A> {
        match self {
            List::Nil => s,
            List::Cons(x, xs) => {
                List::Cons(x.clone(), Box::new(xs.append(s)))
            }
        }
    }
}



impl <A>List<A> {
    pub fn length(&self) -> i32 { 
        match self {
            List::Nil => 0,
            List::Cons(_x,xs) => 1 + xs.length() 
        }
    }
    
    pub fn map<B>(&self, f:impl Fn(&A) -> B) -> List<B> {
        match self {
            List::Nil => List::Nil,
            List::Cons(x,xs) => List::Cons(f(x), Box::new(xs.map(f)))
        }
    }    
}
