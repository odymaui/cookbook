pub fn sample_work_with_next()
{
    //normally just for x in  
    //or while next this is a slight variation
    let a = [1, 2, 3];
    //let a: [i32;0] = [];

    let mut iter = a.iter();

    println!("Count: {:?}", &iter.size_hint());

    let mut nxt = iter.next();

    if nxt.is_some() {
        println!("first value: {}", nxt.unwrap());
    }


    nxt = iter.next();

    if nxt.is_some() {
        println!("next value: {}", nxt.unwrap());
    }

}