use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Life {
    pub num1: u32,
    pub num2: u32
}

pub fn main2() {    
    let mut ll = Life{num1: 11, num2: 22};

    let cm = copy_and_mutate(&mut ll);
    let mc = clone_and_copy(ll);

    println!("{}", ll);
    println!("{}", cm);
    println!("{}", mc);
}

pub fn copy_and_mutate(l: & mut Life) -> Life {
    let copy = *l;
    l.num1 *= 2;
    l.num2 *= 2;

    return copy;
}

pub fn clone_and_copy(l: Life) -> Life {   
    let mut ll = l.clone();
    ll.num1 *= 2;
    ll.num2 *= 2;

    return ll;
}

//implicit
fn foo(x: &i32) -> i32 {
    return *x;
}

// explicit
fn bar<'a>(x: &'a i32) -> i32 {
    return *x;
}

impl fmt::Display for Life {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "num1={} num2={}", self.num1, self.num2)
    }
}
