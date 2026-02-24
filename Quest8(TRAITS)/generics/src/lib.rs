//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

pub fn identity<T>(v: T) -> T {
	v
}

/*
	* Q & A :
	* Q1 : Why do we need generics?
	- A1 : Generics allow us to write flexible and reusable code that can work with different types without sacrificing type safety. They enable us to create functions, structs, enums, and traits that can operate on a variety of types while still ensuring that the code is type-checked at compile time. This leads to more maintainable and efficient code, as we can avoid code duplication and leverage the power of Rust's type system to catch errors early in the development process.
	What does type-check at compile time mean?
	- : Type-checking at compile time means that the Rust compiler verifies that the types of variables, function parameters, and return values are consistent and correct before the program is run. This process helps catch type-related errors early in the development cycle, preventing potential runtime errors and ensuring that the code adheres to the defined type constraints. For example, if you try to pass a value of the wrong type to a function, the compiler will generate an error, allowing you to fix the issue before the program is executed. This is one of the key features of Rust that contributes to its safety and reliability.
	* Q2: What does this identity<T> syntax mean?
	- A2: Its like saying this function is marked as generic and it can work with any type T.  
*/