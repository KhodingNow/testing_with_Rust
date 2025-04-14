use adder::add_five;

mod common;

#[test]
fn it_adds_five() {
    let result = add_five(5);
    assert_eq!(result, 10);
}
      

