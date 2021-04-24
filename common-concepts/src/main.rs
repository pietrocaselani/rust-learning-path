use std::collections::HashMap;

fn main() {
    let a_number = 10;
    let a_boolean = true;

    // println!("the number is {}.", a_number);
    // println!("the boolean is {}.", a_boolean);
    println!("the number is {} and the boolean is {}.", a_number, a_boolean);

    // Immutability

    // println!("the number is {}.", a_number);
    // a_number = 15;
    // println!("and now the number is {}.", a_number);

    // Shadowing

    let number = 5;          // the first binding is created using the name "number"
    let number = number + 5; // a different binding shadows the name "number"
    let number = number * 2; // again, a new binding is created
    println!("The number is: {}", number);

    // Data types

    let number: u32 = "42".parse().expect("Not a number!");
    println!("[Data types]: Number is {}", number);

    // Math operations
    // Addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // ^ Try changing `1i32` to `1u32` to see why the type is important

    // Integer Division
    println!("9 / 2 = {}", 9u32 / 2);

    // Float Division
    println!("9 / 2 = {}", 9.0 / 2.0);

    // Multiplication
    println!("3 * 6 = {}", 3 * 6);

    // Bools

    let is_bigger = 1 > 4;
    println!("1 > 4 is {}", is_bigger);  // prints "false"

    // Chars
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("var c is {}", c);
    println!("var z is {}", z);
    println!("var heart_eyed_cat is {}", heart_eyed_cat);

    // Strings

    let mut hello = String::from("Hello, ");  // create a String from a string literal
    hello.push('w');                          // push a character into our String
    hello.push_str("orld!");                  // push a string literal into our String
    println!("{}", hello);

    // Tuples

    let tuple = ("hello", 5, 'c');
    // tuple signature (&'static str, i32, char)

    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
    // assert_eq! is a macro to check expressions are equal

    // Structs

    // Classical struct with named fields
    struct Person {
        name: String,
        age: u8,
        likes_oranges: bool
    }

    // Tuple struct
    struct Point2D(u32, u32);

    // Unit struct
    struct Unit;

    // Initializing structs

    // Order doesn't matter for named structs
    let person = Person {
        name: String::from("PC"),
        likes_oranges: true,
        age: 28
    };

    // Order matters for tuple structs
    let origin = Point2D(0, 0);

    // Initializing a Unit struct
    let unit = Unit;

    // Enums

    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 }
    }

    // Exercise: Fixing the code

    struct Car {
        color: String,
        transmission: Transmission,
        convertible: bool,
        mileage: u32,
    }

    #[derive(PartialEq, Debug)]
    enum Transmission {
        Manual,
        SemiAuto,
        Automatic,
    }

    fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
        let car = Car {
            color: color,
            transmission: transmission,
            convertible: convertible,
            mileage: 0
        };

        // Factory's Quality Control Department says that new cars must always have zero mileage!
        assert_eq!(car.mileage, 0);
        return car;
    }

    let client_request_1 = car_factory(String::from("Red"), Transmission::Manual, false);
    assert_eq!(client_request_1.color, "Red");
    assert_eq!(client_request_1.transmission, Transmission::Manual);
    assert_eq!(client_request_1.convertible, false);

    let client_request_2 = car_factory(String::from("Silver"), Transmission::Automatic, true);
    assert_eq!(client_request_2.color, "Silver");
    assert_eq!(client_request_2.transmission, Transmission::Automatic);
    assert_eq!(client_request_2.convertible, true);

    let client_request_2 = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    assert_eq!(client_request_2.color, "Yellow");
    assert_eq!(client_request_2.transmission, Transmission::SemiAuto);
    assert_eq!(client_request_2.convertible, false);

    another_function();

    assert_eq!(is_divisible_by(2, 3), false);
    assert_eq!(is_divisible_by(5, 1), true);
    assert_eq!(is_divisible_by(24, 6), true);

    // Arrays

    // a comma-separated list inside of brackets
    let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

    // initialize an array of 512 elements where every element is a zero
    let byte_buffer = [0_u8; 512];

    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    println!("first element of the array: {}", letters[0]);  // prints 'a'
    println!("second element of the array: {}", letters[1]); // prints 'b'

    // our `letters` array has only 7 elements
    // Compiler error
    //println!("invalid array access: {}", letters[99]);

    // Vectors

    let three_numbers = vec![1, 2, 3];
    println!("Initial vector: {:?}", three_numbers);

    let ten_zeros = vec![0; 10];
    println!("Ten zeros: {:?}", ten_zeros);

    let mut mutable_vector = Vec::new();
    mutable_vector.push(5);
    mutable_vector.push(6);
    mutable_vector.push(7);
    mutable_vector.push(8);
    println!("mutable_vector = {:?}", mutable_vector);

    let mut v = vec![1, 2];
    println!("v = {:?}", v);
    let two = v.pop();
    println!("v = {:?}", v);

    let mut v = vec![1, 2, 3];
    println!("v = {:?}", v);
    let three = v[2];
    v[1] = v[1] + 5;
    println!("v = {:?}", v);

    // Panic accessing index out of bounds
    // let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = v[100];

    // HashMap

    let mut book_reviews: HashMap<String, String> = HashMap::new();

    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    println!("Book reviews {:?}", book_reviews);

    if !book_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.", book_reviews.len());
    }

    // Searching for an existing key returns the value associated to it
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);

    // But searching for a nonexisting key will cause a panic
    // println!("Review for Herman: {}", book_reviews["Moby Dick"]);  // panics!

    let sherlock = "The Adventures of Sherlock Holmes";
    assert_eq!(book_reviews.contains_key(sherlock), true);
    book_reviews.remove(sherlock);
    assert_eq!(book_reviews.contains_key(sherlock), false);

    // Exercise: Indexing

    indexing_tuple();
    indexing_array();

    // Exercise: HashMap

    let basket = fruit_basket();
    assert!(
        basket.len() >= 3,
        "basket must have at least three types of fruits"
    );
    assert!(
        basket.values().sum::<u32>() >= 5,
        "basket must have at least five fruits"
    );

    // Conditional expressions

    if 1 == 2 {
        println!("whoops, mathematics broke");
    } else {
        println!("everything's fine!");
    }

    // If block are expressions, meaning they return values
    let formal = true;
    let greeting = if formal {
        "Good evening."
    } else {
        "Hello, friend!"
    };
    println!("{}", greeting); // prints "Good evening."

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Loops

    // Forever
    // loop {
    //     println!("I loop forever");
    // }

    let mut i = 1;
    let something = loop {
        i *= 2;
        if i > 100 {
            break i;
        }
    };
    assert_eq!(something, 128);

    let mut counter = 0;

    while counter < 10 {
        println!("hello");
        counter = counter + 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for item in 0..5 {
        println!("{}", item * 2);
    }

}

fn another_function() {
    println!("Hello from another function!");
}

fn is_divisible_by(
    dividend: u32,
    divisor: u32
) -> bool {
    if divisor == 0 {
        return false;
    }
    dividend % divisor == 0
}

fn indexing_tuple() {
    let numbers = (1, 2, 3);
    let second = numbers.1;

    assert_eq!(
        2, second,
        "This is not the 2nd number in the tuple: {}",
        second
    )
}

fn indexing_array() {
    let characters = ['a', 'b', 'c', 'd', 'e'];
    let letter_d = characters[3];

    assert_eq!(
        'd', letter_d,
        "This is not the character for the letter d: {}",
        letter_d
    )
}

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    basket.insert(String::from("banana"), 2);
    basket.insert("Apple".to_string(), 2);
    basket.insert("Mango".to_string(), 1);

    basket
}
