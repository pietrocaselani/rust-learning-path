fn main() {
    let mascot = String::from("ferris");

    // Transfering ownership to ferris binding
    let ferris = mascot;

    // println!("{}", mascot); // Compile error. Using mascot after borrow
    println!("{}", ferris);

    let s = String::from("Hello");
    process(s); // Ownership of s moved to `process` function
                // //Error! Ownership already moved
                // process(s);

    let number = 1u32;
    process_number(number); // Owership is copied to process_number
    process_number(number); // `number` can be used again because was copied and not moved

    // Copying values that don't implement [Copy]

    let str = String::from("World");
    process(str.clone()); // Passing a clone of str
    process(str); // str was never moved, we still have ownership

    // Borrowing & References

    let greeting = String::from("Hello");
    let greeting_reference = &greeting; // Borrowing `greeting` but the data is still owned by `greeting`
    println!("Greeting: {}", greeting); // `greeting` can be used since ownership didn't change

    print_greeting(greeting_reference);
    print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String`. We are borrowing with `&`
    print_greeting(&greeting); // `greeting` can be used since ownership didn't change

    // Mutating borrowed values
    // let muthello = "Hello".to_string();
    // mutate_message(&hello); `&` borrows immutable

    let mut hello = "Hello".to_string();
    mutate_message(&mut hello); // `&mut` borrows mutable

    // Borrowing & Mutable References

    // You can have either of the following but not both at the same time:
    // * One or more immutable references (&T)
    // * Exactly one mutable reference (&mut T)

    let mut value = "Hello".to_string();

    let ref1 = &mut value;
    // let ref2 = &mut value; // Compiler error here: `second mutable borrow occurs here`
    // println!("{}, {}", ref1, ref2);

    // compiler will error even when mixing mutable and immutable refs
    // `cannot borrow `helloValue` as mutable because it is also borrowed as immutable`
    let mut helloValue = "Hello".to_string();
    let helloValueRef1 = &helloValue;
    // let helloValueRef2 = &mut helloValue; // `mutable borrow occurs here`
    // println!("{}, {}", helloValueRef1, helloValueRef2);

    // Validating references by using lifetimes

    // Snippet will throw compiler error: `y` does not live long enough
    /*
    let x;
    {
        let y = 42;
        x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
    }
    println!("x: {}", x); // `x` refers to `y` but `y has been dropped!
    */

    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);

    /*
    // Compiler error: `magicWord2` doesn't live long enough. Doesn't have the same lifetime as `magicWord1` and `magicResult`
    let magicWord1 = String::from("abracadabra!");
    let magicResult;
    {
        let magicWord2 = String::from("shazam!");
        magicResult = longest_word(&magicWord1, &magicWord2);
    }
    println!("The longest magic word is {}", magicResult);
    */

    // Annotating lifetimes in types

    // Means that an instance of Highlight can't live longer than an instance of `& str` that is borrowing
    #[derive(Debug)]
    struct Highlight<'document>(&'document str);

    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);

    // Compiler error: cannot move out of `text` because it is borrowed
    // erase(text);

    println!("{:?}", fox);
    println!("{:?}", dog);

    // Exercise - Lifetimes

    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}

fn process(input: String) {}

fn process_number(input: u32) {}

fn print_greeting(message: &String) {
    println!("Greeting from function: {}", message);
}

// Doesn't compile since `message: &String` can't be mutate
// fn mutate_message(message: &String) {
fn mutate_message(message: &mut String) {
    message.push_str("!");
}

/*
// Compiler error: `missing lifetime specifier`
fn longest_word(x: &String, y: &String) -> &String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

// `a` is a generic lifetime parameter
// says: the lifetime of the return value and the references parameters are always the same
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn erase(_: String) {}

// TODO: modify only this function.
fn copy_and_return<'a>(vector: &mut Vec<String>, value: &'a str) -> &'a str {
    vector.push(String::from(value));
    return value;
}
