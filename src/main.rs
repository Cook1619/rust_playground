// Seems like a type or interface in TS
struct BankAccount {
    balance: i32,
    verified: bool
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
    println!("{:?}", my_account.balance);
    println!("{:?}", my_account.verified);
}

