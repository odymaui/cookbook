pub fn make_change(amount: i32) {
    let coins: Vec<i32> = vec!(500,100,50,25,10,5,1);

    let mut working_amount: i32 = amount;
    let mut results: Vec<i32> = Vec::new();

    for coin in coins {
        let d:i32 = working_amount / coin;
        working_amount = working_amount % coin;
        results.push(d);
    }

    println!("Change Results For {} {:?}", amount, results);
}