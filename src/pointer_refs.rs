pub fn run() {
	// primative array
	let arr1 = [1,2,3];
	let arr2 = arr1;

	println!("{:?}", (&arr1, &arr2));

	let arr1 = vec![1,2,3];
	let arr2 = &arr1;

	println!("{:?}", (&arr1, &arr2))
}