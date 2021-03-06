#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let a = super::A::new(3);
        let mut b = 3;
        b += a.b;

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_also_works() {
        assert_eq!(2 + 2, 4);
    }
}

// some comments
pub struct A {
    pub b: i32,
}

impl A {
    pub fn new(b: i32) -> Self {
        let a: bool = true;
        println!("a log message");
        Self { b }
    }

    pub fn print(&self) {
        println!("{}", self.b)
    }
}

pub enum B {
    V1,
    V2(String),
    V3(bool),
}

// Function that returns a boolean value
fn can_divide(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) {
    if can_divide(n, 15) {
        println!("fizzbuzz");
    } else if can_divide(n, 3) {
        println!("fizz");
    } else if can_divide(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}

