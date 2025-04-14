pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn exploration() {
// //         let result = add(2, 2);
// //         assert_eq!(result, 4);
// //     }

// //     #[test]
// //     fn another() {
// //         panic!("Make this test fail");
// //     }
// // }

// // #[derive(Debug)]
// // struct Rectangle {
// //     width: u32,
// //     height: u32,
// // }

// // impl Rectangle {
// //     fn can_hold(&self, other: &Rectangle) -> bool {
// //         self.width > other.width && self.height > other.height
// //     }
// // }


// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn larger_can_hold_smaller() {
// //         let larger = Rectangle {
// //             width: 8,
// //             height: 7,
// //         };
// //         let smaller = Rectangle {
// //             width : 5,
// //             height: 2,
// //         };

// //         assert!(larger.can_hold(&smaller));
// //     }
// // // }

// // #[test]
// // fn smaller_cannot_hold_larger() {
// //     let larger = Rectangle {
// //         width: 18,
// //         height: 17,
// //     };

// //     let smaller = Rectangle {
// //         width: 15,
// //         height: 10,
// //     };

// //     assert!(!smaller.can_hold(&larger));
// // }

// // pub fn add_two(a: usize) -> usize {
// //     a + 2
// // }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn it_adds_two() {
// //         let result = add_two(2);
// //         assert_eq!(result, 4)
// //     }
// // }

// // Greet ppl with name test:

// // pub fn greeting(name: &str) -> String {
// //     format!("Hello!")
// // }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn greeting_contains_name() {
// //         let result = greeting("Karoline");
// //         assert!(
// //             result.contains("Karoline"),
// //         "Greeting did not contain name, value was '{result}'"
// //     );

// //     }
// // }

// // Checking for panics with 'should_panic'

// pub struct Guess {
//     value: i32,
// }

// // impl Guess {
// //     pub fn new(value: i32) -> Guess {
// //         if value < 1 || value > 100 {
// //             panic!("Guess value must be between 1 and 100, got value {value}");

// //         }

// //         Guess { value }
// //     }
// // }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     #[should_panic]
// //     fn greater_than_100() {
// //         Guess::new(200);
// //     }
// // }


// // Making 'should_panic' TESTS more precise by adding an optional 'expected' parameter to the should_panic attribute.
// // THe test harness will ensure the failure message contains the provided text.

//     // impl Guess {
//     //     pub fn new(value: i32) -> Guess {
//     //         if value < 1 {
//     //             panic! (
//     //                 "Guess value must be greater than or equal to 1, got {value}"
//     //             );

//     //         }else if value > 100 {
//     //             panic!(
//     //                 "Guess value must be less or equal to 100, got {value}"
//     //             );
//     //         }

//     //         Guess {value}
//     //     }
//     // }

//     // #[cfg(test)]
//     // mod tests {
//     //     use super::*;

//     //     #[test]
//     //     #[should_panic(expected = "less than or equal to 100")]
//     //     fn greater_than_100() {
//     //         Guess::new(200);
//     //     }

//     // }



// // Using Result<T, E> in Tests - so far, our tests panic when they fail. We can write tests that use 'Result<T, E> as well.

// //EXAMPLE:
// // this test returns an 'Err' instead of panicking:

//     // #[test]
//     // fn it_works() -> Result<(), String> {
//     //     let result = add(2, 2);

//     //     if result == 4 {
//     //         Ok(())
//     //     } else {
//     //         Err(String::from("two plus two does not equal four"))
//     //     }
//     // }
// // Running the tests in parallel OR Consecutively

// // Show function OUTPUT:

    fn prints_and_returns_10(a: i32) -> i32 {
        println!("I got the value {a}");
        10
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn this_test_will_pass() {
            let value = prints_and_returns_10(4);
            assert_eq!(value, 10);
        }
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(value, 5);
    // }


// // Running a Subset of Tests by Name: - this allow a dev to test code in a particular area - you can then run 'cargo test' the name / names of the test/s you want to run as an argument.

    pub fn add_three(a: usize) -> usize {
        a + 3
     }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn add_three_and_three() {
            let result = add_three(3);
            assert_eq!(result, 6);
        }

        #[test]
        fn add_three_and_four() {
            let result = add_three(4);
            assert_eq!(result, 7);
        }

        #[test]
        fn one_hundred() {
            let result = add_three(100);
            assert_eq!(result, 103);
        }

    // Ignoring some Test Unless Speciafically Requestd

        #[test]
        fn does_it_work() {
            let result = add(4, 5);
            assert_eq!(result, 9);
        }

        #[test]
        #[ignore]
        fn expensive_test() {

        }    
    }


    // Testing Private functions (Rust allows it)
    pub fn add_five(a: usize) -> usize {
        internal_adder(a, 5)
    }
    fn internal_adder(left: usize, right:usize) -> usize {
        left + right
    }

    // #[cfg(test)]
    // mod tests {
    //     use super::*;

    //     #[test]
    //     fn internal() {
    //         let result = internal_adder(5, 5);
    //         assert_eq!(result, 10);
    //     }
    // }




