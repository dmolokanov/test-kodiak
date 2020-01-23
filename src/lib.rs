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
}

pub enum B {
    V1,
    V2(String),
    V3(bool),
}

fn main() {
    println!("work")
}
