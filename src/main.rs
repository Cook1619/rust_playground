// Seems like a type or interface in TS
struct BankAccount {
    balance: i32,
    verified: bool
}
// The & lets ownership happen
fn print_balance(account: &BankAccount){
    println!("{:?}", account.balance);
}

fn print_verified(account: &BankAccount){
    println!("{:?}", account.verified);
}

fn is_verified(account: &BankAccount) -> Result<bool, bool>{
    match account.verified {
        true => Ok(true),
        false => Err(false)
    }
}

fn add(num_one: i32, num_two: i32) -> i32 {
    num_one + num_two
}

fn main() {
    let mut total = add(21, 5);
    let mut free_shipping = false;

    if total > 50 {
        println!("You qualify for free shipping");
        free_shipping = true;
    }
    else if total > 20 {
        println!("If you add more items, you can qualify for free shipping!")
    }
    else {
        println!("No free shipping")
    }

    total = match free_shipping  {
        true => total + 0,
        false => total + 5
    };

    println!("Your total was: {}", total);

    // Array -> fixed sized, can't push data into them
    let items: [i32;5] = [1,2,3,4,5];
    println!("Normal array: {:?}", items);

    // Vector -> dynamic sized -> takes up more storage compared to an array
    let vector_items = vec![1,2,3,4,5];
    // Another way to create vectors
    let mut vector_items_2 = Vec::new();
    vector_items_2.push(1);
    vector_items_2.push(2);
    vector_items_2.push(3);
    vector_items_2.push(4);
    vector_items_2.push(5);
    println!("Vector 1: {:?}", vector_items);
    println!("Vector 2: {:?}", vector_items_2);

    let my_account = BankAccount {
        balance: 10000,
        verified: false,
    };
    // Dot notation to access a value from a struct
    // println!("{:?}", my_account.verified);
    print_balance(&my_account);
    // this wont work because print_balance has the ownership, it does this for memory leaks, we can get around this by borrowing ownership
    // this is done in the function signature and function call with the & symbol
    print_verified(&my_account);

    let verification_status = is_verified(&my_account)
        .expect("Unable to unwrap result");
    println!("Verification Status: {:?}", verification_status);
}

