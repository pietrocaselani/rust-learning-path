struct Point<T> {
    x: T,
    y: T,
}

struct AnotherPoint<T, U> {
    x: T,
    y: U,
}

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64
}

struct Rectangle {
    width: f64,
    height: f64
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

#[derive(Debug, PartialEq)]
struct IntPoint {
    x: i32,
    y: i32
}

// Implement Display trait for IntPoint
use std::fmt;

impl fmt::Display for IntPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let boolean = Point { x: true, y: false };
    let interger = Point { x: 1, y: 9 };
    let float = Point { x: 1.7, y: 4.3 };
    let string_slice = Point { x: "High", y: "Low" };

    // let wont_work = Point { x: 25, y: true };

    let integer_and_boolean = AnotherPoint { x: 5, y: false };
    let float_and_string = AnotherPoint { x: 1.0, y: "hey" };
    let integer_and_float = AnotherPoint { x: 5, y: 4.0 };
    let both_integer = AnotherPoint { x: 10, y: 30 };
    let both_boolean = AnotherPoint { x: true, y: true };

    // Traits

    let circle = Circle { radius: 5.0 };

    let rectangle = Rectangle {
        width: 10.0,
        height: 20.0
    };

    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());

    // Derive trait

    let p1 = IntPoint { x: 1, y: 2 };
    let p2 = IntPoint { x: 4, y: -3 };

    if p1 == p2 { // can't compare two Point values!
        println!("equal!");
    } else {
        println!("not equal!");
    }

    println!("{}", p1); // can't print using the '{}' format specifier!
    println!("{:?}", p1); //  can't print using the '{:?}' format specifier!

    // Trait bounds & generic functions

    let laura = Person {
	    name: String::from("Laura"),
	    age: 31,
	    favorite_fruit: String::from("apples"),
    };

    let fido = Dog {
	    name: String::from("Fido"),
	    color: String::from("Black"),
	    likes_petting: true,
    };

    send_data_as_json(&laura);
    send_data_as_json(&fido);

    let kitty = Cat {
        name: String::from("Kitty"),
        sharp_claws: false,
    };

    // Compiler error: Cat doesn't implement AsJson trait
    // send_data_as_json(&kitty);

    // Iterators
    // Implemeting our own

    let mut counter = Counter::new(6);

    for number in counter {
        println!("{}", number);
    }

    let sum_until_10: usize = Counter::new(10).sum();
    assert_eq!(sum_until_10, 55);

    let powers_of_2: Vec<usize> = Counter::new(8).map( |n| 2usize.pow(n as u32)).collect();
    assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256]);

    // Exercise - Implement generic type

    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(Container::new(String::from("Bar")).value, String::from("Bar"));
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));

    // Exercise - Implement an iterator

    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
    // groups:     |->|---->|->|->|--->|----------->|--->|
    assert_eq!(
	Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
	vec![
	    vec![4],
	    vec![1, 1],
	    vec![2],
	    vec![1],
	    vec![3, 3],
	    vec![-2, -2, -2],
	    vec![5, 5],
	]
    );

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    // groups:      |->|---->|---->|----|->|----->|->|
    assert_eq!(
	Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
	vec![
	    vec![1],
	    vec![2, 2],
	    vec![1, 1],
	    vec![2, 2],
	    vec![3],
	    vec![4, 4],
	    vec![3],
	]
    )
}

trait AsJson {
    fn as_json(&self) -> String;
}

// We can also write like this
// fn send_data_as_json<T: AsJson>(value: &T) { }
fn send_data_as_json(value: &impl AsJson) {
    println!("Sending JSON data...");
    println!("{}", value.as_json());
    println!("Done\n");
}


struct Person {
    name: String,
    age: u8,
    favorite_fruit: String
}

struct Dog {
    name: String,
    color: String,
    likes_petting: bool
}

struct Cat {
    name: String,
    sharp_claws: bool,
}

impl AsJson for Person {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "person", "name": "{}", "age": "{}", "favorite_fruit": "{}" }}"#,
            self.name, self.age, self.favorite_fruit
        )
    }
}

impl AsJson for Dog {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
            self.name, self.color, self.likes_petting
        )
    }
}

#[derive(Debug)]
struct Counter {
    length: usize,
    count: usize
}

impl Counter {
    fn new(length: usize) -> Counter {
        Counter {
            count: 0,
            length
        }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count <= self.length {
            Some(self.count)
        } else {
            None
        }
    }
}

struct Container<V> {
    value: V,
}

impl<V> Container<V> {
    pub fn new(value: V) -> Self {
        Container { value }
    }
}

struct Groups<T> {
    inner: Vec<T>,
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
        Groups { inner }
    }
}

impl<T: PartialEq> Iterator for Groups<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.len() == 0 {
            return None;
        }

        let mut cursor = 1;
        let first = &self.inner[0];

        for value in &self.inner[1..] {
            if value == first {
                cursor += 1;
            } else {
                break;
            }
        }

        let items = self.inner.drain(0..cursor).collect();

        Some(items)
    }
}
