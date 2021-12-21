

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
