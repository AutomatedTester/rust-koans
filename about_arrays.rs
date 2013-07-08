#[test]
fn test_we_can_get_item_from_array() {
    // Create an array
    assert!("5" == foo[2]);
}

#[test]
fn test_we_add_a_new_item_to_an_array() {
    let mut foo = ~[];
    // Add new items to the array/vector
    assert!(foo.len() == 4);
}

#[test]
fn test_we_can_pop_items_from_list() {
    let mut foo = ~["a", "b", "c"];
    // Pop items from the list
    assert!(foo.len() == 2);
    assert!(~["a", "b"] == foo);
}

#[test]
fn test_that_we_can_slice_arrays() {
    let foo = ["a", "b", "c"];
    // Slice the array so you get 1 array item back
    assert!(["c"] == bar);
}

#[test]
fn should_find_the_largest_number_in_an_array(){
    let mut max = 0;
    let vec = [1, 5, 6, 8, 2, 3, 4];
    // Use a map to find the largest number
    assert!(8 == max);
}
