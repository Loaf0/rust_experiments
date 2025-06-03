#![allow(unused)]

use std::vec;

pub fn datatypes(){
    // array
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1; 5]; // all elements are 1
    let arr3: [Foo; 5] = [Foo {}; 5];

    println!("{:?}", arr3);

    //tupe
    let tuple1: (&'static str, i32, f64) = ("Hello", 42, 3.25);
    let tuple2: Bar = Bar(2, true, Foo {}); // alternative tuple with preset fields
    println!("item 1 : {}", tuple1.1);

    //vector (arraylist)
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5]; // mutable generic vector with declared type
    vector.push(6); // add an element to the vector
    println!("vector: {:?}", vector.get(0)); // get the first element // get handles None if index is out of bounds

    // enum
    enum SuperBoolean {
        True,
        False,
        Unknown,
        Maybe,
        Possibly,
        Probably,
        ProbablyNot,
        Definitely,
        DefinitelyNot,
        Uncertain,
        Unclear,
        IDontKnow,
        Unlikely,
        Likely,
        UnlikelyButPossible,
        LikelyButUncertain,
    };

    let answer = SuperBoolean::Unclear;

    match answer {
        SuperBoolean::True => println!("The answer is true!"),
        SuperBoolean::False => println!("The answer is false!"),
        SuperBoolean::Unknown => println!("The answer is unknown!"),
        SuperBoolean::Maybe => println!("The answer is maybe!"),
        SuperBoolean::Possibly => println!("The answer is possibly!"),
        SuperBoolean::Probably => println!("The answer is probably!"),
        SuperBoolean::ProbablyNot => println!("The answer is probably not!"),
        SuperBoolean::Definitely => println!("The answer is definitely!"),
        SuperBoolean::DefinitelyNot => println!("The answer is definitely not!"),
        SuperBoolean::Uncertain => println!("The answer is uncertain!"),
        SuperBoolean::Unclear => println!("The answer is unclear!"),
        SuperBoolean::IDontKnow => println!("I don't know the answer!"),
        SuperBoolean::Unlikely => println!("The answer is unlikely!"),
        SuperBoolean::Likely => println!("The answer is likely!"),
        SuperBoolean::UnlikelyButPossible => println!("The answer is unlikely but possible!"),
        SuperBoolean::LikelyButUncertain => println!("The answer is likely but uncertain!"),
    }

    enum State { // enums can also have data associated with them
        On(u16), // power level
        Off, // no power
        Standby(u8), // timeout Length
        Error(String), // error message
        Unknown, // unknown state
    };

    let current_state = State::On(75); // current state with power level

    match current_state {
        State::On(power) => println!("The device is on with power level: {}", power),
        State::Off => println!("The device is off"),
        State::Standby(length) if length > 8 => println!("entering low power standby for : {}", length),
        State::Standby(length) => println!("The device is in standby mode"), // length is available here
        State::Error(msg) => println!("The device has an error: {}", msg),
        State::Unknown => println!("The device state is unknown"),
    }

    //generics
    
    let c1 = GenericContainer { value: 10 };
    let c2 = GenericContainer { value: 20 };
    println!("Comparison result: {:?}", c1.compare(&c2.value));

    println!("Comparing items: {:?}", compare_items(&22, &20));

    // option (nullable) type

    let foo: Nullable<i32> = Nullable::Value(42);
    let bar: Nullable<i32> = Nullable::Null;
    let foo2: Option<i32> = None; // this is the auto implementation of the Nullable type

    // method results and exceptions 
    let result = match floor_divide(10., 2.){
        Ok(value) => value,
        Err(e) => {
            println!("Error: {}", e);
            0 // default value in case of error
        }
    };
    println!("Result of division: {}", result);
}

fn floor_divide(numerator: f32, denominator: f32) -> Result<i32, String>{
    if denominator == 0.0 {
        // panic!(); // cannot recover from panic
        Err("Division by zero".to_string())
    } else {
        Ok((numerator / denominator).floor() as i32)
    }
}

enum Nullable<T>{
    Null,
    Value(T),
}

use std::cmp::Ord;
use std::cmp::Ordering;

struct GenericContainer<T : Ord>{
    value: T,
}

impl<T : Ord> GenericContainer<T> {
    fn compare(&self, other_item: &T) -> Ordering {
        self.value.cmp(other_item)
    }
}

#[derive(Clone, Copy, Debug)]
struct Foo {}

struct Bar(u64, bool, Foo);

fn compare_items<T: Ord>(item1: &T, item2: &T) -> Ordering {
    return item1.cmp(item2)
}