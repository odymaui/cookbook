

pub fn simple_cast() {
    let i64value: i64 = 255;
    let casted_value: u8 = i64value as u8;

    //size_of_val expects a reference to a type...
    println!("Max Byte Values: {} takes {} byte(s)", casted_value, std::mem::size_of_val(&casted_value));
}


type SpecialName = u64;

pub fn simple_type_alias() {
    
    let special_value: SpecialName = 552;

    //size_of_val expects a reference to a type...
    println!("Special Value {}", special_value);
}

pub fn get_stringify_result() {

    let _foo = simple_type_alias();

    //doesn't do any calculation of input expresssion, just convert expression to str...
    println!("stringify!(foo) : {}", stringify!(_foo));
    println!("stringify!(foo) : {}", stringify!(simple_type_alias()));
}

pub fn run_fibonacci_sequence(sequence_count: usize) {
    /*
        0th - 0
        1 - 1
        fn = fn-1 + fn-2

        0 1 1 2 3 5 8 13 21 34 55
    */

    let mut fib = Fibonacci::default();

    for _ in 0..sequence_count {
        println!("{:?}", &fib);
        fib = fib.next().unwrap();
    }

    //Note, skips the first entry
    for fibn in Fibonacci::default().take(sequence_count) {
        println!("{:?}", &fibn);
    }
}

#[derive(Debug)]
pub struct Fibonacci {
    current: u64,
    next: u64,
    index: u32
}

impl Default for Fibonacci {
    fn default() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
            index: 0
        }
    }
}

impl Iterator for Fibonacci {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {

        let tmp = self.current + self.next;
        self.current = self.next;
        self.next = tmp;
        self.index = self.index + 1;
        Some(
            Fibonacci {
                current: self.current,
                next: self.next,
                index: self.index
            })
    }
}