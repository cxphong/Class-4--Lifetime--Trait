// Bài 2: Lifetime
// Yêu cầu: In dãy Fibonacci

#[derive(Debug)]
struct Fibonacci {
    a: u32,
    b: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let next = self.a + self.b;

        self.a = self.b;
        self.b = next;

        Some(self.a)
    }
}

fn fibonacci_numbers() -> Fibonacci {
    Fibonacci { a: 1, b: 0 }
}

fn main() {
    for number in fibonacci_numbers() {
        println!("{}", number);
    }
}


// Bài 2: Lifetime
// Yêu cầu: Sửa lỗi Lifetime 

use std::fmt;
struct StrDisplayable<'a>(Vec<&'a str>);

impl<'a> fmt::Display for StrDisplayable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "\n{}", v)?;
        }
        Ok(())
    }
}

fn main() {
        let vec: Vec<&str> = vec!["a","bc","def"];
        let vec_Foo = StrDisplayable(vec);
        println!("{}",vec_Foo);
}
