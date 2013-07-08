#[test]
fn should_call_method_and_handle_response(){
    // Return an integer
    let foo = sum(4, 5);
    assert!(foo == 9);
}

#[test]
fn should_call_method_and_have_an_implicit_return(){
    /*
        Implicit returns have a little magic a thanks to the way
        semicolons are handled
    */

    let foo = sum2(4, 5);
    assert!(foo == 9);
}