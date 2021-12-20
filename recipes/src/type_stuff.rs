use core::any::{Any, TypeId};
use std::ops::Deref;

struct Rectangle;
struct Triangle;

trait Shape: Any {}

impl Shape for Rectangle {}
impl Shape for Triangle {}

//https://www.jakobmeier.ch/blogging/Untapped-Rust.html

//Note the usage example at the end of the article but not going to investigate here.

pub fn example_type_test() {
    let one_hundred = 100u32;
    // Get the type ID usaing a value of that type.
    let t0 = one_hundred.type_id();
    // Get the type ID directly
    let t1 = TypeId::of::<u32>();
    assert_eq!(t0, t1);
    println!("Types: {:?} <-> {:?}", t0, t1);
    //typeid mapping to a name?
}

pub fn count_shapes() {
    let expected_count : usize = 2;
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Rectangle), Box::new(Triangle), Box::new(Rectangle)];
    let n = count_rectangles(&shapes);

    println!("Count Of Rectangles... {:?}", n);

    assert_eq!(expected_count, n);
}

fn count_rectangles(shapes: &[Box<dyn Shape>]) -> usize {
    let mut n = 0;
    for shape in shapes {
        // Need to derefernce once or we will get the type of the Box!
        let type_of_shape = shape.deref().type_id();
        //note this doesn't work because is looks like it is treated as dyn Shape...
        //let type_of_shape = (*shape).type_id();
        
        if type_of_shape == TypeId::of::<Rectangle>() {
        println!("{:?} is a Rectangle!", type_of_shape);
            n += 1;
        } else {
            println!("{:?} is not a Rectangle!", type_of_shape);
            //println!("{:?} is not a Rectangle!  It's a dyn shape?? {}", type_of_shape, (TypeId::of::<dyn Shape>() == TypeId::of::<dyn Shape>()));
        }
    }
    n
}

