use core::ops::*;
use std::rc::Rc;


// it seems that we don't need this. we used Vec

/** 
 * List Algebraic data type
 */
#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Rc<List<T>>)
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

impl <T:Eq> PartialEq for List<T> {
    fn eq(&self, other:&Self) -> bool {
        match (self, other) {
            (List::Nil,List::Nil) => true,
            (List::Cons(x,xs), List::Cons(y,ys)) => x == y && xs == ys,
            (_,_) => false
        }
    }
}

impl <T:Eq> Eq for List<T> { }

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
                    List::Cons(x.clone(), Rc::new(xs.nub()))
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
                List::Cons(x.clone(), Rc::new(xs.append(s)))
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
            List::Cons(x,xs) => List::Cons(f(x), Rc::new(xs.map(f)))
        }
    }

    pub fn foldl<B>(&self, acc:B, f: impl Fn(B, &A) -> B) -> B {
        match self {
            List::Nil => acc,
            List::Cons(x,xs) => xs.foldl(f(acc,x), f)
        }
    }    
}
