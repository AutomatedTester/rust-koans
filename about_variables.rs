#[test]
fn should_show_variables_are_immutable_by_default(){
    let mut foo = "I like foo";
    foo = "I actually like baz";
}

#[test]
fn should_show_a_managed_pointer(){
    // A managed pointer allows us to work with mutable objects
    let foo = [1, 2, 3, 4];
    foo.push(5);
}