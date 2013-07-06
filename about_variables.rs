#[test]
fn should_show_variables_are_immutable_by_default(){
	let foo = "I like foo";
	foo = "I actually like baz";
}