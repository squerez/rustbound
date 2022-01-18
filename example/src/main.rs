fn main() {
	// comma missing
	let number = 123
	// divided by zero
	let y = 0;     
	assert 1 / y != 0;
	let divbyzero = 1/0;
	println!("{}", divbyzero);
	//out of scope error
	{
		let outofscope = "world";
	}
    println!("Hello, {}!", outofscope);
}
