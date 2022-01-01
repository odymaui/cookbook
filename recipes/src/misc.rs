pub fn print_ruler() {
    let mut ruler: Vec<String> = Vec::new();
    ruler.push(String::from("1"));
    for v in [2,3,4,5] {
        let crnt = ruler.last().unwrap();
        let result = format!("{}|{}|{}", crnt, v, crnt);
        ruler.push(result);
    }
    println!("{}", ruler.last().unwrap());
}


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

    //this prints the zero because it prints before next is called...
    for _ in 0..sequence_count {
        println!("{:?}", &fib);
        fib = fib.next().unwrap();
    }

    //Note, skips the first entry
    //also no need to call unwrap...
    for fibn in Fibonacci::default().take(sequence_count) {
        println!("{:?}", &fibn);
    }

    let fibiter = Fibonacci::default().take(sequence_count);
    //note the take wrapper of the iterator...
    println!("Zero iter ver: {:?}", fibiter);

    for val in fibiter {
        println!("iter ver: {:?}", val);
    }

}

pub fn get_fibonacci_sequence() -> Fibonacci {
    //since this return outside module
    //if struct fields are not public not visible
    //however if in the module they are public...
    Fibonacci::default()
}

pub fn iterator_processing() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("v1 iterator: {}", val);
    }
}

#[derive(Debug)]
pub struct Fibonacci {
   pub current: u64,
   next: u64,
   pub index: u32
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

impl std::fmt::Display for Fibonacci {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "diplay trait format: {} -> {}", self.index, self.current)
    }
}

/*
note: conflicting implementation in crate `alloc`:
            - impl<T> ToString for T
              where T: std::fmt::Display, T: ?Sized;

impl std::string::ToString for Fibonacci {
    fn to_string(&self) -> String {
        format!("{} -> {}", self.index, self.current)
    }
}
*/

/* //not working as needed - come back later.
impl FromIterator<Fibonacci> for Fibonacci {
    fn from_iter<I: IntoIterator<Item=Fibonacci>>(_iter: I) -> Self {
        Fibonacci::default()
    }
}
*/

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