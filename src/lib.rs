pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn another() {
//         panic!("Make this test fail");
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width : 5,
//             height: 2,
//         };

//         assert!(larger.can_hold(&smaller));
//     }
// }

#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
        width: 18,
        height: 17,
    };

    let smaller = Rectangle {
        width: 15,
        height: 10,
    };

    assert!(!smaller.can_hold(&larger));
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_adds_two() {
//         let result = add_two(2);
//         assert_eq!(result, 4)
//     }
// }

// Greet ppl with name test:

// pub fn greeting(name: &str) -> String {
//     format!("Hello!")
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Karoline");
//         assert!(
//             result.contains("Karoline"),
//         "Greeting did not contain name, value was '{result}'"
//     );

//     }
// }

// Checking for panics with 'should_panic'

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got value {value}");

        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}

// Using Result<T, E> in Tests